<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { onboarding_state } from '$src/stores';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Circle from '~icons/ph/circle';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';

  import { checkPasswordPolicy, passwordPolicy } from './utils';

  // TODO: remove
  const initialPasswordValue = 'sup3rSecr3t';
  onboarding_state.set({ ...$onboarding_state, password: initialPasswordValue });

  let passwordPolicyViolations: string[] = checkPasswordPolicy(initialPasswordValue);
  let showPassword = false;
</script>

<TopNavigation on:back={() => history.back()} title="Password" />
<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-700 dark:text-grey">
      Set your new <span class="text-primary">password</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      You need to create a strong password to secure your backup.
    </p>
    <!-- <div class="mt-[70px] flex w-full items-center justify-center" /> -->
  </div>
  <div class="relative">
    <input
      type={showPassword ? 'text' : 'password'}
      class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
      placeholder="Enter a password"
      value={initialPasswordValue}
      on:input={(e) => {
        passwordPolicyViolations = checkPasswordPolicy(e.target.value);
        if (passwordPolicyViolations.length === 0) {
          console.log('Setting password');
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
    <div
      class="mt-3 rounded-2xl border border-slate-200 bg-white p-4 dark:border-slate-600 dark:bg-dark"
    >
      <p class="mb-[10px] text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        Your password must contain
      </p>
      <div class="flex flex-wrap">
        {#each passwordPolicy as rule}
          {#if passwordPolicyViolations.indexOf(rule.name) > -1}
            <!-- not fulfilled -->
            <div
              class="mb-2 mr-2 flex items-center space-x-1 rounded-full bg-gray-100 px-2 py-1 dark:bg-navy"
            >
              <Circle class="h-4 w-4 text-primary" />
              <p class="text-[12px]/[20px] font-medium text-teal">
                {rule.count}
                {rule.name}
              </p>
            </div>
          {:else}
            <!-- fulfilled -->
            <div class="mb-2 mr-2 flex items-center space-x-1 rounded-full bg-ex-blue-2 px-2 py-1">
              <CheckCircle class="h-4 w-4 text-primary" />
              <p class="text-[12px]/[20px] font-medium text-teal">
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

<div
  class="rounded-t-3xl bg-white p-6 dark:bg-dark"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
  <Button
    label="Continue"
    on:click={() => goto('/welcome/password/confirm')}
    disabled={passwordPolicyViolations.length > 0}
  />
</div>
