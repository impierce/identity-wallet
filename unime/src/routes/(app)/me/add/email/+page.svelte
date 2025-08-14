<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { PinInput } from 'melt/builders';
  import { cubicOut } from 'svelte/easing';
  import { Tween } from 'svelte/motion';
  import { get } from 'svelte/store';
  import { z } from 'zod';

  import { debug, info } from '@tauri-apps/plugin-log';

  import { Button, TopNavBar } from '$lib/components';
  import CircularProgressBar from '$lib/components/CircularProgressBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState } from '$lib/stores';

  // When the current verification case has expired, the user should be informed about it.
  // However, we do not want to leave the expired case open forever, but reset the forms after a given period.
  const AUTO_DISCARD_AFTER_MINUTES = 5;

  const pinInput = new PinInput({
    type: 'numeric',
    maxLength: 4,
    placeholder: '',
    allowPaste: false,
    onValueChange() {
      showError = false;
    },
    onComplete(value) {
      redeemCode(value);
    },
  });

  let label: string = $state('');
  let email: string = $state('');

  // Input validation
  const emailSchema = z.string().email({ message: get(LL).ADD_CREDENTIALS.EMAIL.ADD.VALUE_PATTERN_ERROR() });
  let emailSchemaError: string | undefined = $state(undefined);
  let showEmailSchemaError: boolean = $state(false);

  // Svelte 5's `bind:this` with `let` is idiomatic
  // svelte-ignore non_reactive_update
  let labelInput: HTMLInputElement | undefined = undefined;

  let showError: boolean = $state(false);

  // We need to hide some elements, so the Android keyboard doesn't cover the input.
  let hideForm = $state(false);

  let loading = $state(false);

  let pending = $state(false);

  let expired = $state(false);

  let progressValue = new Tween(0, {
    duration: 400,
    easing: cubicOut,
  });

  function formatTime(totalSeconds: number): string {
    const positiveSeconds = Math.max(0, Math.floor(totalSeconds)); // Ensure integer and non-negative
    const minutes = Math.floor(positiveSeconds / 60);
    const seconds = positiveSeconds % 60;
    const paddedMinutes = String(minutes).padStart(2, '0');
    const paddedSeconds = String(seconds).padStart(2, '0');
    return `${paddedMinutes}:${paddedSeconds}`;
  }

  let displayTime = $derived(formatTime(progressValue.current));

  function validateEmailSchema() {
    const result = emailSchema.safeParse(email);
    if (result.success) {
      emailSchemaError = undefined;
    } else {
      emailSchemaError = result.error.errors[0].message;
    }
  }

  let secsRemaining = 0;

  let interval: ReturnType<typeof setInterval>;

  const startTimer = (seconds: number) => {
    interval = setInterval(() => {
      // debug(`seconds: ${seconds}`);
      if (seconds < 1) {
        clearInterval(interval);
        expired = true;
        pending = false;
        progressValue.set(0);
      }
      //   if (seconds === 1) {
      //     expired = true;
      //     awaitingConfirmation = false;
      //   }
      seconds--;
      if (seconds >= 0) {
        secsRemaining = seconds;
        // progress = Math.floor((seconds / MAX_SECONDS) * 100);
        progressValue.set(secsRemaining);
      }
    }, 1000);
  };

  const startVerificationSession = async () => {
    loading = true;

    await dispatch({ type: '[Verified Data] Send verification email', payload: { email, label } });

    const expires_at = $appState.verified_data.email_verification?.expires_at;
    if (!expires_at) {
      debug('No `expires_at` found in app state');
      loading = false;
      return;
    }

    const expires_in_seconds = Math.floor((new Date(expires_at).getTime() - Date.now()) / 1_000);

    pinInput.value = '';

    loading = false;

    hideForm = true;

    progressValue.set(expires_in_seconds);
    pending = true;
    expired = false;
    // emailSentTimestamp = new Date();
    startTimer(expires_in_seconds);
  };

  async function redeemCode(code: string) {
    await dispatch({ type: '[Verified Data] Redeem code', payload: { code } });
    setTimeout(() => {
      pinInput.value = '';
      showError = true;
    }, 250);
  }

  const reset = () => {
    expired = false;
    pending = false;
    hideForm = false;
    labelInput?.focus();
    dispatch({ type: '[Verified Data] Reset email verification' });
  };

  const checkAutoDiscard = (expires_at: Date) => {
    const expired_mins_ago = Math.floor((new Date().getTime() - expires_at.getTime()) / 1_000 / 60);
    info(`expired_mins_ago: ${expired_mins_ago}`);
    if (expired_mins_ago >= AUTO_DISCARD_AFTER_MINUTES) {
      debug(`Auto-discarding expired verification case (${expired_mins_ago} minutes ago) ...`);
      reset();
      // TODO: bug: still shows the expired timer
    }
  };

  onMount(async () => {
    dispatch({
      type: '[Verified Data] Check service health',
      payload: { service: 'email-verification-service' },
    });

    const current = $appState.verified_data.email_verification;

    if (current) {
      hideForm = true;
      email = current.email;
      label = current.label;

      // Resume verification timer across app restarts by reading from app state
      if (current.expires_at) {
        checkAutoDiscard(new Date(current.expires_at));

        info('Resuming existing email verification timer');
        //   emailSentTimestamp = new Date($appState.verified_data.email_verification.expires_at);
        const expires_in_secs = Math.floor((new Date(current.expires_at).getTime() - Date.now()) / 1_000);
        //   info(`emailSentTimestamp: ${emailSentTimestamp}`);
        //   const diff = emailSentTimestamp.getTime() - Date.now();
        // const diff = expires_in_secs / 1_000;
        // info(`diff: ${diff}`);
        // max = diff;
        if (expires_in_secs <= 0) {
          progressValue.set(0);
          pending = false;
          expired = true;
        } else {
          // if (diff > MAX_SECONDS * 1_000) {
          //   progressValue.set(0);
          //   awaitingConfirmation = false;
          //   expired = true;
          // } else {
          // progressValue.set(MAX_SECONDS - diff / 1_000);
          // startTimer(MAX_SECONDS - diff / 1_000);
          progressValue.set(expires_in_secs);
          startTimer(expires_in_secs);
          pending = true;
          // }
        }
      } else {
        debug('No current email verification timer found in app state');
      }
    } else {
      hideForm = false;
      labelInput?.focus();
      if ($appState.dev_mode !== 'Off') {
        label = 'Personal Email';
        email = 'ferris.rustacean@example.test';
      }
    }
  });

  onDestroy(() => {
    // Clearing the interval to avoid duplicate counters when the page is loaded next time
    info('clearingInterval');
    clearInterval(interval);
  });
</script>

<TopNavBar
  on:back={() => goto('/me/add')}
  title={$LL.ADD_CREDENTIALS.EMAIL.ADD.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<!-- The 50px height of the TopNavBar are manually subtracted -->
<div class="relative flex h-[calc(100%_-_50px)] flex-col">
  <div class="flex grow flex-col items-center p-4">
    {#if !hideForm}
      <div class="mb-8 mt-4 flex w-full flex-col gap-1">
        <div class="flex items-center justify-between">
          <label for="label" class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">
            {$LL.ADD_CREDENTIALS.EMAIL.ADD.LABEL()}
          </label>
        </div>
        <input
          name="label"
          type="text"
          class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-800 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
          placeholder={$LL.ADD_CREDENTIALS.EMAIL.ADD.LABEL_PLACEHOLDER()}
          bind:value={label}
          bind:this={labelInput}
          oninput={() => {
            // TODO: when the label is changed after a verification session has expired, reset everything.
          }}
          disabled={pending}
        />
        <div class="pt-1 text-[12px]/[14px] font-medium text-primary">
          {$LL.ADD_CREDENTIALS.LABEL_DISCLAIMER()}
        </div>

        <!-- Divider -->
        <div class="my-4 h-px bg-slate-300"></div>

        <label for="email" class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">
          {$LL.ADD_CREDENTIALS.EMAIL.ADD.VALUE_LABEL()}
        </label>
        <input
          name="email"
          type="email"
          class="w-full rounded-xl border border-slate-300 bg-background-alt px-4 py-3 text-[13px]/[24px] font-normal text-slate-800 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:text-slate-300 dark:caret-slate-300"
          placeholder={$LL.ADD_CREDENTIALS.EMAIL.ADD.VALUE_PLACEHOLDER()}
          bind:value={email}
          oninput={() => {
            validateEmailSchema();
            // When the email is changed after a verification session has expired, reset everything.
            if (expired) {
              reset();
            }
          }}
          onblur={() => (showEmailSchemaError = true)}
          disabled={pending}
        />
        {#if showEmailSchemaError && emailSchemaError}
          <div class="mt-1 text-[12px]/[14px] font-medium text-rose-500">
            {emailSchemaError}
          </div>
        {/if}
      </div>
    {:else}
      <div class="p-8 pt-0 text-[14px]/[22px] font-medium text-slate-500 dark:text-grey">
        {$LL.ADD_CREDENTIALS.EMAIL.ADD.CHECK_EMAIL()}
      </div>
    {/if}

    {#if pending || expired}
      {#key progressValue.current}
        <CircularProgressBar
          class="stroke-slate-200 dark:stroke-background-alt"
          max={$appState.verified_data.email_verification?.validation_expiration_in_secs ?? 0}
          min={0}
          value={progressValue.current}
          displayValue={displayTime}
          gaugePrimaryColor={`${progressValue.target > 0 ? 'rgb(var(--color-brand))' : 'oklch(0.586 0.253 17.585)'} `}
        />
      {/key}

      <div {...pinInput.root} class="mt-8 flex items-center justify-center gap-2 font-mono">
        {#each pinInput.inputs as input}
          <input
            {...input}
            class="size-12 rounded-xl border border-slate-300 bg-background-alt text-center text-2xl font-semibold text-text-alt outline-none focus:border-primary disabled:cursor-not-allowed dark:border-slate-500"
            disabled={!pending}
          />
        {/each}
      </div>

      <!-- Errors -->
      {#if showError && $appState.verified_data.email_verification?.error}
        <div class="mt-4 flex flex-col items-center">
          <div class="rounded-full bg-rose-100 px-4 py-3 text-sm font-semibold text-rose-500">
            <span>{$appState.verified_data.email_verification?.error}</span>
          </div>
        </div>
      {/if}

      {#if expired}
        <div class="mt-4 flex flex-col items-center">
          <div class="rounded-full bg-rose-100 px-4 py-3 text-sm font-semibold text-rose-500">
            <span>{$LL.ADD_CREDENTIALS.EMAIL.ADD.EXPIRED_ERROR()}</span>
          </div>
        </div>
      {/if}
    {/if}

    <!-- <div class="pt-4 text-sm text-slate-400 dark:text-slate-500">
      <span>Verified by</span> <span class="font-semibold">Impierce Technologies B.V.</span>
    </div> -->
  </div>

  <!-- TODO: REFACTOR! -->
  <div class="absolute bottom-0 left-0 z-10 flex w-full flex-col gap-3 rounded-t-3xl bg-background-alt p-6">
    {#if expired || $appState.dev_mode !== 'Off'}
      <Button label={$LL.DISCARD()} variant="secondary" on:click={reset} />
    {/if}
    <Button
      label={expired || pending
        ? $LL.ADD_CREDENTIALS.EMAIL.ADD.BUTTON_SEND_AGAIN()
        : $LL.ADD_CREDENTIALS.EMAIL.ADD.BUTTON_SEND()}
      on:click={() => startVerificationSession()}
      {loading}
      disabled={pending || !!emailSchemaError || label.length === 0 || email.length === 0}
    />
  </div>
</div>
