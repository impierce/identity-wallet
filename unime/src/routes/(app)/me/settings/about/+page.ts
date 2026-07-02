import type { PageLoad } from './$types';

interface Specification {
  id: string;
  description: string;
  version: string;
  url: string;
}

export const load = (async () => {
  // https://github.com/impierce/openid4vc?tab=readme-ov-file#rust-library-for-openid-for-verifiable-credentials

  const specifications: Specification[] = [
    {
      id: 'OID4VCI',
      description: 'OpenID for Verifiable Credential Issuance',
      version: 'Final 1.0 (published: 16 September 2025)',
      url: 'https://openid.net/specs/openid-4-verifiable-credential-issuance-1_0.html',
    },
    {
      id: 'OID4VP',
      description: 'OpenID for Verifiable Presentations',
      version: 'Final 1.0 (published: 9 July 2025)',
      url: 'https://openid.net/specs/openid-4-verifiable-presentations-1_0.html',
    },
    {
      id: 'SIOPv2',
      description: 'Self-Issued OpenID Provider v2',
      version: 'Working Group Draft 13 (published: 28 November 2023)',
      url: 'https://openid.net/specs/openid-connect-self-issued-v2-1_0-13.html',
    },
  ];

  return {
    specifications,
  };
}) satisfies PageLoad;
