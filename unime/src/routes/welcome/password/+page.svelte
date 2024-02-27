<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { onboarding_state } from '$src/stores';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Circle from '~icons/ph/circle';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';

  import { checkPasswordPolicy, passwordPolicy } from './utils';

  let passwordPolicyViolations: string[] = checkPasswordPolicy($onboarding_state.password ?? '');
  let showPassword = false;
  let keyboardView = false;

  function setFocus() {
    keyboardView = true;
  }

  function unsetFocus() {
    // Small delay to keep button available.
    setTimeout(() => keyboardView = false, 200);
  }
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.NAVBAR_TITLE()} />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4 {keyboardView ? 'shrink-height' : 'expand-height'}">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.PASSWORD.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.PASSWORD.SUBTITLE()}
    </p>
    <!-- <div class="mt-[70px] flex w-full items-center justify-center" /> -->
  </div>

  <div class="relative">
    <input
      type={showPassword ? 'text' : 'password'}
      class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
      placeholder={$LL.ONBOARDING.PASSWORD.INPUT_PLACEHOLDER()}
      value={$onboarding_state.password ?? ''}
      on:focus={setFocus}
      on:blur={unsetFocus}
      on:input={(e) => {
        passwordPolicyViolations = checkPasswordPolicy(e.target.value);
        if (passwordPolicyViolations.length === 0) {
          onboarding_state.set({ ...$onboarding_state, password: e.target.value });
        }
      }}
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
  <div class="mt-6">
    <div class="mt-3 rounded-2xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark">
      <p class="mb-[10px] text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SETTINGS.PASSWORD.POLICY.TITLE()}
      </p>
      <div class="flex flex-wrap">
        {#each passwordPolicy as rule}
          {#if passwordPolicyViolations.indexOf(rule.name) > -1}
            <!-- not fulfilled -->
            <div class="mb-2 mr-2 flex items-center space-x-1 rounded-full bg-gray-100 px-2 py-1 dark:bg-navy">
              <Circle class="h-4 w-4 text-primary" />
              <p class="text-[12px]/[20px] font-medium text-teal">
                {rule.count}
                {rule.name}
              </p>
            </div>
          {:else}
            <!-- fulfilled -->
            <div class="mb-2 mr-2 flex items-center space-x-1 rounded-full bg-ex-blue-2 px-2 py-1 dark:bg-primary">
              <CheckCircle class="h-4 w-4 text-primary dark:text-navy" />
              <p class="text-[12px]/[20px] font-medium text-teal dark:text-dark">
                {rule.count}
                {rule.name}
              </p>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label={$LL.CONTINUE()}
    on:click={() => goto('/welcome/password/confirm')}
    disabled={passwordPolicyViolations.length > 0}
  />
</div>

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
