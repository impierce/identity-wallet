import type { CredentialStatus } from '@bindings/credentials/CredentialStatus';
import type { EventType } from '@bindings/history/EventType';
import type { HistoryCredential } from '@bindings/history/HistoryCredential';
import type { HistoryEvent } from '@bindings/history/HistoryEvent';
import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

import type { AcceptConnectionPrompt, Certification } from '$lib/dev/accept-connection.types';

const base: AcceptConnectionPrompt = {
  type: 'accept-connection',
  client_name: 'BestDex',
  logo_uri: 'https://bestdex.com/logo.png',
  redirect_uri: 'https://www.bestdex.com/callback',
  // `connection_data` omitted: absent means we have never interacted with this party.
  // `domain_validation` carries no `url` — the header renders its domain from `redirect_uri`.
  domain_validation: { status: 'Success' },
  linked_verifiable_presentations: [],
  ecosystems: [],
};

/** Readable, stable ids: they end up in the detail route's URL. */
const slug = (name: string) =>
  name
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');

const defaultClaims = (name: string, issuer?: string) => ({
  id: 'did:web:bestdex.com',
  certificationName: name,
  ...(issuer ? { certifyingBody: issuer } : {}),
  validFrom: '2025-03-12T00:00:00Z',
  validUntil: '2028-03-11T00:00:00Z',
});

const certification = (
  name: string,
  issuer?: string,
  domain?: string,
  status: ValidationStatus = 'Success',
  // `unknown` rather than a claims type: `data` is `any` on the wire, and some fixtures
  // deliberately pass a malformed subject.
  credentialSubject: unknown = undefined,
  credential_status: CredentialStatus | undefined = undefined,
): Certification => ({
  credential: {
    id: slug(name),
    format: { format: 'jwt_vc_json' },
    issuer_name: issuer ?? '',
    ...(credential_status ? { credential_status } : {}),
    data: {
      type: ['VerifiableCredential'],
      issuer: 'did:web:iso.org',
      credentialSubject: credentialSubject === undefined ? defaultClaims(name, issuer) : credentialSubject,
    },
    // Empty for `jwt_vc_json`: display claims come from issuer metadata in a credential
    // offer, which a linked verifiable presentation never has. `DefaultRenderer` falls
    // back to iterating `credentialSubject`, which is the path this whole page relies on.
    display_claims: [],
    metadata: { is_favorite: false, date_added: '', date_issued: '2025-03-12T00:00:00Z' },
    display_name: name,
  },
  issuer_domain_validations: domain ? [{ status, url: `https://${domain}/`, ...(issuer ? { name: issuer } : {}) }] : [],
});

const historyCredential = (title: string): HistoryCredential => ({
  title,
  issuer_name: 'BestDex',
  id: slug(title),
});

const interaction = (event_type: EventType, date: string, credentials: HistoryCredential[] = []): HistoryEvent => ({
  connection_id: 'did:web:bestdex.com',
  connection_name: 'BestDex',
  event_type,
  date,
  credentials,
});

/**
 * A connection we established, then received one credential from and shared data with twice.
 * Four interactions, of which `ConnectionAdded` counts towards neither direction tile.
 */
const interactions: HistoryEvent[] = [
  interaction('ConnectionAdded', '2023-04-28T10:12:00Z'),
  interaction('CredentialsAdded', '2023-05-02T14:05:00Z', [historyCredential('Loyalty Card')]),
  interaction('CredentialsShared', '2023-06-14T11:48:00Z', [historyCredential('National ID')]),
  // One exchange carrying several credentials: still a single interaction.
  interaction('CredentialsShared', '2023-07-28T09:30:00Z', [
    historyCredential('National ID'),
    historyCredential('Proof of Address'),
  ]),
];

const connected = {
  first_interacted_at: '2023-04-28T10:12:00Z',
  last_interacted_at: '2023-07-28T09:30:00Z',
  interactions,
};

const certifications: Certification[] = [
  certification('ISO 27001 Certified', 'Intl. Organization for Standardization', 'iso.org', 'Failure'),
  certification('SOC 2 Type II', 'AICPA', 'aicpa.com'),
  certification('eIDAS Qualified Trust Service Provider', 'European Commission', 'ec.europa.eu'),
  certification('PCI DSS Level 1', 'PCI Security Standards Council', 'pcisecuritystandards.org', 'Unknown'),
  certification('ISO 9001 Quality Management', 'Intl. Organization for Standardization', 'iso.org'),
  certification('GDPR Compliance Attestation', 'European Data Protection Board', 'edpb.europa.eu'),
  certification('NEN 7510 Information Security', 'Koninklijk Nederlands Normalisatie-instituut', 'nen.nl'),
  certification('CSA STAR Level 2', 'Cloud Security Alliance', 'cloudsecurityalliance.org'),
  certification('WebTrust for CAs', 'Chartered Professional Accountants of Canada', 'cpacanada.ca'),
  certification('ETSI EN 319 401', 'European Telecommunications Standards Institute', 'etsi.org'),
];

export const mocks = {
  // M1
  new: base,
  known: { ...base, connection_data: connected },
  // Connected, but no data has moved either way: both direction tiles read zero.
  'known-no-data': { ...base, connection_data: { ...connected, interactions: interactions.slice(0, 1) } },
  untrusted: {
    ...base,
    domain_validation: {
      status: 'Failure',
      message: 'No did-configuration.json found',
    },
  },
  'unknown-domain': { ...base, domain_validation: { status: 'Unknown' } },
  'long-name': { ...base, client_name: 'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek' },
  'no-logo': { ...base, logo_uri: undefined },
  // No `redirect_uri`: the domain line disappears and the validation pill stands alone.
  'no-redirect': { ...base, redirect_uri: undefined },

  // M2 — certifications
  'certs-one': { ...base, linked_verifiable_presentations: certifications.slice(0, 1) },
  // Exactly PREVIEW_COUNT: the section fills up but shows no "Show more" link.
  'certs-preview': { ...base, linked_verifiable_presentations: certifications.slice(0, 3) },
  // Over PREVIEW_COUNT: the "Show more" link appears and the sub-route lists all ten.
  'certs-many': { ...base, linked_verifiable_presentations: certifications },
  // Revoked certification: the detail page's status tile turns red.
  'certs-revoked': {
    ...base,
    linked_verifiable_presentations: [
      certification('ISO 27001 Certified', 'Intl. Organization for Standardization', 'iso.org', 'Success', undefined, {
        status: 'INVALID',
        last_checked: '2026-08-24T09:30:00Z',
      }),
    ],
  },
  // Known connection with certifications: the section starts collapsed behind a count,
  // and "Show More" expands it into the section the other `certs-*` fixtures show.
  'known-certs': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications.slice(0, 3),
  },
  // Collapsed label in the singular.
  'known-certs-one': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications.slice(0, 1),
  },
  // Issuer name and domain both missing: the card must degrade to just the title.
  'certs-bare': {
    ...base,
    linked_verifiable_presentations: [certification('Unattributed Certification')],
  },
  'known-with-certs': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications,
  },

  // M2 — certification detail pages
  // Claims covering every `ClaimRenderer` branch: a country code, two timestamps, and
  // plain text. `id` and `type` are in `DefaultRenderer`'s hide list and must not show up.
  'cert-claims-rich': {
    ...base,
    linked_verifiable_presentations: [
      certification('ISO 27001 Certified', 'Intl. Organization for Standardization', 'iso.org', 'Success', {
        id: 'did:web:bestdex.com',
        type: ['VerifiableCredential', 'CertificationCredential'],
        legalName: 'BestDex B.V.',
        certificationScope: 'Information Security Management System',
        registrationNumber: 'NL-ISO-27001-88213',
        country: 'NL',
        validFrom: '2025-03-12T00:00:00Z',
        validUntil: '2028-03-11T00:00:00Z',
      }),
    ],
  },
  // A single claim beyond the hidden `id`: the detail page must not look broken.
  'cert-claims-sparse': {
    ...base,
    linked_verifiable_presentations: [
      certification('Minimal Certification', 'Some Authority', 'authority.example', 'Success', {
        id: 'did:web:bestdex.com',
        legalName: 'BestDex B.V.',
      }),
    ],
  },
  // No `credentialSubject` at all. `DefaultRenderer` dereferences it unguarded, so the
  // detail page has to stop before reaching it rather than white-screen the prompt.
  'cert-claims-missing': {
    ...base,
    linked_verifiable_presentations: [
      certification('Malformed Certification', 'Some Authority', 'authority.example', 'Success', null),
    ],
  },
  // The logo URL lives in the subject's `image` claim. This still renders the badge in DEV:
  // `<Image>` looks for `assets/tmp/<hash(url)>`, which only exists once the backend has
  // downloaded the file. Kept so the shape is represented and `certificationLogoId` is exercised.
  'cert-logo': {
    ...base,
    linked_verifiable_presentations: [
      certification('ISO 27001 Certified', 'Intl. Organization for Standardization', 'iso.org', 'Success', {
        ...defaultClaims('ISO 27001 Certified', 'Intl. Organization for Standardization'),
        image: 'https://iso.org/badge.png',
      }),
    ],
  },
} satisfies Record<string, AcceptConnectionPrompt>;

export type MockName = keyof typeof mocks;
