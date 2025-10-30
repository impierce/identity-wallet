<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button, TopNavBar } from '$lib/components';
  import { CheckCircleFillIcon, CircleRegularIcon, EyeClosedRegularIcon, EyeRegularIcon } from '$lib/icons';
  import { onboarding_state } from '$lib/stores';

  import { checkPasswordPolicy, passwordPolicy } from './utils';

  let passwordPolicyViolations: string[] = checkPasswordPolicy($onboarding_state.password ?? '');
  let showPassword = false;

  // Ref to input DOM element.
  let inputElement: HTMLInputElement;

  onMount(() => {
    inputElement.focus();
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.PASSWORD.NAVBAR_TITLE()} />

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="dark:text-grey pb-8 text-3xl font-semibold text-slate-700">
      {$LL.ONBOARDING.PASSWORD.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.PASSWORD.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.PASSWORD.SUBTITLE()}
    </p>
  </div>
  <div class="relative">
    <input
      bind:this={inputElement}
      type={showPassword ? 'text' : 'password'}
      class="dark:bg-dark h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:text-slate-300"
      placeholder={$LL.ONBOARDING.PASSWORD.INPUT_PLACEHOLDER()}
      value={$onboarding_state.password ?? ''}
      on:input={(e) => {
        // Here e.currentTarget is the same as e.target and fully typed.
        passwordPolicyViolations = checkPasswordPolicy(e.currentTarget.value);
        if (passwordPolicyViolations.length === 0) {
          onboarding_state.set({ ...$onboarding_state, password: e.currentTarget.value });
        }
      }}
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
          <EyeRegularIcon class="dark:text-grey text-slate-700" />
        {:else}
          <EyeClosedRegularIcon class="dark:text-grey text-slate-700" />
        {/if}
      </button>
    </div>
  </div>
  <div class="mt-6">
    <div class="dark:bg-dark mt-3 rounded-2xl border border-slate-200 bg-white p-4 dark:border-slate-600">
      <p class="mb-[10px] text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SETTINGS.PASSWORD.POLICY.TITLE()}
      </p>
      <div class="flex flex-wrap">
        {#each passwordPolicy as rule}
          {#if passwordPolicyViolations.indexOf(rule.name) > -1}
            <!-- not fulfilled -->
            <div class="dark:bg-navy mb-2 mr-2 flex items-center space-x-1 rounded-full bg-gray-100 px-2 py-1">
              <CircleRegularIcon class="text-primary h-4 w-4" />
              <p class="text-secondary text-[12px]/[20px] font-medium">
                {rule.count}
                {rule.name}
              </p>
            </div>
          {:else}
            <!-- fulfilled -->
            <div class="bg-ex-blue-2 dark:bg-primary mb-2 mr-2 flex items-center space-x-1 rounded-full px-2 py-1">
              <CheckCircleFillIcon class="text-primary dark:text-navy h-4 w-4" />
              <p class="text-secondary dark:text-dark text-[12px]/[20px] font-medium">
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

<div class="dark:bg-dark rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label={$LL.CONTINUE()}
    on:click={() => goto('/welcome/password/confirm')}
    disabled={passwordPolicyViolations.length > 0}
  />
</div>
