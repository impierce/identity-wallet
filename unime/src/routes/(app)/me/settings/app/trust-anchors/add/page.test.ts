import '@testing-library/jest-dom';

import type { AppState } from '@bindings/AppState';
import type { PendingTrustAnchor } from '@bindings/trust_anchors/PendingTrustAnchor';
import { fireEvent, render, screen } from '@testing-library/svelte';

import { dispatch } from '$lib/dispatcher';
import { state as appState } from '$lib/stores';

import { trustAnchor } from '../trustAnchor.fixture';
import Page from './+page.svelte';

// The loading spinner's Lottie player needs a canvas, which jsdom lacks.
vi.mock('@lottiefiles/lottie-player', () => ({}));
vi.mock('$lib/dispatcher', () => ({ dispatch: vi.fn() }));
vi.mock('$lib/navigation', () => ({ useParentRoute: () => vi.fn() }));

function renderWith(pending?: PendingTrustAnchor) {
  appState.set({
    profile_settings: { locale: 'en-US' },
    trust_anchors: { anchors: [], pending },
  } as unknown as AppState);
  return render(Page);
}

const added = trustAnchor({ source: { type: 'added', url: 'https://ca.example/root.crt' }, enabled: true });

describe('adding a trust anchor', () => {
  beforeEach(() => vi.mocked(dispatch).mockClear());

  test('fetches the certificate at the URL entered', async () => {
    renderWith();

    await fireEvent.input(screen.getByPlaceholderText('https://ca.example/root.crt'), {
      target: { value: ' https://ca.example/root.crt ' },
    });
    await fireEvent.click(screen.getByRole('button', { name: 'Fetch certificate' }));

    expect(dispatch).toHaveBeenCalledWith({
      type: '[Trust Anchors] Fetch',
      payload: { url: 'https://ca.example/root.crt' },
    });
  });

  test('shows a fetched certificate to accept or deny', async () => {
    renderWith({ anchor: added });

    expect(screen.getByText('Authbound EUDI Testbed Issuer Issuing CA')).toBeInTheDocument();
    expect(screen.getByText('Added from https://ca.example/root.crt')).toBeInTheDocument();
    expect(screen.getByText('AB:CD')).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'Accept' }));
    expect(dispatch).toHaveBeenCalledWith({ type: '[Trust Anchors] Accept' });
  });

  test('explains why a certificate cannot be a trust anchor, and offers no way to accept it', async () => {
    renderWith({ anchor: added, refusal: 'not_ca' });

    expect(screen.getByTestId('trust-anchor-refusal')).toHaveTextContent('not a CA certificate');
    expect(screen.queryByRole('button', { name: 'Accept' })).not.toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: 'Close' }));
    expect(dispatch).toHaveBeenCalledWith({ type: '[Trust Anchors] Deny' });
  });

  test('discards a certificate under review when the screen is left', () => {
    const { unmount } = renderWith({ anchor: added });

    unmount();

    expect(dispatch).toHaveBeenCalledWith({ type: '[Trust Anchors] Deny' });
  });
});
