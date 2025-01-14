<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { ListItemCard, TopNavBar } from '$lib/components';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import SettingsEntry from '$lib/components/SettingsEntry.svelte';
  import { CloudFillIcon, LockFillIcon, LockKeyFillIcon, LockSimpleFillIcon, VaultFillIcon } from '$lib/icons';

  const backups = [
    { id: 0, title: 'Backup from 2021-10-07, 14:21' },
    { id: 1, title: 'Backup from 2021-10-08, 09:32' },
  ];
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={() => history.back()} />
<div class="mt-8 grow p-4">
  <div class="flex w-full justify-center">
    <PaddedIcon icon={VaultFillIcon} />
  </div>
  <!-- <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}> -->
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {"We've found your"} <span class="text-primary">{'recovery backups'}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {'Choose a backup from below'}
    </p>
  </div>
  <!-- List -->
  <div class="space-y-3">
    {#each backups as { id, title }}
      <SettingsEntry
        icon={LockSimpleFillIcon}
        {title}
        hasCaretRight={true}
        on:click={() => goto(`/welcome/recover/${id}`)}
      />
    {/each}
    <!-- <ListItemCard title={'Backup 1'} description={'Backup from 2021-10-10, 16:09'} /> -->
    <!-- <SettingsEntry
      icon={LockSimpleFillIcon}
      title={'Backup from 2021-10-07, 14:21'}
      hasCaretRight={true}
      on:click={() => goto('/welcome/recover/0')}
    />
    <SettingsEntry icon={LockFillIcon} title={'Backup from 2021-10-08, 09:32'} hasCaretRight={true} />
    <SettingsEntry icon={LockKeyFillIcon} title={'Backup from 2021-10-10, 16:09'} hasCaretRight={true} /> -->
  </div>
</div>
