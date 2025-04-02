<script lang="ts">
  import { goto } from '$app/navigation';
  import { PinInput } from 'melt/builders';
  import { cubicOut } from 'svelte/easing';
  import { Tween } from 'svelte/motion';

  import { TopNavBar } from '$lib/components';
  import CircularProgressBar from '$lib/components/CircularProgressBar.svelte';
  import { state as appState } from '$lib/stores';

  let progressValue = new Tween(0, {
    duration: 400,
    easing: cubicOut,
  });

  const pinInput = new PinInput({
    type: 'numeric',
    maxLength: 4,
    placeholder: '', // '•',
    onValueChange(value) {
      // onComplete(value) does not seem to trigger on "melt 0.17.0", that's why we use onValueChange and count the length
      if (value.length === 4) {
        // redeemCode(value);
      }
    },
  });
</script>

<TopNavBar on:back={() => goto('/me/add')} title={'Verified email'} class="sticky top-0 z-10" />

<CircularProgressBar
  max={$appState.verified_data.email_verification?.validation_expiration_in_secs ?? 0}
  min={0}
  value={progressValue.current}
  gaugePrimaryColor={`${progressValue.target > 0 ? 'rgb(var(--color-brand))' : 'oklch(0.586 0.253 17.585)'} `}
  gaugeSecondaryColor="rgba(0, 50, 100, 0.1)"
/>

<div {...pinInput.root} class="mt-8 flex items-center justify-center gap-2 font-mono">
  {#each pinInput.inputs as input}
    <input
      {...input}
      class="size-12 rounded-xl border border-slate-300 bg-background-alt text-center text-2xl font-semibold text-text-alt outline-none focus:border-primary disabled:cursor-not-allowed dark:border-slate-500"
      disabled={false}
    />
  {/each}
</div>

<p>Resend code</p>
