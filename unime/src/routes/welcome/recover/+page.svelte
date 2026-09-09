<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { TopNavBar } from '$lib/components';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { CaretRightBoldIcon, LockSimpleFillIcon, VaultFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  // The backend owns the backup store and returns opaque ids, newest first.
  $: backups = $state.backups ?? [];

  onMount(async () => {
    await dispatch({ type: '[Backup] List' });
  });
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={() => history.back()} />
<div class="mt-8 grow p-4">
  <div class="flex w-full justify-center">
    <PaddedIcon icon={VaultFillIcon} />
  </div>
  <!-- <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}> -->
  <div class="px-2 pb-8 pt-4">
    {#if backups.length === 0}
      <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
        <span class="text-primary">{'Nothing '}</span>{'to see here'}
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {'UniMe was not able to find any existing backups'}
      </p>
    {:else}
      <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
        {"We've found your"} <span class="text-primary">{'recovery backups'}</span>
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {'Choose a backup from below'}
      </p>
    {/if}
  </div>
  <!-- List -->
  <div class="space-y-3">
    {#each backups as backup (backup.id)}
      <!-- Design derived from <SettingsEntry> -->
      <button
        class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
        on:click={() => goto(`/welcome/recover/${backup.id}`)}
      >
        <LockSimpleFillIcon class="h-5 w-5 text-primary" />
        <div class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
          <p>{backup.name}</p>
          <p class="text-[12px]/[20px] font-normal text-slate-500 dark:text-slate-300">
            {formatDateTime(backup.modifiedAt, $state.profile_settings.locale)}
          </p>
        </div>
        <CaretRightBoldIcon class="h-4 w-4 text-slate-500" />
      </button>
    {/each}
  </div>
</div>
