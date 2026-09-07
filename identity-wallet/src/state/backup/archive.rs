//! The UniMe backup archive format.
//!
//! A backup is a single self-describing file: a plaintext header followed by one
//! AES-256-GCM ciphertext. The header carries everything needed to derive the key
//! and decrypt, so a backup can be opened by any build that understands its format
//! version — no out-of-band state required.
//!
//! ```text
//! offset  size  field
//! 0       8     magic, b"UNIMEBAK"
//! 8       2     format version (u16 LE)
//! 10      1     KDF id (1 = Argon2id)
//! 11      4     Argon2 m_cost, in KiB (u32 LE)
//! 15      4     Argon2 t_cost (u32 LE)
//! 19      4     Argon2 p_cost (u32 LE)
//! 23      16    salt
//! 39      12    nonce
//! 51      2     app version length (u16 LE)
//! 53      n     app version, UTF-8
//! 53+n    ..    ciphertext, with the 16-byte GCM tag appended
//! ```
//!
//! The whole header is passed to AES-GCM as additional authenticated data, so a
//! tampered version string, salt, or KDF parameter fails authentication rather
//! than silently changing how the body is decrypted.
//!
//! The plaintext body is a section list, which is what makes the archive
//! recoverable — the previous prototype concatenated the state and Stronghold
//! files with no length prefix and could not be taken apart again:
//!
//! ```text
//! 2   section count (u16 LE)
//! per section:
//!   1   kind (1 = state, 2 = stronghold, 3 = assets)
//!   8   length (u64 LE)
//!   n   bytes
//! ```
//!
//! The assets section is itself a list, so downloaded issuer logos and credential
//! images survive a restore:
//!
//! ```text
//! 4   asset count (u32 LE)
//! per asset:
//!   2   name length (u16 LE)
//!   n   name, UTF-8
//!   8   data length (u64 LE)
//!   n   data
//! ```

use aes_gcm::{
    aead::{Aead, OsRng, Payload as AeadPayload},
    AeadCore, Aes256Gcm, KeyInit, Nonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use zeroize::Zeroizing;

/// Identifies a UniMe backup file.
pub const MAGIC: [u8; 8] = *b"UNIMEBAK";

/// The archive layout this build writes. Bump on any breaking layout change.
pub const FORMAT_VERSION: u16 = 1;

const KDF_ARGON2ID: u8 = 1;
const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 12;
const KEY_LEN: usize = 32;

/// Argon2id cost parameters, recorded in the header so that raising them later
/// does not orphan existing backups.
const ARGON2_M_COST: u32 = 19 * 1024; // 19 MiB, the OWASP baseline
const ARGON2_T_COST: u32 = 2;
const ARGON2_P_COST: u32 = 1;

/// Refuse absurd headers before allocating anything based on them.
const MAX_APP_VERSION_LEN: usize = 64;

const SECTION_STATE: u8 = 1;
const SECTION_STRONGHOLD: u8 = 2;
const SECTION_ASSETS: u8 = 3;

#[derive(Debug, thiserror::Error)]
pub enum ArchiveError {
    #[error("not a UniMe backup file")]
    NotABackup,
    #[error("unsupported backup format version {found} (this build writes {supported})")]
    UnsupportedVersion { found: u16, supported: u16 },
    #[error("unsupported key derivation function id {0}")]
    UnsupportedKdf(u8),
    #[error("backup file is truncated or malformed")]
    Malformed,
    #[error("wrong password, or the backup file has been modified")]
    Authentication,
    #[error("unknown section kind {0}")]
    UnknownSection(u8),
    #[error("backup is missing its {0} section")]
    MissingSection(&'static str),
    #[error("failed to derive a key from the password: {0}")]
    KeyDerivation(String),
    #[error("asset name is not valid UTF-8")]
    InvalidAssetName,
    #[error("app version is not valid UTF-8")]
    InvalidAppVersion,
}

/// One file from the assets directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Asset {
    pub name: String,
    pub data: Vec<u8>,
}

/// Everything a backup carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Payload {
    /// The app version that wrote this backup, so a restore can replay the
    /// migration chain rather than deserializing the state blindly.
    pub app_version: String,
    /// Contents of `state.json`.
    pub state: Vec<u8>,
    /// Contents of `stronghold.bin`.
    pub stronghold: Vec<u8>,
    /// Contents of the assets directory, by file name.
    pub assets: Vec<Asset>,
}

/// Encrypts `payload` under `password`, returning the complete archive bytes.
///
/// A fresh salt and nonce are drawn for every call, so sealing the same payload
/// twice produces different files — nonce reuse under one key is what breaks
/// AES-GCM, and a stored constant would guarantee it.
pub fn seal(password: &str, payload: &Payload) -> Result<Vec<u8>, ArchiveError> {
    let mut salt = [0u8; SALT_LEN];
    getrandom_bytes(&mut salt);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let header = encode_header(&salt, nonce.as_slice(), &payload.app_version);
    let key = derive_key(password, &salt, ARGON2_M_COST, ARGON2_T_COST, ARGON2_P_COST)?;

    let cipher = Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| ArchiveError::Malformed)?;
    let body = encode_body(payload);
    let ciphertext = cipher
        .encrypt(
            &nonce,
            AeadPayload {
                msg: &body,
                aad: &header,
            },
        )
        .map_err(|_| ArchiveError::Authentication)?;

    let mut out = header;
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypts and parses an archive produced by [`seal`].
///
/// Returns [`ArchiveError::Authentication`] both for a wrong password and for a
/// modified file; the two are indistinguishable by design, and neither reveals
/// which it was.
pub fn open(password: &str, bytes: &[u8]) -> Result<Payload, ArchiveError> {
    let mut reader = Reader::new(bytes);

    read_magic(&mut reader)?;

    let format_version = reader.u16()?;
    if format_version != FORMAT_VERSION {
        return Err(ArchiveError::UnsupportedVersion {
            found: format_version,
            supported: FORMAT_VERSION,
        });
    }

    let kdf_id = reader.u8()?;
    if kdf_id != KDF_ARGON2ID {
        return Err(ArchiveError::UnsupportedKdf(kdf_id));
    }

    let m_cost = reader.u32()?;
    let t_cost = reader.u32()?;
    let p_cost = reader.u32()?;
    let salt = reader.take(SALT_LEN)?.to_vec();
    let nonce = reader.take(NONCE_LEN)?.to_vec();

    let app_version_len = reader.u16()? as usize;
    if app_version_len > MAX_APP_VERSION_LEN {
        return Err(ArchiveError::Malformed);
    }
    let app_version =
        String::from_utf8(reader.take(app_version_len)?.to_vec()).map_err(|_| ArchiveError::InvalidAppVersion)?;

    // Everything consumed so far is the header, and all of it is authenticated.
    let header = &bytes[..reader.position()];
    let ciphertext = reader.rest();

    let key = derive_key(password, &salt, m_cost, t_cost, p_cost)?;
    let cipher = Aes256Gcm::new_from_slice(key.as_slice()).map_err(|_| ArchiveError::Malformed)?;
    let body = cipher
        .decrypt(
            Nonce::from_slice(&nonce),
            AeadPayload {
                msg: ciphertext,
                aad: header,
            },
        )
        .map_err(|_| ArchiveError::Authentication)?;

    decode_body(&body, app_version)
}

/// Reads the app version from an archive without decrypting it, so a recovery
/// screen can show what it is about to restore before asking for a password.
pub fn peek_app_version(bytes: &[u8]) -> Result<String, ArchiveError> {
    let mut reader = Reader::new(bytes);
    read_magic(&mut reader)?;
    let format_version = reader.u16()?;
    if format_version != FORMAT_VERSION {
        return Err(ArchiveError::UnsupportedVersion {
            found: format_version,
            supported: FORMAT_VERSION,
        });
    }
    // Skip the KDF id, its three cost parameters, the salt, and the nonce.
    reader.take(1 + 4 + 4 + 4 + SALT_LEN + NONCE_LEN)?;
    let len = reader.u16()? as usize;
    if len > MAX_APP_VERSION_LEN {
        return Err(ArchiveError::Malformed);
    }
    String::from_utf8(reader.take(len)?.to_vec()).map_err(|_| ArchiveError::InvalidAppVersion)
}

/// A file shorter than the magic is simply not one of ours, so it reports
/// [`ArchiveError::NotABackup`] rather than looking like a corrupt backup.
fn read_magic(reader: &mut Reader<'_>) -> Result<(), ArchiveError> {
    match reader.take(MAGIC.len()) {
        Ok(magic) if magic == MAGIC => Ok(()),
        _ => Err(ArchiveError::NotABackup),
    }
}

fn derive_key(
    password: &str,
    salt: &[u8],
    m_cost: u32,
    t_cost: u32,
    p_cost: u32,
) -> Result<Zeroizing<[u8; KEY_LEN]>, ArchiveError> {
    let params =
        Params::new(m_cost, t_cost, p_cost, Some(KEY_LEN)).map_err(|e| ArchiveError::KeyDerivation(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = Zeroizing::new([0u8; KEY_LEN]);
    argon2
        .hash_password_into(password.as_bytes(), salt, key.as_mut())
        .map_err(|e| ArchiveError::KeyDerivation(e.to_string()))?;
    Ok(key)
}

fn encode_header(salt: &[u8; SALT_LEN], nonce: &[u8], app_version: &str) -> Vec<u8> {
    let version_bytes = app_version.as_bytes();
    let mut header = Vec::with_capacity(53 + version_bytes.len());
    header.extend_from_slice(&MAGIC);
    header.extend_from_slice(&FORMAT_VERSION.to_le_bytes());
    header.push(KDF_ARGON2ID);
    header.extend_from_slice(&ARGON2_M_COST.to_le_bytes());
    header.extend_from_slice(&ARGON2_T_COST.to_le_bytes());
    header.extend_from_slice(&ARGON2_P_COST.to_le_bytes());
    header.extend_from_slice(salt);
    header.extend_from_slice(nonce);
    header.extend_from_slice(&(version_bytes.len() as u16).to_le_bytes());
    header.extend_from_slice(version_bytes);
    header
}

fn encode_body(payload: &Payload) -> Vec<u8> {
    let mut assets = Vec::new();
    assets.extend_from_slice(&(payload.assets.len() as u32).to_le_bytes());
    for asset in &payload.assets {
        let name = asset.name.as_bytes();
        assets.extend_from_slice(&(name.len() as u16).to_le_bytes());
        assets.extend_from_slice(name);
        assets.extend_from_slice(&(asset.data.len() as u64).to_le_bytes());
        assets.extend_from_slice(&asset.data);
    }

    let mut body = Vec::new();
    body.extend_from_slice(&3u16.to_le_bytes());
    push_section(&mut body, SECTION_STATE, &payload.state);
    push_section(&mut body, SECTION_STRONGHOLD, &payload.stronghold);
    push_section(&mut body, SECTION_ASSETS, &assets);
    body
}

fn push_section(out: &mut Vec<u8>, kind: u8, bytes: &[u8]) {
    out.push(kind);
    out.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    out.extend_from_slice(bytes);
}

fn decode_body(body: &[u8], app_version: String) -> Result<Payload, ArchiveError> {
    let mut reader = Reader::new(body);
    let section_count = reader.u16()?;

    let mut state = None;
    let mut stronghold = None;
    let mut assets = None;

    for _ in 0..section_count {
        let kind = reader.u8()?;
        let len = reader.u64()? as usize;
        let bytes = reader.take(len)?;
        match kind {
            SECTION_STATE => state = Some(bytes.to_vec()),
            SECTION_STRONGHOLD => stronghold = Some(bytes.to_vec()),
            SECTION_ASSETS => assets = Some(decode_assets(bytes)?),
            other => return Err(ArchiveError::UnknownSection(other)),
        }
    }

    Ok(Payload {
        app_version,
        state: state.ok_or(ArchiveError::MissingSection("state"))?,
        stronghold: stronghold.ok_or(ArchiveError::MissingSection("stronghold"))?,
        assets: assets.ok_or(ArchiveError::MissingSection("assets"))?,
    })
}

fn decode_assets(bytes: &[u8]) -> Result<Vec<Asset>, ArchiveError> {
    let mut reader = Reader::new(bytes);
    let count = reader.u32()?;
    let mut assets = Vec::new();
    for _ in 0..count {
        let name_len = reader.u16()? as usize;
        let name = String::from_utf8(reader.take(name_len)?.to_vec()).map_err(|_| ArchiveError::InvalidAssetName)?;
        let data_len = reader.u64()? as usize;
        let data = reader.take(data_len)?.to_vec();
        assets.push(Asset { name, data });
    }
    Ok(assets)
}

/// A bounds-checked cursor. A backup file is attacker-influenced input, so every
/// read is fallible rather than an indexing panic.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }

    fn position(&self) -> usize {
        self.at
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8], ArchiveError> {
        let end = self.at.checked_add(len).ok_or(ArchiveError::Malformed)?;
        let slice = self.bytes.get(self.at..end).ok_or(ArchiveError::Malformed)?;
        self.at = end;
        Ok(slice)
    }

    fn rest(&self) -> &'a [u8] {
        &self.bytes[self.at..]
    }

    fn u8(&mut self) -> Result<u8, ArchiveError> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16, ArchiveError> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    fn u32(&mut self) -> Result<u32, ArchiveError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn u64(&mut self) -> Result<u64, ArchiveError> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }
}

fn getrandom_bytes(buf: &mut [u8]) {
    use aes_gcm::aead::rand_core::RngCore;
    OsRng.fill_bytes(buf);
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Argon2id at the production cost is deliberately slow. The tests exercise
    /// the format, not the KDF tuning, so they run on a small payload set.
    fn payload() -> Payload {
        Payload {
            app_version: "0.15.1".to_string(),
            state: br#"{"credentials":[],"connections":[]}"#.to_vec(),
            stronghold: vec![0xde, 0xad, 0xbe, 0xef, 0x00, 0xff],
            assets: vec![
                Asset {
                    name: "university.png".to_string(),
                    data: vec![0x89, 0x50, 0x4e, 0x47],
                },
                Asset {
                    name: "iota-icon-dark.svg".to_string(),
                    data: b"<svg/>".to_vec(),
                },
            ],
        }
    }

    #[test]
    fn round_trip_preserves_every_section() {
        let original = payload();
        let sealed = seal("sup3rSecr3t", &original).unwrap();
        let opened = open("sup3rSecr3t", &sealed).unwrap();
        assert_eq!(original, opened);
    }

    #[test]
    fn round_trip_survives_empty_sections() {
        let original = Payload {
            app_version: "0.15.1".to_string(),
            state: Vec::new(),
            stronghold: Vec::new(),
            assets: Vec::new(),
        };
        let sealed = seal("pw", &original).unwrap();
        assert_eq!(original, open("pw", &sealed).unwrap());
    }

    #[test]
    fn round_trip_survives_duplicate_asset_names_and_binary_data() {
        let original = Payload {
            app_version: "0.15.1".to_string(),
            state: b"{}".to_vec(),
            stronghold: (0u8..=255).collect(),
            assets: vec![
                Asset {
                    name: "logo.png".to_string(),
                    data: (0u8..=255).rev().collect(),
                },
                Asset {
                    name: "logo.png".to_string(),
                    data: vec![],
                },
            ],
        };
        let sealed = seal("pw", &original).unwrap();
        assert_eq!(original, open("pw", &sealed).unwrap());
    }

    #[test]
    fn sealing_twice_produces_different_files() {
        let payload = payload();
        let first = seal("pw", &payload).unwrap();
        let second = seal("pw", &payload).unwrap();

        // A fresh salt and nonce each time; identical output would mean the
        // nonce is fixed, which is what made the prototype unsafe.
        assert_ne!(first, second);
        assert_eq!(open("pw", &first).unwrap(), open("pw", &second).unwrap());
    }

    #[test]
    fn wrong_password_is_rejected() {
        let sealed = seal("correct horse", &payload()).unwrap();
        assert!(matches!(
            open("battery staple", &sealed),
            Err(ArchiveError::Authentication)
        ));
    }

    #[test]
    fn tampering_with_the_ciphertext_is_detected() {
        let mut sealed = seal("pw", &payload()).unwrap();
        let last = sealed.len() - 20;
        sealed[last] ^= 0x01;
        assert!(matches!(open("pw", &sealed), Err(ArchiveError::Authentication)));
    }

    #[test]
    fn tampering_with_the_header_is_detected() {
        let mut sealed = seal("pw", &payload()).unwrap();
        // Flip a bit inside the salt, which lives in the authenticated header.
        sealed[23] ^= 0x01;
        assert!(matches!(open("pw", &sealed), Err(ArchiveError::Authentication)));
    }

    #[test]
    fn a_foreign_file_is_not_mistaken_for_a_backup() {
        assert!(matches!(open("pw", b"just some bytes"), Err(ArchiveError::NotABackup)));
    }

    #[test]
    fn a_future_format_version_is_refused_by_name() {
        let mut sealed = seal("pw", &payload()).unwrap();
        sealed[8..10].copy_from_slice(&99u16.to_le_bytes());
        assert!(matches!(
            open("pw", &sealed),
            Err(ArchiveError::UnsupportedVersion { found: 99, .. })
        ));
    }

    #[test]
    fn a_truncated_header_does_not_panic() {
        let sealed = seal("pw", &payload()).unwrap();
        // Every prefix that cannot carry a complete header must produce an error
        // rather than an index panic. Prefixes past the header are excluded only
        // because each one would run the full-cost KDF before failing.
        let header_len = 53 + "0.15.1".len();
        for len in 0..header_len {
            assert!(
                open("pw", &sealed[..len]).is_err(),
                "prefix of {len} bytes should error"
            );
            assert!(
                peek_app_version(&sealed[..len]).is_err(),
                "peek at {len} bytes should error"
            );
        }

        // A complete header with no body: `open` has nothing to authenticate,
        // but peeking is defined to need only the header and must succeed.
        assert!(open("pw", &sealed[..header_len]).is_err());
        assert_eq!(peek_app_version(&sealed[..header_len]).unwrap(), "0.15.1");
    }

    #[test]
    fn a_truncated_ciphertext_is_rejected() {
        let sealed = seal("pw", &payload()).unwrap();
        assert!(matches!(
            open("pw", &sealed[..sealed.len() - 1]),
            Err(ArchiveError::Authentication)
        ));
    }

    #[test]
    fn app_version_is_readable_without_the_password() {
        let sealed = seal("pw", &payload()).unwrap();
        assert_eq!(peek_app_version(&sealed).unwrap(), "0.15.1");
    }

    #[test]
    fn peeking_rejects_a_foreign_file() {
        assert!(matches!(peek_app_version(b"nope"), Err(ArchiveError::NotABackup)));
    }
}
