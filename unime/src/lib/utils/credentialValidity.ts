import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

export type CredentialValidity =
  | { state: 'valid'; validUntil?: string }
  | { state: 'expired'; validUntil: string }
  | { state: 'revoked' }
  | { state: 'suspended' }
  | { state: 'not_yet_valid'; validFrom: string };

function parseDate(isoDate: string | undefined): Date | undefined {
  if (!isoDate) {
    return undefined;
  }
  const date = new Date(isoDate);
  return Number.isNaN(date.getTime()) ? undefined : date;
}

/**
 * `date_issued` holds `issuanceDate`/`validFrom` (or `nbf`/`iat`), which is when the credential starts being valid.
 */
export function getCredentialValidity(
  credential: Pick<DisplayCredential, 'credential_status' | 'metadata'>,
  now = new Date(),
): CredentialValidity {
  const status = credential.credential_status?.status;
  if (status === 'INVALID') {
    return { state: 'revoked' };
  }
  if (status === 'SUSPENDED') {
    return { state: 'suspended' };
  }

  const { date_issued, expiration_date } = credential.metadata;

  const validUntil = parseDate(expiration_date);
  if (expiration_date && validUntil && validUntil <= now) {
    return { state: 'expired', validUntil: expiration_date };
  }

  const validFrom = parseDate(date_issued);
  if (validFrom && validFrom > now) {
    return { state: 'not_yet_valid', validFrom: date_issued };
  }

  return validUntil ? { state: 'valid', validUntil: expiration_date } : { state: 'valid' };
}
