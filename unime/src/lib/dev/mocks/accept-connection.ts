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
  client_name: 'BestDex',
  logo_uri: 'https://bestdex.com/logo.png',
  redirect_uri: 'https://www.bestdex.com/callback',
  domain_validation: { status: 'Success', url: 'https://www.bestdex.com/' },
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
): LinkedVerifiableCredentialData => ({
  credential: {
    id: slug(name),
    format: { format: 'jwt_vc_json' },
    issuer_name: issuer ?? '',
    issuer_logo_uri: null,
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

/** Marks a certification as having an issuer logo the backend downloaded. */
const withIssuerLogo = (
  certification: LinkedVerifiableCredentialData,
  url: string,
): LinkedVerifiableCredentialData => ({
  ...certification,
  credential: { ...certification.credential, issuer_logo_uri: url },
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

const certifications: LinkedVerifiableCredentialData[] = [
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

const member = (name: string, description: string | null, domain: string): Member => ({
  logo_uri: null,
  name,
  description,
  domain,
});

// `member_count` (what the ecosystem has) and `members` (what the payload carries) are separate
// fields and need not match. Defaulted to keep them in step; override to model the mismatch.
const ecosystem = (
  name: string,
  description: string | null,
  members: Member[],
  member_count: number = members.length,
): EcosystemProfile => ({
  logo_uri: null,
  name,
  description,
  ecosystem_leader: members[0],
  member_count,
  members,
});

const universities = ecosystem('Dutch Organization for Universities', 'Education & research network', [
  member('University of Harderwijk', 'Leading research institution', 'university.harderwijk.com'),
  member('Finara NL', 'Financial compliance partners', 'finara.nl'),
  member('Optimizor', 'Data-driven insight tooling', 'optimizor.io'),
  member('TU Delft', 'Technical university', 'tudelft.nl'),
  member('Radboud Universiteit', 'Research university', 'ru.nl'),
  member('Hogeschool Utrecht', 'University of applied sciences', 'hu.nl'),
]);

const ecosystems: EcosystemProfile[] = [
  universities,
  ecosystem('Global FinTech Alliance', 'Financial innovation & compliance', [
    member('Finara NL', 'Financial compliance partners', 'finara.nl'),
    member('PayStream', 'Cross-border payments', 'paystream.com'),
    member('Northgate Capital', 'Institutional asset management', 'northgate.capital'),
    member('Vault22', 'Custody and settlement', 'vault22.io'),
    member('ClearBank NV', 'Clearing and reconciliation', 'clearbank.nl'),
  ]),
  ecosystem('EU Digital Identity Network', 'Standardizing digital IDs in Europe', [
    member('European Commission', 'Policy and standards body', 'ec.europa.eu'),
    member('SignaTrust', 'Qualified trust service provider', 'signatrust.eu'),
    member('IDunion', 'Decentralised identity network', 'idunion.org'),
    member('Bundesdruckerei', 'National document authority', 'bundesdruckerei.de'),
    member('eIDAS Node NL', 'Cross-border eID gateway', 'eidas.nl'),
  ]),
  ecosystem('Open Banking Consortium', 'API governance for banking', [
    member('Bank of Harderwijk', 'Regional retail bank', 'bankofharderwijk.nl'),
    member('LedgerWorks', 'Open banking infrastructure', 'ledgerworks.dev'),
    member('Tulip Pay', 'Payment initiation services', 'tulippay.nl'),
    member('Meridian Bank', 'Commercial banking', 'meridianbank.eu'),
  ]),
  ecosystem('Nordic Trust Framework', 'Cross-border security framework', [
    member('Nordic Digital Authority', 'Government trust anchor', 'nordicdigital.no'),
    member('Svensk Identitet', 'National identity provider', 'svenskidentitet.se'),
    member('Suomi.fi', 'Finnish public service gateway', 'suomi.fi'),
  ]),
  ecosystem('Healthcare Data Alliance', 'Patient data interoperability', [
    member('Harderwijk Medical Centre', 'Teaching hospital', 'hmc.nl'),
    member('CareSync', 'Health record exchange', 'caresync.health'),
    member('Nictiz', 'Standards for healthcare IT', 'nictiz.nl'),
    member('LabNet', 'Diagnostic laboratory network', 'labnet.nl'),
    member('Zorgdomein', 'Referral and results platform', 'zorgdomein.nl'),
  ]),
];

export const mocks = {
  // M1
  new: base,
  known: { ...base, connection_data: connected },
  // Connected, but nothing has happened since: the interaction count reads one.
  'known-no-data': { ...base, connection_data: { ...connected, interactions: interactions.slice(0, 1) } },
  untrusted: {
    ...base,
    domain_validation: {
      status: 'Failure',
      url: 'https://www.bestdex.com/',
      message: 'No did-configuration.json found',
    },
  },
  'unknown-domain': { ...base, domain_validation: { status: 'Unknown', url: 'https://www.bestdex.com/' } },
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
  // Known connection with certifications: the section collapses into the summary card,
  // whose logo stack is full at three and whose "See all" links to the list sub-route.
  'known-certs': {
    ...base,
    connection_data: connected,
    linked_verifiable_presentations: certifications.slice(0, 3),
  },
  // A single badge in the stack, and the count label in the singular.
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
  // An issuer logo the backend has resolved and downloaded. This still renders the fallback
  // badge in DEV: `<Image>` looks for `assets/tmp/<hash(url)>`, which only exists once the
  // backend has written the file. Kept so the shape is represented.
  'cert-logo': {
    ...base,
    linked_verifiable_presentations: [
      withIssuerLogo(
        certification('ISO 27001 Certified', 'Intl. Organization for Standardization', 'iso.org'),
        'https://iso.org/badge.png',
      ),
    ],
  },

  // No "Show more" link below the preview count.
  'eco-one': { ...base, ecosystems: ecosystems.slice(0, 1) },
  'eco-preview': { ...base, ecosystems: ecosystems.slice(0, 3) },
  // Past the preview count: truncates and links to the full list.
  'eco-many': { ...base, ecosystems },
  // Known connection: both sections collapse to summary cards.
  'known-eco': { ...base, connection_data: connected, ecosystems },
  // Summary names the single ecosystem outright instead of "and N more".
  'known-eco-one': { ...base, connection_data: connected, ecosystems: ecosystems.slice(0, 1) },
  'certs-and-eco': { ...base, linked_verifiable_presentations: certifications.slice(0, 2), ecosystems },
  // Every optional field absent. Nothing should render an empty block.
  'eco-bare': {
    ...base,
    ecosystems: [ecosystem('Bare Ecosystem', null, [member('Sole Member', null, 'sole.example')])],
  },
  // Singular "1 Member" pill, with the descriptions `eco-bare` omits.
  'eco-one-member': {
    ...base,
    ecosystems: [
      ecosystem('Solo Consortium', 'Exactly one member', [member('Only Org', 'The sole participant', 'only.example')]),
    ],
  },
  // Claims 147 members but carries 6: pill shows the total, list shows what arrived.
  'eco-count-exceeds-list': {
    ...base,
    ecosystems: [{ ...universities, member_count: 147 }],
  },
  // Long enough to wrap in the card, the nav bar and the banner.
  'eco-long-name': {
    ...base,
    ecosystems: [
      ecosystem(
        'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek en Innovatie',
        'A description long enough to run past the two lines the card allows, so the clamp has something to bite on.',
        [
          member(
            'Koninklijke Nederlandse Akademie van Wetenschappen',
            'A member name long enough to need truncating in the row',
            'knaw.nl',
          ),
          member('TU Delft', 'Technical university', 'tudelft.nl'),
        ],
      ),
    ],
  },
  // Still renders the fallback monogram in DEV — `assets/tmp/<hash(url)>` only exists once the
  // backend has written it. Kept so the shape is represented. Mirrors `cert-logo`.
  'eco-logo': {
    ...base,
    ecosystems: [{ ...universities, logo_uri: 'https://university.harderwijk.com/logo.png' }],
  },
} satisfies Record<string, AcceptConnectionPrompt>;

export type MockName = keyof typeof mocks;
