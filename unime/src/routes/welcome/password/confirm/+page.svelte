<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import Button from '$lib/components/atoms/Button.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { onboarding_state } from '$lib/stores';

  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';
  import Smiley from '~icons/ph/smiley';
  import SmileySad from '~icons/ph/smiley-sad';

  // 3 states: true (match), false (mismatch), undefined (not checked yet).
  let passwordsMatch: boolean | undefined = undefined;

  let showPassword = false;
  let password = '';

  // Ref to input DOM element.
  let input_element: HTMLInputElement;

  onMount(() => {
    input_element.focus();
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.CONFIRM.NAVBAR_TITLE()} />

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.PASSWORD.CONFIRM.SUBTITLE()}
    </p>
  </div>
  <div class="relative">
    <!-- Dynamic type attribute requires one-way binding instead of two-way bind:value. -->
    <input
      value={password}
      bind:this={input_element}
      type={showPassword ? 'text' : 'password'}
      placeholder={$LL.ONBOARDING.PASSWORD.CONFIRM.INPUT_PLACEHOLDER()}
      on:blur={() => {
        if (password === $onboarding_state.password) {
          passwordsMatch = true;
        } else {
          passwordsMatch = false;
        }
      }}
      on:input={(e) => {
        password = e.currentTarget.value;
        if (password === $onboarding_state.password) {
          passwordsMatch = true;
        } else {
          // Suppress validation message.
          passwordsMatch = undefined;
        }
      }}
      class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button
        class="rounded-full p-2"
        on:click={() => {
          // Focus input element when toggling visibility.
          input_element.focus();
          return (showPassword = !showPassword);
        }}
      >
        {#if showPassword}
          <Eye class="text-slate-700 dark:text-grey" />
        {:else}
          <EyeClosed class="text-slate-700 dark:text-grey" />
        {/if}
      </button>
    </div>
  </div>
  {#if passwordsMatch !== undefined}
    <div class="mt-8 flex items-center justify-center">
      {#if passwordsMatch}
        <Smiley class="mr-[10px] h-5 w-5 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.MATCH()}</p>
      {:else}
        <SmileySad class="mr-[10px] h-5 w-5 text-rose-500" />
        <p class="text-[13px]/[24px] font-medium text-rose-500">{$LL.ONBOARDING.PASSWORD.CONFIRM.NO_MATCH()}</p>
      {/if}
    </div>
  {/if}
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password/completed')} disabled={!passwordsMatch} />
</div>
