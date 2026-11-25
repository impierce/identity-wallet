import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import type { AppState } from '@bindings/AppState';
import { fireEvent, render, screen } from '@testing-library/svelte';

import { dispatch } from '$lib/dispatcher';
import { state as appState } from '$lib/stores';

import Page from './+page.svelte';

// The loading spinner's Lottie player needs a canvas, which jsdom lacks.
vi.mock('@lottiefiles/lottie-player', () => ({}));
vi.mock('$lib/dispatcher', () => ({ dispatch: vi.fn() }));
vi.mock('$lib/navigation', () => ({ cancelUserFlow: vi.fn(), useBackNavigation: (back: () => void) => back }));

type TxCode = { input_mode?: 'numeric' | 'text'; length?: number; description?: string };

function renderOffer(tx_code?: TxCode) {
  appState.set({
    current_user_prompt: { type: 'credential-offer', issuer_name: 'Capella', credential_configurations: {}, tx_code },
  } as unknown as AppState);
  return render(Page);
}

const acceptButton = () => screen.getByRole('button', { name: 'Accept credentials' });

describe('credential offer transaction code', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
  });

  beforeEach(() => vi.mocked(dispatch).mockClear());

  test('asks for no code when the offer has none', () => {
    renderOffer();

    expect(screen.queryByText('A code is required to accept these credentials.')).not.toBeInTheDocument();
    expect(screen.queryByRole('textbox')).not.toBeInTheDocument();
    expect(acceptButton()).toBeEnabled();
  });

  test('shows one box per character for a code of up to six characters', () => {
    const { container } = renderOffer({ input_mode: 'numeric', length: 4 });

    expect(container.querySelectorAll('[data-melt-pin-input-input]')).toHaveLength(4);
    expect(screen.queryByPlaceholderText('Enter code')).not.toBeInTheDocument();
    expect(acceptButton()).toBeDisabled();
  });

  test.each([
    ['without a length', { input_mode: 'numeric' as const }],
    ['longer than six characters', { input_mode: 'numeric' as const, length: 8 }],
  ])('shows a single field for a code %s', (_, tx_code) => {
    const { container } = renderOffer(tx_code);

    expect(container.querySelectorAll('[data-melt-pin-input-input]')).toHaveLength(0);
    expect(screen.getByPlaceholderText('Enter code')).toBeInTheDocument();
  });

  test('opens the number pad and keeps only digits for a numeric code in the single field', async () => {
    renderOffer({ length: 8 });
    const field = screen.getByPlaceholderText<HTMLInputElement>('Enter code');

    expect(field).toHaveAttribute('inputmode', 'numeric');
    expect(field).toHaveAttribute('maxlength', '8');

    await fireEvent.input(field, { target: { value: '12-34 ab5' } });

    expect(field.value).toBe('12345');
  });

  test('takes any character for a text code in the single field', async () => {
    renderOffer({ input_mode: 'text' });
    const field = screen.getByPlaceholderText<HTMLInputElement>('Enter code');

    expect(field).toHaveAttribute('inputmode', 'text');
    expect(field).not.toHaveAttribute('maxlength');

    await fireEvent.input(field, { target: { value: 'ab-12' } });

    expect(field.value).toBe('ab-12');
  });

  test('enables accepting only once a code of the given length is entered', async () => {
    renderOffer({ input_mode: 'numeric', length: 8 });
    const field = screen.getByPlaceholderText('Enter code');

    await fireEvent.input(field, { target: { value: '1234567' } });
    expect(acceptButton()).toBeDisabled();

    await fireEvent.input(field, { target: { value: '12345678' } });
    expect(acceptButton()).toBeEnabled();

    await fireEvent.click(acceptButton());
    expect(dispatch).toHaveBeenCalledWith({
      type: '[Credential Offer] Selected',
      payload: { credential_configuration_ids: [], tx_code: '12345678' },
    });
  });

  test('enables accepting with any code when the offer gives no length', async () => {
    renderOffer({});
    const field = screen.getByPlaceholderText('Enter code');

    expect(acceptButton()).toBeDisabled();

    await fireEvent.input(field, { target: { value: '7' } });
    expect(acceptButton()).toBeEnabled();
  });

  test("shows the issuer's description of the code", () => {
    renderOffer({ length: 6, description: 'Use the code from the email we sent you.' });

    expect(screen.getByText('A code is required to accept these credentials.')).toBeInTheDocument();
    expect(screen.getByText('Use the code from the email we sent you.')).toBeInTheDocument();
  });
});
