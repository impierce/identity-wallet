import '@testing-library/jest-dom';

import { tick } from 'svelte';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import type { InteractionResult } from '@bindings/interaction_result/InteractionResult';
import { fireEvent, render, screen, waitFor } from '@testing-library/svelte';

import { dispatch } from '$lib/dispatcher';
import { state as appState, pendingInteraction } from '$lib/stores';

import InteractionResultSheet from './InteractionResultSheet.svelte';

vi.mock('$lib/dispatcher', () => ({ dispatch: vi.fn() }));
// The spinner of a button draws on a canvas, which jsdom does not have.
vi.mock('@lottiefiles/lottie-player', () => ({}));

const accepted = (continue_to: string | null): InteractionResult => ({
  kind: 'presentation',
  counterparty: 'Verifier',
  outcome: { status: 'approved' },
  continue_to,
});

const show = async (interaction_result: InteractionResult | null) => {
  appState.update((state) => ({ ...state, interaction_result }));
  await tick();
};

describe('InteractionResultSheet', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
    // jsdom has no Web Animations, which the sheet's transitions use: here an animation is over as soon as it starts.
    Element.prototype.animate ??= () => {
      const animation = { cancel: () => {}, onfinish: null as (() => void) | null };
      setTimeout(() => animation.onfinish?.());
      return animation as unknown as Animation;
    };
  });

  beforeEach(async () => {
    vi.mocked(dispatch).mockClear();
    await show(null);
  });

  test('an accepted presentation closes itself', async () => {
    render(InteractionResultSheet);
    await show(accepted(null));

    expect(screen.getByRole('button', { name: 'Done' })).toBeInTheDocument();
    expect(screen.getByText(/Closing automatically in/)).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: /Continue to/ })).not.toBeInTheDocument();
  });

  test('a verifier that asks to continue is named on the button, and the sheet waits for the user', async () => {
    render(InteractionResultSheet);
    await show(accepted('verifier.example'));

    expect(screen.getByRole('button', { name: 'Continue to verifier.example' })).toBeInTheDocument();
    expect(screen.getByTestId('continue-to')).toHaveTextContent(
      'Verifier asks you to continue at verifier.example. This opens your browser.',
    );
    expect(screen.getByRole('button', { name: 'Done' })).toBeInTheDocument();
    expect(screen.queryByText(/Closing automatically in/)).not.toBeInTheDocument();
    expect(dispatch).not.toHaveBeenCalled();
  });

  test('continuing asks the backend to open the address, and the sheet closes once it has', async () => {
    render(InteractionResultSheet);
    await show(accepted('verifier.example'));

    await fireEvent.click(screen.getByRole('button', { name: 'Continue to verifier.example' }));
    expect(dispatch).toHaveBeenCalledExactlyOnceWith({ type: '[Interaction Result] Continue' });
    // The backend has not answered: the sheet is still there, for another try if the browser cannot be opened.
    expect(screen.getByTestId('interaction-result')).toBeInTheDocument();

    await show(null);
    await waitFor(() => expect(screen.queryByTestId('interaction-result')).not.toBeInTheDocument());
    expect(dispatch).toHaveBeenCalledTimes(1);
  });

  describe('past the time an accepted result closes itself', () => {
    // Longer than the countdown of five seconds and the delay before it starts.
    const PAST_AUTO_CLOSE_MS = 6_000;

    beforeEach(() => {
      vi.useFakeTimers();
    });

    afterEach(() => {
      vi.useRealTimers();
    });

    test('a result without a continuation is dismissed', async () => {
      render(InteractionResultSheet);
      await show(accepted(null));

      await vi.advanceTimersByTimeAsync(PAST_AUTO_CLOSE_MS);

      expect(dispatch).toHaveBeenCalledExactlyOnceWith({ type: '[Interaction Result] Dismiss' });
    });

    test('a result with a continuation stays', async () => {
      render(InteractionResultSheet);
      await show(accepted('verifier.example'));

      await vi.advanceTimersByTimeAsync(PAST_AUTO_CLOSE_MS);

      expect(screen.getByRole('button', { name: 'Continue to verifier.example' })).toBeInTheDocument();
      expect(dispatch).not.toHaveBeenCalled();
    });

    test('a result with a continuation that follows one without stays as well', async () => {
      render(InteractionResultSheet);
      await show(accepted(null));
      // The countdown of the first result is running when the second one takes its place.
      await vi.advanceTimersByTimeAsync(2_000);
      await show(accepted('verifier.example'));

      expect(screen.getByRole('button', { name: 'Continue to verifier.example' })).toBeInTheDocument();

      await vi.advanceTimersByTimeAsync(PAST_AUTO_CLOSE_MS);

      expect(screen.getByRole('button', { name: 'Continue to verifier.example' })).toBeInTheDocument();
      expect(screen.queryByText(/Closing automatically in/)).not.toBeInTheDocument();
      expect(dispatch).not.toHaveBeenCalled();
    });
  });

  test('the sheet that comes back for a continuation takes the place of the waiting toast', async () => {
    render(InteractionResultSheet);
    pendingInteraction.set({ kind: 'presentation', counterparty: 'Verifier' });
    await tick();
    // The user closes the sheet while the verifier has not answered: a toast says it is still waiting.
    await fireEvent.click(screen.getAllByRole('button', { name: 'Close' })[0]);
    expect(screen.getByText('Still waiting for Verifier')).toBeInTheDocument();

    await show(accepted('verifier.example'));

    expect(screen.getByRole('button', { name: 'Continue to verifier.example' })).toBeInTheDocument();
    expect(screen.queryByText('Still waiting for Verifier')).not.toBeInTheDocument();
    pendingInteraction.set(undefined);
  });

  test('done declines to continue', async () => {
    render(InteractionResultSheet);
    await show(accepted('verifier.example'));

    await fireEvent.click(screen.getByRole('button', { name: 'Done' }));
    expect(dispatch).toHaveBeenCalledExactlyOnceWith({ type: '[Interaction Result] Dismiss' });
  });
});
