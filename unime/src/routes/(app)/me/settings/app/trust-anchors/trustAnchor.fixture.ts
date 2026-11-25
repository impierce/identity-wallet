import type { TrustAnchor } from '@bindings/trust_anchors/TrustAnchor';

// 2026-11-25 and 2031-09-07, in seconds.
export const NOW = Date.UTC(2026, 10, 25) / 1000;
export const NOT_AFTER = Date.UTC(2031, 8, 7) / 1000;

/** A curated trust anchor for tests, valid on `NOW`. */
export function trustAnchor(overrides: Partial<TrustAnchor> = {}): TrustAnchor {
  return {
    id: 'thumbprint',
    name: 'Authbound EUDI Testbed Issuer Issuing CA',
    source: { type: 'curated', enabled_by_default: false },
    enabled: false,
    certificate: '-----BEGIN CERTIFICATE-----\n-----END CERTIFICATE-----\n',
    details: {
      subject: 'CN=Authbound EUDI Testbed Issuer Issuing CA 20260908 v3,O=Authbound,C=FI',
      issuer: 'CN=Authbound EUDI Testbed Root CA 20260908 v3,O=Authbound,C=FI',
      self_issued: false,
      ca: true,
      not_before: NOW - 86400,
      not_after: NOT_AFTER,
      sha256_thumbprint: 'thumbprint',
      sha256_fingerprint: 'AB:CD',
      permitted_dns_names: [],
      excluded_dns_names: [],
    },
    ...overrides,
  };
}
