<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { dev } from '$app/environment';
  import LL from '$i18n/i18n-svelte';
  import { PinInput } from 'melt/builders';
  import { cubicOut } from 'svelte/easing';
  import { Tween } from 'svelte/motion';

  import { debug, info } from '@tauri-apps/plugin-log';

  import { Button, TopNavBar } from '$lib/components';
  import CircularProgressBar from '$lib/components/CircularProgressBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState } from '$lib/stores';

  const pinInput = new PinInput({
    type: 'numeric',
    maxLength: 4,
    onValueChange(value) {
      // onComplete(value) does not seem to trigger on "melt 0.17.0", that's why we use onValueChange and count the length
      if (value.length === 4) {
        redeemCode(value);
      }
    },
  });

  let email: string = $state(dev ? 'ferris.rustacean@impierce.com' : '');

  let loading = $state(false);

  let awaitingConfirmation = $state(false);

  let expired = $state(false);

  let progressValue = new Tween(0, {
    duration: 400,
    easing: cubicOut,
  });

  // Gauge
  let max = $state(60);

  let secsRemaining = 0;

  let interval: ReturnType<typeof setInterval>;

  const startTimer = (seconds: number) => {
    interval = setInterval(() => {
      // debug(`seconds: ${seconds}`);
      if (seconds < 1) {
        clearInterval(interval);
        expired = true;
        awaitingConfirmation = false;
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
    console.log(`sending request to email-verification-service for email: ${email}`);

    await dispatch({ type: '[Verified Data] Send verification email', payload: { email } });

    console.log(`$appState expiry: ${$appState.verified_data.email_verification?.expires_at}`);

    const expires_at = $appState.verified_data.email_verification?.expires_at!!;
    const expires_in_seconds = Math.round((new Date(expires_at).getTime() - Date.now()) / 1_000);

    console.log(`expires_in_seconds: ${expires_in_seconds}`);

    // Reset PIN input
    pinInput.value = '';

    loading = false;
    progressValue.set(expires_in_seconds);
    awaitingConfirmation = true;
    expired = false;
    // emailSentTimestamp = new Date();
    startTimer(expires_in_seconds);
  };

  async function redeemCode(code: string) {
    console.log(`TODO: trying to redeem code: ${code}`);
    dispatch({ type: '[Verified Data] Redeem code', payload: { code } });
  }

  const reset = () => {
    // TODO: send action to backend to clear state
    expired = false;
    awaitingConfirmation = false;
  };

  onMount(() => {
    // Resume verification timer across app restarts by reading from app state
    if ($appState.verified_data.email_verification?.expires_at) {
      info('Resuming email verification timer');
      //   emailSentTimestamp = new Date($appState.verified_data.email_verification.expires_at);
      const expires_at = $appState.verified_data.email_verification?.expires_at!!;
      const expires_in_ms = new Date(expires_at).getTime() - Date.now();
      //   info(`emailSentTimestamp: ${emailSentTimestamp}`);
      //   const diff = emailSentTimestamp.getTime() - Date.now();
      const diff = expires_in_ms / 1_000;
      info(`diff: ${diff}`);
      max = diff;
      if (diff <= 0) {
        progressValue.set(0);
        awaitingConfirmation = false;
        expired = true;
      } else {
        // if (diff > MAX_SECONDS * 1_000) {
        //   progressValue.set(0);
        //   awaitingConfirmation = false;
        //   expired = true;
        // } else {
        // progressValue.set(MAX_SECONDS - diff / 1_000);
        // startTimer(MAX_SECONDS - diff / 1_000);
        progressValue.set(diff);
        startTimer(diff);
        awaitingConfirmation = true;
        // }
      }
    } else {
      debug('No email verification timer found in app state');
    }
  });

  onDestroy(() => {
    // Clearing the interval to avoid duplicate counters when the page is loaded next time
    info('clearingInterval');
    clearInterval(interval);
  });
</script>

<TopNavBar on:back={() => history.back()} title={'Verified email'} class="sticky top-0 z-10" />

<div class="flex h-[calc(100vh-48px-64px)] flex-col">
  <div class="flex grow flex-col items-center p-4">
    <div class="mb-8 mt-4 flex w-3/4 flex-col gap-1">
      <label for="email" class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300"> Your email </label>
      <input
        name="email"
        type="email"
        class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 disabled:cursor-not-allowed disabled:opacity-50 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
        placeholder={'Your email address'}
        bind:value={email}
        oninput={() => {
          // When the email is changed after a verification session has expired, reset everything.
          if (expired) {
            reset();
          }
        }}
        disabled={awaitingConfirmation}
      />
    </div>

    {#if awaitingConfirmation || expired}
      <!-- {emailSentTimestamp?.toISOString()} -->

      <!-- 
        color-brand: "primary": #5cc7c7, rgb(92, 199, 199)
        rose-600: oklch(0.586 0.253 17.585)

        gaugeSecondaryColor="rgba(0, 50, 100, 0.1)"
      -->

      {#key progressValue.current}
        <CircularProgressBar
          max={$appState.verified_data.email_verification?.validation_expiration_in_secs ?? 0}
          min={0}
          value={progressValue.current}
          gaugePrimaryColor={`${progressValue.target > 0 ? 'rgb(var(--color-brand))' : 'oklch(0.586 0.253 17.585)'} `}
          gaugeSecondaryColor="rgba(0, 50, 100, 0.1)"
        />
      {/key}

      <div {...pinInput.root} class="mt-8 flex items-center justify-center gap-4 font-mono">
        {#each pinInput.inputs as input}
          <input
            {...input}
            class="size-14 rounded-xl border-2 border-slate-300 bg-background-alt text-center text-2xl font-semibold text-text-alt outline-none focus:border-primary disabled:cursor-not-allowed dark:border-slate-500"
            disabled={!awaitingConfirmation}
          />
        {/each}
      </div>

      <!-- <div use:melt={$root} class="flex items-center gap-2">
          {#each Array.from({ length: 4 }) as _}
            <input
              class="size-12 rounded-md border-2 border-slate-500 bg-background-alt text-center text-lg font-semibold text-slate-500"
              use:melt={$input()}
            />
          {/each}
        </div> -->
      <!-- {:else} -->
      {#if expired}
        <div class="mt-4 flex flex-col items-center">
          <div class="rounded-lg px-4 py-3 text-sm font-semibold text-rose-500">
            <span>Verification code expired</span>
          </div>
        </div>
      {/if}
    {/if}

    <!-- <div class="pt-4 text-sm text-slate-400 dark:text-slate-500">
      <span>Verified by</span> <span class="font-semibold">Impierce Technologies B.V.</span>
    </div> -->
  </div>

  <!-- TODO: REFACTOR! -->
  <div class="absolute bottom-[64px] left-0 z-10 flex w-full flex-col gap-3 rounded-t-3xl bg-white p-6 dark:bg-dark">
    {#if expired}
      <Button label="Dismiss" variant="secondary" on:click={reset} />
    {/if}
    <Button
      label={expired ? 'Resend verification email' : 'Send verification email'}
      on:click={() => startVerificationSession()}
      {loading}
      disabled={awaitingConfirmation}
    />
  </div>
</div>
