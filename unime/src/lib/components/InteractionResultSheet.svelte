<script lang="ts">
  import { onDestroy, untrack } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { fade, fly } from 'svelte/transition';

  import type { InteractionOutcome } from '@bindings/interaction_result/InteractionOutcome';

  import { dispatch } from '$lib/dispatcher';
  import { SealCheckFillIcon, WarningCircleFillIcon, XRegularIcon } from '$lib/icons';
  import { state as appState, error, pendingInteraction } from '$lib/stores';

  import Button from './Button.svelte';
  import Toast from './Toast.svelte';

  const AUTO_CLOSE_SECONDS = 3;

  type Phase = 'waiting' | InteractionOutcome['status'];
  type StepState = 'pending' | 'loading' | 'done' | 'failed';

  // `open`: the sheet is visible. `minimized`: the user closed it while still waiting, the outcome arrives as a toast.
  let open = $state(false);
  let minimized = $state(false);
  // Generic errors (not reported as an interaction result) that arrive while waiting.
  let unexpectedError = $state<string | undefined>(undefined);
  let secondStepRevealed = $state(false);
  let countdown = $state<number | undefined>(undefined);
  let copied = $state(false);
  let toast = $state<{ variant: 'info' | 'error'; title: string; detail?: string } | undefined>(undefined);

  let timers: ReturnType<typeof setTimeout>[] = [];
  const later = (fn: () => void, ms: number) => timers.push(setTimeout(fn, ms));
  const clearTimers = () => {
    timers.forEach(clearTimeout);
    timers = [];
  };

  const result = $derived($appState.interaction_result);
  const kind = $derived(result?.kind ?? $pendingInteraction?.kind ?? 'presentation');
  const name = $derived(result?.counterparty || $pendingInteraction?.counterparty || '');
  const outcome = $derived<InteractionOutcome | undefined>(
    result?.outcome ?? (unexpectedError !== undefined ? { status: 'not_sent', reason: unexpectedError } : undefined),
  );
  const phase = $derived<Phase>(outcome?.status ?? 'waiting');
  const texts = $derived(kind === 'issuance' ? $LL.INTERACTION_RESULT.ISSUANCE : $LL.INTERACTION_RESULT.PRESENTATION);

  const steps = $derived.by((): [StepState, string][] => {
    switch (phase) {
      case 'waiting':
        return [
          ['loading', texts.STEP_SENDING({ name })],
          ['pending', texts.STEP_APPROVE({ name })],
        ];
      case 'approved':
        return [
          ['done', texts.STEP_SEND({ name })],
          [secondStepRevealed ? 'done' : 'loading', texts.STEP_APPROVE({ name })],
        ];
      case 'rejected':
        return [
          ['done', texts.STEP_SEND({ name })],
          ['failed', texts.STEP_REJECTED({ name })],
        ];
      case 'not_sent':
        return [
          ['failed', texts.STEP_SEND_FAILED({ name })],
          ['pending', texts.STEP_APPROVE({ name })],
        ];
    }
  });

  const title = $derived.by(() => {
    switch (phase) {
      case 'waiting':
        return texts.SENDING_TITLE({ name });
      case 'approved':
        return $LL.INTERACTION_RESULT.APPROVED_TITLE();
      case 'rejected':
        return texts.REJECTED_TITLE({ name });
      case 'not_sent':
        return $LL.INTERACTION_RESULT.NOT_SENT_TITLE({ name });
    }
  });

  const body = $derived.by(() => {
    if (outcome?.status === 'approved') return texts.APPROVED_BODY({ name });
    if (outcome?.status === 'rejected') return outcome.error_description ?? $LL.INTERACTION_RESULT.REJECTED_BODY();
    if (outcome?.status === 'not_sent') return texts.NOT_SENT_BODY();
    return $LL.INTERACTION_RESULT.SENDING_BODY();
  });

  const errorCode = $derived.by(() => {
    if (outcome?.status === 'rejected') return outcome.error_code ?? $LL.INTERACTION_RESULT.UNKNOWN_ERROR();
    if (outcome?.status === 'not_sent') return unexpectedError !== undefined ? 'unexpected_error' : 'network_error';
    return undefined;
  });

  const errorMeta = $derived(
    outcome?.status === 'rejected' && outcome.http_status
      ? $LL.INTERACTION_RESULT.HTTP_STATUS({ status: outcome.http_status })
      : $LL.INTERACTION_RESULT.NO_RESPONSE(),
  );

  // Only a presentation that could not be sent keeps its flow, so only that one can be retried.
  const canRetry = $derived(
    outcome?.status === 'not_sent' && unexpectedError === undefined && !!$pendingInteraction?.retry,
  );

  // A new interaction starts: show the waiting state.
  let startPromptType: string | undefined;
  $effect(() => {
    if ($pendingInteraction) {
      untrack(() => {
        startPromptType = $appState.current_user_prompt?.type;
        open = true;
        minimized = false;
        unexpectedError = undefined;
      });
    }
  });

  // The backend asks for more input first (e.g. an interactive authorization): step aside for that prompt.
  $effect(() => {
    const promptType = $appState.current_user_prompt?.type;
    if (
      phase === 'waiting' &&
      $pendingInteraction &&
      promptType &&
      promptType !== 'redirect' &&
      promptType !== startPromptType
    ) {
      untrack(() => {
        open = false;
        pendingInteraction.set(undefined);
      });
    }
  });

  // Errors that the backend did not turn into an interaction result still end the waiting state.
  const unsubscribeError = error.subscribe((message) => {
    if (message !== undefined && $pendingInteraction && !result) {
      unexpectedError = message;
      // The sheet shows the error, so skip the error toast.
      error.set(undefined);
    }
  });

  // An outcome arrived. Every state update brings a new copy of the same outcome, so handle each outcome once.
  let handledOutcome: string | undefined;
  $effect(() => {
    const key = outcome ? JSON.stringify(outcome) : undefined;
    if (key === handledOutcome) return;
    handledOutcome = key;
    if (outcome) untrack(() => showOutcome(outcome));
  });

  function showOutcome(outcome: InteractionOutcome) {
    if (minimized) {
      toast = {
        variant: outcome.status === 'approved' ? 'info' : 'error',
        title:
          outcome.status === 'approved'
            ? $LL.INTERACTION_RESULT.LATER_APPROVED_TITLE({ name })
            : $LL.INTERACTION_RESULT.LATER_FAILED_TITLE({ name }),
        detail: outcome.status === 'approved' ? texts.APPROVED_BODY({ name }) : errorCode,
      };
      minimized = false;
      reset();
      return;
    }
    open = true;
    if (outcome.status === 'approved') {
      secondStepRevealed = false;
      later(() => (secondStepRevealed = true), 400);
      later(() => (countdown = AUTO_CLOSE_SECONDS), 450);
      later(() => close(), 450 + AUTO_CLOSE_SECONDS * 1000);
    }
  }

  // Clears the outcome in the backend and the frontend.
  function reset() {
    clearTimers();
    countdown = undefined;
    copied = false;
    unexpectedError = undefined;
    pendingInteraction.set(undefined);
    if (result) dispatch({ type: '[Interaction Result] Dismiss' });
  }

  function close() {
    open = false;
    if (phase === 'waiting') {
      // Keep waiting in the background; the outcome arrives as a toast. The pending interaction stays set until then,
      // so the prompt page keeps its controls disabled while the request is still in flight.
      minimized = true;
      toast = {
        variant: 'info',
        title: $LL.INTERACTION_RESULT.STILL_WAITING_TITLE({ name }),
        detail: $LL.INTERACTION_RESULT.STILL_WAITING_DETAIL(),
      };
      return;
    }
    reset();
  }

  async function retry() {
    const action = $pendingInteraction?.retry;
    if (!action) return;
    clearTimers();
    await dispatch({ type: '[Interaction Result] Dismiss' });
    dispatch(action);
  }

  // Tapping the sheet stops the auto-close countdown.
  function holdOpen() {
    if (countdown === undefined) return;
    clearTimers();
    countdown = undefined;
  }

  async function copyDetails() {
    await navigator.clipboard
      .writeText(JSON.stringify({ kind, counterparty: name, outcome }, null, 2))
      .then(() => (copied = true))
      .catch(() => {});
  }

  onDestroy(() => {
    clearTimers();
    unsubscribeError();
  });
</script>

{#if open}
  <!-- Overlay -->
  <div
    class="fixed inset-0 z-40 bg-black/50 backdrop-blur-xs"
    transition:fade={{ duration: 150 }}
    onclick={close}
    role="presentation"
  ></div>

  <!-- Sheet -->
  <div
    class="fixed bottom-0 left-0 z-40 flex w-screen flex-col items-center gap-5 rounded-t-[28px] bg-white px-6 pt-3 pb-[calc(32px+var(--safe-area-inset-bottom))] dark:bg-dark"
    transition:fly={{ y: 400, duration: 300, opacity: 1 }}
    onclick={holdOpen}
    onkeydown={(event) => event.key === 'Escape' && close()}
    role="dialog"
    aria-modal="true"
    aria-labelledby="interaction-result-title"
    tabindex="-1"
    data-testid="interaction-result"
    data-phase={phase}
  >
    <!-- Grabber and close button -->
    <div class="relative flex h-8 w-full justify-center">
      <div class="h-1 w-9 rounded-full bg-slate-200 dark:bg-slate-600"></div>
      <button
        class="absolute top-0 right-0 flex size-8 items-center justify-center rounded-full bg-slate-100 dark:bg-slate-700"
        aria-label={$LL.CLOSE()}
        onclick={(event) => {
          event.stopPropagation();
          close();
        }}
      >
        <XRegularIcon class="size-4 text-slate-600 dark:text-slate-300" />
      </button>
    </div>

    <!-- Hero -->
    <div class="relative flex size-[120px] items-center justify-center">
      {#if phase === 'waiting'}
        <div
          class="absolute size-[96px] rounded-full bg-primary/15 motion-safe:animate-[breathe_1.2s_ease-in-out_infinite]"
        ></div>
        <div class="absolute size-[72px] rounded-full bg-primary/20"></div>
        <div
          class="size-10 rounded-full border-4 border-primary/25 border-t-primary motion-safe:animate-spin"
          aria-hidden="true"
        ></div>
      {:else if phase === 'approved'}
        <div class="absolute size-[96px] rounded-full bg-primary/15 motion-safe:animate-[burst_600ms_ease-out]"></div>
        <div
          class="flex size-[72px] items-center justify-center rounded-full bg-primary motion-safe:animate-[pop_450ms_cubic-bezier(0.34,1.56,0.64,1)]"
        >
          <SealCheckFillIcon class="size-11 text-white" />
        </div>
      {:else}
        <div class="absolute size-[96px] rounded-full bg-rose-500/10"></div>
        <div
          class="flex size-[72px] items-center justify-center rounded-full bg-rose-500 motion-safe:animate-[shake_320ms_ease-in-out]"
        >
          <WarningCircleFillIcon class="size-11 text-white" />
        </div>
      {/if}
    </div>

    <!-- Heading -->
    <div class="flex w-full flex-col items-center gap-2 text-center">
      <p id="interaction-result-title" class="text-[22px]/[30px] font-semibold text-slate-800 dark:text-grey">
        {title}
      </p>
      <p class="text-[14px]/[20px] text-slate-500 dark:text-slate-300">{body}</p>
    </div>

    <!-- Steps -->
    <div class="flex w-full flex-col gap-3.5 rounded-2xl border border-slate-200 p-4 dark:border-slate-600">
      {#each steps as [stepState, label]}
        <div class="flex items-center gap-3" data-step-state={stepState}>
          {#if stepState === 'pending'}
            <div class="size-5 rounded-full border-2 border-slate-300 dark:border-slate-500"></div>
          {:else if stepState === 'loading'}
            <div class="size-5 rounded-full border-2 border-primary/25 border-t-primary motion-safe:animate-spin"></div>
          {:else if stepState === 'done'}
            <SealCheckFillIcon class="size-5 text-primary" />
          {:else}
            <WarningCircleFillIcon class="size-5 text-rose-500" />
          {/if}
          <p
            class="text-[14px]/[22px] font-medium {stepState === 'pending'
              ? 'text-slate-400'
              : stepState === 'failed'
                ? 'text-rose-500'
                : 'text-slate-800 dark:text-grey'}"
          >
            {label}
          </p>
        </div>
      {/each}
    </div>

    <!-- Error details -->
    {#if errorCode}
      <div class="flex w-full flex-col gap-1.5 rounded-xl bg-rose-50 p-3.5 dark:bg-rose-950/40">
        <div class="flex items-center justify-between gap-2">
          <p class="text-[12px]/[20px] font-medium text-rose-700 dark:text-rose-300">
            {$LL.INTERACTION_RESULT.ERROR_CODE()}
          </p>
          <p
            class="truncate text-[12px]/[20px] font-semibold text-rose-700 dark:text-rose-300"
            data-testid="error-code"
          >
            {errorCode}
          </p>
        </div>
        <p class="text-[12px]/[16px] text-rose-900/70 dark:text-rose-200/70">{errorMeta}</p>
      </div>
    {/if}

    <!-- Auto-close countdown -->
    {#if phase === 'approved'}
      <div class="flex w-full flex-col gap-2" transition:fade={{ duration: 150 }}>
        <div class="h-1 w-full overflow-hidden rounded-full bg-slate-200 dark:bg-slate-600">
          <div
            class="h-full rounded-full bg-primary"
            style:width={countdown === undefined ? '100%' : '0%'}
            style:transition={countdown === undefined ? 'none' : `width ${AUTO_CLOSE_SECONDS}s linear`}
          ></div>
        </div>
        {#if countdown !== undefined}
          <p class="text-center text-[12px]/[16px] font-medium text-slate-500 dark:text-slate-300">
            {$LL.INTERACTION_RESULT.AUTO_CLOSE({ seconds: AUTO_CLOSE_SECONDS })}
          </p>
        {/if}
      </div>
    {/if}

    <!-- Actions -->
    <div class="flex w-full flex-col gap-3">
      {#if phase === 'approved'}
        <Button label={$LL.INTERACTION_RESULT.DONE()} on:click={close} />
      {:else if phase === 'rejected'}
        <Button label={$LL.CLOSE()} on:click={close} />
        <button class="h-10 text-[13px]/[24px] font-medium text-slate-800 dark:text-grey" onclick={copyDetails}>
          {copied ? $LL.INTERACTION_RESULT.COPIED() : $LL.INTERACTION_RESULT.COPY_DETAILS()}
        </button>
      {:else if phase === 'not_sent'}
        {#if canRetry}
          <Button label={$LL.INTERACTION_RESULT.TRY_AGAIN()} on:click={retry} />
        {/if}
        <Button label={$LL.CLOSE()} variant={canRetry ? 'secondary' : 'primary'} on:click={close} />
      {:else}
        <Button label={$LL.CLOSE()} variant="secondary" on:click={close} />
      {/if}
    </div>
  </div>
{/if}

{#if toast}
  <!-- Remount for every new toast so its auto-dismiss timer starts over. -->
  {#key toast}
    <div class="fixed right-4 bottom-[calc(16px+var(--safe-area-inset-bottom))] z-50 w-[calc(100%-32px)]">
      <Toast
        variant={toast.variant}
        title={toast.title}
        detail={toast.detail}
        autoDismissAfterMs={5_000}
        ondismissed={() => (toast = undefined)}
      />
    </div>
  {/key}
{/if}

<style>
  @keyframes -global-breathe {
    0%,
    100% {
      transform: scale(1);
      opacity: 1;
    }
    50% {
      transform: scale(1.18);
      opacity: 0.6;
    }
  }
  @keyframes -global-burst {
    0% {
      transform: scale(0.4);
      opacity: 0;
    }
    60% {
      transform: scale(1.25);
      opacity: 1;
    }
    100% {
      transform: scale(1);
      opacity: 1;
    }
  }
  @keyframes -global-pop {
    0% {
      transform: scale(0.4);
      opacity: 0;
    }
    100% {
      transform: scale(1);
      opacity: 1;
    }
  }
  @keyframes -global-shake {
    0%,
    100% {
      transform: translateX(0);
    }
    25% {
      transform: translateX(-6px);
    }
    75% {
      transform: translateX(6px);
    }
  }
</style>
