import type { AcceptConnectionPrompt } from '$lib/dev/accept-connection.types';

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

export const mocks = {
  // M1
  new: base,
  known: {
    ...base,
    connection_data: {
      first_interacted_at: '2023-04-28T10:12:00Z',
      last_interacted_at: '2023-07-28T09:30:00Z',
      interactions: [],
    },
  },
  untrusted: {
    ...base,
    domain_validation: { status: 'Failure', message: 'No did-configuration.json found' },
  },
  'unknown-domain': { ...base, domain_validation: { status: 'Unknown' } },
  'long-name': { ...base, client_name: 'Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek' },
  'no-logo': { ...base, logo_uri: undefined },
} satisfies Record<string, AcceptConnectionPrompt>;

export type MockName = keyof typeof mocks;
