import type { CredentialStatus } from '@bindings/credentials/CredentialStatus';
import type { EventType } from '@bindings/history/EventType';
import type { HistoryCredential } from '@bindings/history/HistoryCredential';
import type { HistoryEvent } from '@bindings/history/HistoryEvent';
import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';
import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';
import type { Member } from '@bindings/user_prompt/Member';
import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

import type { AcceptConnectionPrompt } from './resolve';

const base: AcceptConnectionPrompt = {
  type: 'accept-connection',
  client_metadata: {
    client_name: 'BestDex',
    logo_uri: 'https://bestdex.com/logo.png',
    connection_url: 'https://www.bestdex.com',
    redirect_uri: 'https://www.bestdex.com/callback',
    // Always a DID: the backend rejects a client_id it cannot parse as one.
    client_id: 'did:web:bestdex.com',
  },
  domain_validation: { status: 'Success', url: 'https://www.bestdex.com/' },
  linked_verifiable_presentations: [],
  ecosystems: [],
};

// Stable ids: they end up in the detail route's URL.
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

interface CertificationOptions {
  name: string;
  issuer?: string;
  domain?: string;
  /** Validation status of `domain`. */
  status?: ValidationStatus;
  /** `unknown`, because `data` is `any` on the wire and one fixture passes `null`. */
  claims?: unknown;
  credential_status?: CredentialStatus;
}

const certification = ({
  name,
  issuer,
  domain,
  status = 'Success',
  claims = defaultClaims(name, issuer),
  credential_status,
}: CertificationOptions): LinkedVerifiableCredentialData => ({
  credential: {
    id: slug(name),
    format: { format: 'jwt_vc_json' },
    issuer_name: issuer ?? '',
    issuer_logo_uri: null,
    credential_status,
    data: {
      type: ['VerifiableCredential'],
      issuer: 'did:web:iso.org',
      credentialSubject: claims,
    },
    // Empty for `jwt_vc_json`: display claims come from issuer metadata in a credential offer,
    // which a linked verifiable presentation never has. `DefaultRenderer` then falls back to
    // iterating `credentialSubject`, the path this whole page relies on.
    display_claims: [],
    metadata: { is_favorite: false, date_added: '', date_issued: '2025-03-12T00:00:00Z' },
    display_name: name,
  },
  issuer_domain_validations: domain ? [{ status, url: `https://${domain}/`, name: issuer }] : [],
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

// Four on the interactions tile: `countInteractions` counts events, so `ConnectionAdded` is
// included and the two credentials in the last exchange still count once.
const interactions: HistoryEvent[] = [
  interaction('ConnectionAdded', '2023-04-28T10:12:00Z'),
  interaction('CredentialsAdded', '2023-05-02T14:05:00Z', [historyCredential('Loyalty Card')]),
  interaction('CredentialsShared', '2023-06-14T11:48:00Z', [historyCredential('National ID')]),
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

/**
 * Five, deliberately over `PREVIEW_COUNT`: the card-level variations sit in the first three,
 * the detail-page ones behind the "show more" link.
 */
const certifications: LinkedVerifiableCredentialData[] = [
  // Covers every `ClaimRenderer` branch: a country code, two timestamps, plain text. `id` and
  // `type` are in `DefaultRenderer`'s hide list and must not appear.
  certification({
    name: 'ISO 27001 Certified',
    issuer: 'Intl. Organization for Standardization',
    domain: 'iso.org',
    claims: {
      id: 'did:web:bestdex.com',
      type: ['VerifiableCredential', 'CertificationCredential'],
      legalName: 'BestDex B.V.',
      certificationScope: 'Information Security Management System',
      registrationNumber: 'NL-ISO-27001-88213',
      country: 'NL',
      validFrom: '2025-03-12T00:00:00Z',
      validUntil: '2028-03-11T00:00:00Z',
    },
  }),
  certification({
    name: 'eIDAS Qualified Trust Service Provider',
    issuer: 'European Commission',
    domain: 'ec.europa.eu',
    status: 'Failure',
  }),
  certification({ name: 'Unattributed Certification' }),
  certification({
    name: 'SOC 2 Type II',
    issuer: 'AICPA',
    domain: 'aicpa.com',
    credential_status: { status: 'INVALID', last_checked: '2026-08-24T09:30:00Z' },
  }),
  // `DefaultRenderer` dereferences `credentialSubject` unguarded, so the detail page has to
  // stop before reaching it rather than white-screen the prompt.
  certification({
    name: 'Malformed Certification',
    issuer: 'Some Authority',
    domain: 'authority.example',
    claims: null,
  }),
];

const member = (name: string, description: string | null, domain: string): Member => ({
  logo_uri: null,
  name,
  description,
  domain,
});

interface EcosystemOptions {
  name: string;
  description: string | null;
  members: Member[];
  /** The ecosystem's true size, which can exceed the members actually sent. */
  member_count?: number;
}

const ecosystem = ({
  name,
  description,
  members,
  member_count = members.length,
}: EcosystemOptions): EcosystemProfile => ({
  logo_uri: null,
  name,
  description,
  ecosystem_leader: members[0],
  member_count,
  members,
});

/** Five again, over `PREVIEW_COUNT`, laid out like `certifications` above. */
const ecosystems: EcosystemProfile[] = [
  // Three significant words, so `calculateOrgInitials` renders "GFA".
  ecosystem({
    name: 'Global FinTech Alliance',
    description: 'Financial innovation & compliance',
    members: [
      member('BestDex', 'Digital asset exchange', 'bestdex.com'),
      member('Nordea Bank', 'Retail and corporate banking', 'nordea.com'),
      member('ClearBank NV', null, 'clearbank.nl'),
    ],
  }),
  // "for" is a stop word, so the initials are "DOU", not "DOFU".
  ecosystem({
    name: 'Dutch Organization for Universities',
    description: 'Education & research network',
    members: [
      member('Universiteit van Amsterdam', 'Research university', 'uva.nl'),
      member('TU Delft', 'Technical university', 'tudelft.nl'),
    ],
  }),
  // Single word, which takes the two-letter initials branch ("TR").
  ecosystem({
    name: 'Trustnet',
    description: null,
    members: [member('Sole Operator', null, 'sole.example')],
  }),
  // Member count and member list disagree on purpose: the pill shows the total, the list
  // shows what actually arrived.
  ecosystem({
    name: 'EU Digital Identity Network',
    description: 'Standardizing digital IDs across member states',
    members: [
      member('European Commission', 'Policy and funding', 'ec.europa.eu'),
      member('SURF', 'Dutch research network', 'surf.nl'),
      member('Bundesdruckerei', 'Identity documents', 'bundesdruckerei.de'),
    ],
    member_count: 147,
  }),
  // Long enough to clamp in the card, wrap in the banner, and truncate in the nav bar title.
  ecosystem({
    name: 'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek en Innovatie',
    description:
      'A description long enough to run past the two lines the card allows, so the clamp has something to bite on, while the detail page shows it in full.',
    members: [
      member(
        'Koninklijke Nederlandse Akademie van Wetenschappen',
        'Long enough to truncate on the member row as well',
        'knaw.nl',
      ),
    ],
  }),
];

/**
 * Reached as `/prompt/accept-connection?mock=<name>`, with `PUBLIC_USE_MOCKS=true` in
 * `.env`. Point `PUBLIC_DEV_REDIRECT` at that URL to boot the app straight into one.
 */
export const mocks = {
  /** First contact: domain validated, nothing linked. */
  new: base,
  known: { ...base, connection_data: connected },
  untrusted: {
    ...base,
    domain_validation: {
      status: 'Failure',
      url: 'https://www.bestdex.com/',
      message: 'No did-configuration.json found',
    },
  },
  /** Every layout degradation at once: overlong name, no logo, unparseable URL, unknown status. */
  edge: {
    ...base,
    client_metadata: {
      ...base.client_metadata,
      client_name: 'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek',
      logo_uri: null,
      connection_url: 'not a url',
    },
    domain_validation: { status: 'Unknown', url: 'https://www.bestdex.com/' },
  },
  'unknown-ecos-and-certs': {
    ...base,
    linked_verifiable_presentations: certifications,
    ecosystems,
  },
  /** The same lists on a known connection, where both sections collapse into summary cards. */
  'known-ecos-and-certs': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications,
    ecosystems,
  },
  /** One of each, so the summary cards name it outright with no "and N more" tail. */
  'known-one-each': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications.slice(0, 1),
    ecosystems: ecosystems.slice(0, 1),
  },
} satisfies Record<string, AcceptConnectionPrompt>;

export type MockName = keyof typeof mocks;
