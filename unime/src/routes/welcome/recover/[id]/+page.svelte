<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Button, PaddedIcon, TopNavBar } from '$lib/components';
  import { EyeClosedRegularIcon, EyeRegularIcon, VaultFillIcon } from '$lib/icons';

  let showPassword = false;
  let password: string;
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={() => history.back()} />
<div class="mt-8 grow p-4">
  <div class="flex w-full justify-center">
    <PaddedIcon icon={VaultFillIcon} />
  </div>
  <!-- <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}> -->
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {'Please enter your'} <span class="text-primary">{'password'}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {'Enter your password to unlock the backup'}
    </p>
  </div>

  <!-- Input -->
  <div class="flex w-full flex-col items-center">
    <div class="relative mb-4 mt-8 flex w-[240px]">
      <input
        type={showPassword ? 'text' : 'password'}
        class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
        placeholder={'Enter your password'}
        on:input={(e) => (password = e.target.value)}
      />
      <div class="absolute right-3 top-0 flex h-full items-center">
        <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
          {#if showPassword}
            <EyeRegularIcon class="text-slate-700 dark:text-grey" />
          {:else}
            <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
          {/if}
        </button>
      </div>
    </div>
    <div class="w-[240px]">
      <Button
        label={'Unlock backup'}
        on:click={() => {}}
        disabled={!password}
        on:click={() => goto('/welcome/recover/1/preview')}
      />
    </div>
  </div>
</div>
