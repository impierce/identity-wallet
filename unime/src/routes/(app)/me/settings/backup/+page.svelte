<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { ActionSheet, Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CloudArrowUpFillIcon, InfoRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  let openConfirmAction = false;

  // The backend owns where backups live and hands back opaque ids, so this screen
  // reads the listing out of app state rather than touching the filesystem.
  $: backups = $state.backups ?? [];
  $: latest = backups[0]; // the backend returns them newest first
  $: enabled = backups.length > 0;

  async function createBackup() {
    // TODO: prompt for a password instead of the development default.
    await dispatch({ type: '[Backup] Create', payload: { password: 'sup3rSecr3t' } });
  }

  async function removeAllBackups() {
    for (const backup of backups) {
      await dispatch({ type: '[Backup] Delete', payload: { id: backup.id } });
    }
  }

  onMount(async () => {
    await dispatch({ type: '[Backup] List' });
  });
</script>

<TopNavBar on:back={() => history.back()} title="Backup and recovery" />

<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
      <span class="mr-4 h-6 w-6">
        <InfoRegularIcon class="h-6 w-6 text-primary" />
      </span>
      <div class="flex flex-col">
        <!-- <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Developer info</p> -->
        <ul class="ml-3 list-disc text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          <li>All your data is automatically encrypted and stored in the cloud of your choice.</li>
          <li>Your data can be restored in case you don't have access to your device anymore.</li>
          <li>We do not have access to your backups.</li>
          <!-- <li>All edits can be reset to the default trust list.</li> -->
        </ul>
      </div>
    </div>

    <!-- <div class="bg-background-alt px-4 text-sm font-medium text-slate-500">
      All your data is automatically encrypted and stored in iCloud, if you choose to enable it. We do not have access
      to your backups.
    </div> -->

    <SettingsSwitch initialChecked={enabled} onchange={async () => {
      if (enabled) {
        openConfirmAction = true;
      } else {
        await createBackup();
      }
    }}>
      {#snippet icon()}
        <CloudArrowUpFillIcon class="h-5 w-5 text-primary"></CloudArrowUpFillIcon>
      {/snippet}
      {$LL.SETTINGS.APP.DEVELOPER_MODE.TITLE()}
    </SettingsSwitch>

    {#if enabled}
      <div class="rounded-xl bg-background-alt p-4">
        <div class="mb-2 text-sm font-semibold text-slate-500">
          {backups.length}
          {backups.length === 1 ? 'backup' : 'backups'}
        </div>
        {#if latest}
          <div class="text-xs font-medium text-slate-400">
            Latest backup on {formatDateTime(latest.modifiedAt, $state.profile_settings.locale)}
          </div>
        {/if}
        {#if $state.dev_mode !== 'Off' && latest}
          <div class="mt-2 space-y-2">
            <div class="text-xs font-medium text-slate-400">Name: {latest.name}</div>
            <div class="text-xs font-medium text-slate-400">{Math.round(Number(latest.size) / 1_000)} kB</div>
          </div>
        {/if}
      </div>
      <Button label={$LL.SETTINGS.BACKUP_RECOVERY.BACKUP_NOW()} on:click={async () => await createBackup()} />
    {/if}
  </div>

  <!-- Confirm disable backups -->
  <div class="mt-8">
    <ActionSheet
      titleText={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DISABLE.TITLE()}
      descriptionText={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DISABLE.DESCRIPTION()}
      isOpen={openConfirmAction}
    >
      <!-- <button
        slot="trigger"
        let:trigger
        use:melt={trigger}
        class="rounded-xl px-4 py-2 text-[13px]/[24px] font-medium text-slate-400 opacity-50 active:bg-grey dark:active:bg-dark"
        >{$LL.LOCK_SCREEN.FORGOT_PASSWORD()}</button
      > -->

      <!-- TODO: bug: after resetting (closing the drawer, main UI is not clickable anymore) -->
      <div slot="content" class="w-full pb-[10px] pt-[20px]">
        <button
          class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
          on:click={() => {
            removeAllBackups();
            openConfirmAction = false;
          }}>{$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DISABLE.CONFIRM()}</button
        >
      </div>

      <Button
        variant="secondary"
        slot="close"
        let:close
        trigger={close}
        label={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DISABLE.CANCEL()}
      />
    </ActionSheet>
    <!-- TODO Button with `KeyboardFillIcon` and `Your DID`. -->
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
