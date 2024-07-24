<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button, TopNavBar } from '$lib/components';
  import { EyeClosedRegularIcon, EyeRegularIcon, SmileyRegularIcon, SmileySadRegularIcon } from '$lib/icons';
  import { onboarding_state } from '$lib/stores';

  // 3 states: true (match), false (mismatch), undefined (not checked yet).
  let passwordsMatch: boolean | undefined = undefined;

  let showPassword = false;
  let value = '';

  // Ref to input DOM element.
  let inputElement: HTMLInputElement;

  // Tracks whether the user has interacted with the input element. This allows to suppress the validation message only on first try.
  let touched = false;

  onMount(() => {
    inputElement.focus();
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
      {value}
      bind:this={inputElement}
      type={showPassword ? 'text' : 'password'}
      placeholder={$LL.ONBOARDING.PASSWORD.CONFIRM.INPUT_PLACEHOLDER()}
      on:blur={() => {
        if (value.length > 0) {
          if (value === $onboarding_state.password) {
            passwordsMatch = true;
          } else {
            passwordsMatch = false;
          }
          // Mark the input element as "touched" after losing focus for the first time.
          touched = true;
        }
      }}
      on:input={(e) => {
        value = e.currentTarget.value;
        if (value === $onboarding_state.password) {
          passwordsMatch = true;
        } else {
          // Suppress validation message (only on first try).
          if (touched) {
            passwordsMatch = false;
          } else {
            passwordsMatch = undefined;
          }
        }
      }}
      class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button
        class="rounded-full p-2"
        on:click={() => {
          // Focus input element when toggling visibility.
          inputElement.focus();
          return (showPassword = !showPassword);
        }}
      >
        {#if showPassword}
          <EyeRegularIcon class="text-slate-700 dark:text-grey" />
        {:else}
          <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
        {/if}
      </button>
    </div>
  </div>
  {#if passwordsMatch !== undefined}
    <div class="mt-8 flex items-center justify-center">
      {#if passwordsMatch}
        <SmileyRegularIcon class="mr-[10px] h-5 w-5 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-primary">{$LL.ONBOARDING.PASSWORD.CONFIRM.MATCH()}</p>
      {:else}
        <SmileySadRegularIcon class="mr-[10px] h-5 w-5 text-rose-500" />
        <p class="text-[13px]/[24px] font-medium text-rose-500">{$LL.ONBOARDING.PASSWORD.CONFIRM.NO_MATCH()}</p>
      {/if}
    </div>
  {/if}
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password/completed')} disabled={!passwordsMatch} />
</div>
