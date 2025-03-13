<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { checkPermissions, getDir, type PermissionState } from '@impierce/tauri-plugin-cloud-storage';
  import * as path from '@tauri-apps/api/path';
  import { BaseDirectory, exists, readDir, remove, stat, type FileInfo } from '@tauri-apps/plugin-fs';
  import { info, warn } from '@tauri-apps/plugin-log';

  import { ActionSheet, Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CloudFillIcon, InfoRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  let enabled: boolean = false; // TODO: persist in app state user settings

  let permissions: PermissionState | null; // TODO: handle "denied" and "prompt" states accordingly
  let fileInfo: FileInfo | undefined = undefined;

  let openConfirmAction = false;

  let dirPath: string | null;
  let fileName: string = 'backup.dat';

  async function getFileInfo() {
    if (!dirPath) {
      return;
    }
    let filePath = await path.join(dirPath, fileName);
    fileInfo = await stat(filePath)
      .then((fileInfo) => {
        info(`location: ${filePath}, attributes: ${JSON.stringify(fileInfo)}`);
        enabled = true;
        return fileInfo;
        // return {
        //   provider: 'Local filesystem',
        //   size: fileInfo.size,
        //   modificationDate: fileInfo.mtime?.toISOString() ?? '',
        // };
      })
      .catch((error) => {
        warn(`Error checking cloud backup exists: ${error}`);
        warn(`${JSON.stringify(error)}`);
        return undefined;
      });
  }

  async function createBackup() {
    if (!dirPath) {
      Promise.reject('No directory path');
    }
    await dispatch({ type: '[Backup] Create', payload: { path: `${dirPath}/${fileName}`, password: 'sup3rSecr3t' } });
    await getFileInfo();
  }

  async function enable() {
    await createBackup();
  }

  async function disable() {
    if (!dirPath) {
      return;
    }

    let filePath = await path.join(dirPath, fileName);

    await remove(filePath)
      .then(() => {
        info(`Successfully deleted backup: ${fileName}`);
        enabled = false;
      })
      .catch((error) => {
        warn(`Error deleting backup: ${error}`);
      });
  }

  onMount(async () => {
    dirPath = await getDir()
      .then((dir) => {
        info(`Cloud storage directory: ${dir}`);
        return dir;
      })
      .catch(async (error) => {
        warn(`Error getting cloud storage directory: ${error}`);
        // TODO: is a fallback to dir AppLocalData a good idea?
        return path
          .appLocalDataDir()
          .then((dir) => {
            info(`Fallback to local app data directory: ${dir}`);
            return dir;
          })
          .catch((error) => {
            warn(`Error getting local app data directory: ${error}`);
            return null;
          });
        // return BaseDirectory.AppLocalData;
      });

    if (!dirPath) {
      return;
    }

    const files = await readDir(dirPath);

    files.map((file) => {
      info(`${file.name}`);
    });

    // info(`Files in app data directory: ${JSON.stringify(files)}`);

    const folderExists = await exists('backup.txt', { baseDir: BaseDirectory.AppLocalData });
    info(`Backup file exists: ${folderExists}`);

    permissions = await checkPermissions()
      .then((permissions) => {
        info(`Permissions to use cloud storage: ${permissions}`);
        return permissions;
      })
      .catch((error) => {
        warn(`Error checking for permissions to use cloud storage: ${error}`);
        return null;
      });

    await getFileInfo();

    // await exists('backup.txt', { baseDir: BaseDirectory.AppLocalData }).then(async (res) => {
    //   if (res) {
    //     getFileInfo();
    //   }
    // });
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
      initialChecked={enabled}
      onchange={async () => {
        if (enabled) {
          openConfirmAction = true;
        } else {
          await enable();
        }
      }}
    >
      {#snippet icon()}
        <CloudFillIcon class="size-5 text-primary"></CloudFillIcon>
      {/snippet}
      {'Automatic cloud backups'}
    </SettingsSwitch>

    {#if enabled}
      <div class="rounded-xl bg-background-alt p-4">
        <div class="mb-2 text-sm font-semibold text-slate-500">{'n/a'}</div>
        {#if fileInfo?.mtime}
          <div class="text-xs font-medium text-slate-400">
            Latest backup on {formatDateTime(fileInfo?.mtime.toISOString(), $state.profile_settings.locale)}
          </div>
        {/if}
        {#if $state.dev_mode !== 'Off'}
          <div class="mt-2 space-y-2">
            <div class="text-xs font-medium text-slate-400">Location: {dirPath}/{fileName}</div>
            <div class="text-xs font-medium text-slate-400">
              {fileInfo ? Math.round(fileInfo.size / 1_000) : '?'} kB
            </div>
          </div>
        {/if}
      </div>
      <Button label={$LL.SETTINGS.BACKUP_RECOVERY.BACKUP_NOW()} on:click={async () => await createBackup()} />
    {/if}
    <pre class="text-xs text-amber-500">permissions: {permissions}</pre>
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
            disable();
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
