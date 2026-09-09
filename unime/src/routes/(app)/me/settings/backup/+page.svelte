<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { writable } from 'svelte/store';

  import { platform } from '@tauri-apps/plugin-os';

  import { ActionSheet, Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch, tryDispatch } from '$lib/dispatcher';
  import {
    CloudArrowUpFillIcon,
    EyeClosedRegularIcon,
    EyeRegularIcon,
    HourglassRegularIcon,
    InfoRegularIcon,
  } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  const openConfirmDelete = writable(false);
  const openPasswordPrompt = writable(false);

  // The backend owns where backups live and hands back opaque ids, so this screen
  // reads the listing out of app state rather than touching the filesystem.
  $: backups = $state.backups ?? [];
  $: latest = backups[0]; // the backend returns them newest first
  // A stored preference, not "a backup happens to exist". The two came apart once
  // backups became automatic: switching off has to stop future backups without
  // touching the ones already made.
  $: enabled = $state.profile_settings.backup_enabled;

  // The Google Drive provider is not implemented yet, so the plugin rejects every
  // operation on Android. Checked here rather than through `getStatus()` because
  // the plugin is only registered on mobile and this screen is opened on desktop
  // during development. Swap this for the plugin's own status once Drive lands.
  const cloudUnavailable = platform() === 'android';

  let password = '';
  let showPassword = false;
  let creating = false;
  let failed = false;

  function askForPassword() {
    password = '';
    failed = false;
    openPasswordPrompt.set(true);
  }

  // Backups are sealed with the profile password. The backend verifies it against
  // the Stronghold snapshot before sealing, so a typo is refused here rather than
  // producing an archive that cannot be opened later.
  async function createBackup() {
    creating = true;
    failed = false;
    try {
      await tryDispatch({ type: '[Backup] Create', payload: { password } });
      password = '';
      openPasswordPrompt.set(false);
    } catch {
      failed = true;
    } finally {
      creating = false;
    }
  }

  // Enabling seals the first backup straight away, using the password the user
  // typed to unlock the profile, so the switch never leaves them with backups
  // "on" and nothing to restore.
  async function setEnabled(enable: boolean) {
    await dispatch({ type: '[Backup] Enable', payload: { enable } });
  }

  async function deleteAllBackups() {
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

    <SettingsSwitch
      checked={enabled}
      disabled={cloudUnavailable}
      onCheckedChange={({ next }) => {
        setEnabled(next);
        return next;
      }}
    >
      {#snippet icon()}
        <CloudArrowUpFillIcon class="h-5 w-5 text-primary"></CloudArrowUpFillIcon>
      {/snippet}
      {$LL.SETTINGS.BACKUP_RECOVERY.SETTINGS_ENTRY()}
    </SettingsSwitch>

    <p class="px-1 text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SETTINGS.BACKUP_RECOVERY.AUTOMATIC_EXPLANATION()}
    </p>
    <p class="px-1 text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SETTINGS.BACKUP_RECOVERY.RETENTION_EXPLANATION()}
    </p>

    {#if cloudUnavailable}
      <div class="flex w-full items-start rounded-lg bg-white px-4 py-4 dark:bg-dark">
        <span class="mr-4 h-6 w-6 shrink-0">
          <HourglassRegularIcon class="h-6 w-6 text-primary" />
        </span>
        <div class="flex flex-col space-y-1">
          <p class="text-[13px]/[20px] font-semibold text-slate-800 dark:text-grey">
            {$LL.SETTINGS.BACKUP_RECOVERY.ANDROID_NOTICE.TITLE()}
          </p>
          <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
            {$LL.SETTINGS.BACKUP_RECOVERY.ANDROID_NOTICE.DESCRIPTION()}
          </p>
        </div>
      </div>
    {/if}

    <!-- Independent of the switch: the stored backups are the same ones whether
         or not new backups are being taken automatically. -->
    {#if backups.length > 0}
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
    {/if}

    <Button label={$LL.SETTINGS.BACKUP_RECOVERY.BACKUP_NOW()} on:click={askForPassword} />

    {#if backups.length > 0}
      <Button label={'Restore from a backup'} variant="secondary" on:click={() => goto('/welcome/recover')} />
      <Button
        label={$LL.SETTINGS.BACKUP_RECOVERY.DELETE_ALL()}
        variant="secondary"
        on:click={() => openConfirmDelete.set(true)}
      />
    {/if}
  </div>

  <!-- Ask for the profile password before sealing a backup -->
  <ActionSheet
    titleText={'Confirm your password'}
    descriptionText={'Backups are encrypted with your profile password. You will need it to restore.'}
    open={openPasswordPrompt}
  >
    <div slot="content" class="w-full space-y-3 pt-[20px] pb-[10px]">
      <div class="relative flex w-full">
        <input
          type={showPassword ? 'text' : 'password'}
          class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
          placeholder={'Enter your password'}
          bind:value={password}
          on:keydown={(e) => {
            if (e.key === 'Enter' && password && !creating) createBackup();
          }}
        />
        <div class="absolute top-0 right-3 flex h-full items-center">
          <button
            type="button"
            class="rounded-full p-2"
            on:click={() => (showPassword = !showPassword)}
            aria-label={showPassword ? 'Hide password' : 'Show password'}
          >
            {#if showPassword}
              <EyeRegularIcon class="text-slate-700 dark:text-grey" />
            {:else}
              <EyeClosedRegularIcon class="text-slate-700 dark:text-grey" />
            {/if}
          </button>
        </div>
      </div>

      {#if failed}
        <p class="text-center text-[13px]/[20px] font-medium text-rose-500" role="alert">
          {"That password doesn't match your profile password."}
        </p>
      {/if}

      <Button
        label={creating ? 'Creating backup…' : 'Create backup'}
        disabled={!password || creating}
        on:click={createBackup}
      />
    </div>

    <Button variant="secondary" slot="close" let:close trigger={close} label={'Cancel'} />
  </ActionSheet>

  <!-- Confirm disable backups -->
  <div class="mt-8">
    <ActionSheet
      titleText={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DELETE.TITLE()}
      descriptionText={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DELETE.DESCRIPTION()}
      open={openConfirmDelete}
    >
      <!-- <button
        slot="trigger"
        let:trigger
        use:melt={trigger}
        class="rounded-xl px-4 py-2 text-[13px]/[24px] font-medium text-slate-400 opacity-50 active:bg-grey dark:active:bg-dark"
        >{$LL.LOCK_SCREEN.FORGOT_PASSWORD()}</button
      > -->

      <!-- TODO: bug: after resetting (closing the drawer, main UI is not clickable anymore) -->
      <div slot="content" class="w-full pt-[20px] pb-[10px]">
        <button
          class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
          on:click={() => {
            deleteAllBackups();
            openConfirmDelete.set(false);
          }}>{$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DELETE.CONFIRM()}</button
        >
      </div>

      <Button
        variant="secondary"
        slot="close"
        let:close
        trigger={close}
        label={$LL.SETTINGS.BACKUP_RECOVERY.CONFIRM_DELETE.CANCEL()}
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
