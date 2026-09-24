import '@testing-library/jest-dom';

import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import { render, screen } from '@testing-library/svelte';

import CredentialClaimsRenderer from './CredentialClaimsRenderer.svelte';

const openBadge = {
  id: 'certification-id',
  format: { format: 'jwt_vc_json' },
  issuer_name: 'Example issuer',
  issuer_logo_uri: null,
  data: {
    type: ['VerifiableCredential', 'OpenBadgeCredential'],
    credentialSubject: {
      achievement: {
        description: 'A structured certification description.',
      },
    },
  },
  display_claims: [],
  metadata: {
    is_favorite: false,
    date_added: '2026-01-01T00:00:00Z',
    date_issued: '2026-01-01T00:00:00Z',
  },
  display_name: 'Example certification',
} satisfies DisplayCredential;

describe('CredentialClaimsRenderer', () => {
  test('uses the Open Badge renderer for structured achievement claims', () => {
    render(CredentialClaimsRenderer, { credential: openBadge });

    expect(screen.getByText('A structured certification description.')).toBeInTheDocument();
    expect(screen.queryByText('[object Object]')).not.toBeInTheDocument();
  });
});
