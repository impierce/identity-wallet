<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { onboarding_state } from '$src/stores';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Circle from '~icons/ph/circle';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';
  import Smiley from '~icons/ph/smiley';
  import SmileySad from '~icons/ph/smiley-sad';

  let passwordsEqual: boolean | undefined;
  let showPassword = false;
  let keyboardView = false;

  function setFocus() {
    keyboardView = true;
  }

  function unsetFocus() {
    // Small delay to keep button available.
    setTimeout(() => (keyboardView = false), 300);
  }

  function comparePasswords(e: Event) {
    const input = e.target as HTMLInputElement;
    console.log(input.value, $onboarding_state.password);
    passwordsEqual = input.value === $onboarding_state.password;
  }
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.CONFIRM.NAVBAR_TITLE()} />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4 {keyboardView ? 'shrink-height' : 'expand-height'}">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.SUBTITLE()}
    </p>
    <!-- <div class="mt-[70px] flex w-full items-center justify-center" /> -->
  </div>
  <div class="relative">
    <input
      type={showPassword ? 'text' : 'password'}
      class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
      placeholder={$LL.ONBOARDING.PASSWORD.CONFIRM.INPUT_PLACEHOLDER()}
      on:focus={setFocus}
      on:blur={unsetFocus}
      on:input={comparePasswords}
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
        {#if showPassword}
          <Eye class="text-slate-700 dark:text-grey" />
        {:else}
          <EyeClosed class="text-slate-700 dark:text-grey" />
        {/if}
      </button>
    </div>
  </div>
  {#if passwordsEqual !== undefined}
    <div class="mt-8 flex items-center justify-center">
      {#if passwordsEqual}
        <Smiley class="mr-[10px] h-5 w-5 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.MATCH()}</p>
      {:else}
        <SmileySad class="mr-[10px] h-5 w-5 text-rose-500" />
        <p class="text-[13px]/[24px] font-medium text-rose-500">{$LL.ONBOARDING.PASSWORD.CONFIRM.NO_MATCH()}</p>
      {/if}
    </div>
  {/if}

  {#if keyboardView}
    <div class="mt-8" transition:fade={{ delay: 200 }}>
      <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password/completed')} disabled={!passwordsEqual} />
    </div>
  {/if}
</div>

{#if !keyboardView}
  <div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
    <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password/completed')} disabled={!passwordsEqual} />
  </div>
{/if}

<style>
  .expand-height {
    max-height: unset;
    transition:
      padding 0.5s ease-in,
      max-height 0.5s ease-in;
  }

  .shrink-height {
    max-height: 0;
    padding: 0;
    overflow: hidden;
    transition:
      padding 0.5s ease-out,
      max-height 0.5s ease-out;
  }
</style>
