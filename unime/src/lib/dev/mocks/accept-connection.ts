import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

import type { AcceptConnectionPrompt, Certification } from '$lib/dev/accept-connection.types';

const base: AcceptConnectionPrompt = {
  type: 'accept-connection',
  client_name: 'BestDex',
  logo_uri: 'https://bestdex.com/logo.png',
  redirect_uri: 'https://www.bestdex.com/callback',
  connection_data: null,
  domain_validation: { status: 'Success' },
  linked_verifiable_presentations: [],
  ecosystems: [],
};

const certification = (
  name: string,
  issuer?: string,
  domain?: string,
  status: ValidationStatus = 'Success',
): Certification => ({
  name,
  logo_uri: null,
  issuance_date: '2025-03-12T00:00:00Z',
  issuer_domain_validation: issuer ? { status, name: issuer } : { status },
  issuer_linked_domains: domain ? [domain] : [],
});

const connected = {
  first_interacted_at: '2023-04-28T10:12:00Z',
  last_interacted_at: '2023-07-28T09:30:00Z',
  interactions: [],
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
  untrusted: {
    ...base,
    domain_validation: { status: 'Failure', message: 'No did-configuration.json found' },
  },
  'unknown-domain': { ...base, domain_validation: { status: 'Unknown' } },
  'long-name': { ...base, client_name: 'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek' },
  'no-logo': { ...base, logo_uri: undefined },

  // M2 — certifications
  'certs-one': { ...base, linked_verifiable_presentations: certifications.slice(0, 1) },
  // Exactly PREVIEW_COUNT: the section fills up but shows no "Show more" link.
  'certs-preview': { ...base, linked_verifiable_presentations: certifications.slice(0, 3) },
  // Over PREVIEW_COUNT: the "Show more" link appears and the sub-route lists all ten.
  'certs-many': { ...base, linked_verifiable_presentations: certifications },
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
} satisfies Record<string, AcceptConnectionPrompt>;

export type MockName = keyof typeof mocks;
