<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { checkPermissions, getDir, type PermissionState } from '@impierce/tauri-plugin-cloud-storage';
  import * as path from '@tauri-apps/api/path';
  import { BaseDirectory, exists, readDir, remove, stat, writeFile, type FileInfo } from '@tauri-apps/plugin-fs';
  import { info, warn } from '@tauri-apps/plugin-log';

  import { Button, SettingsEntry, Switch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CloudFillIcon, InfoRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  let enabled = true;

  let permissions: PermissionState | null; // TODO: handle "denied" and "prompt" states accordingly
  let fileInfo: FileInfo | undefined = undefined;

  let dirPath: string | null;
  let fileName: string = 'backup.txt';

  async function getFileInfo() {
    if (!dirPath) {
      return;
    }
    let filePath = await path.join(dirPath, fileName);
    fileInfo = await stat(filePath)
      .then((fileInfo) => {
        info(`location: "backup.txt", attributes: ${JSON.stringify(fileInfo)}`);
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
  }

  async function writeToStorage(data: Uint8Array | ReadableStream<Uint8Array>) {
    info(`${data.toString()}`);

    if (!dirPath) {
      return;
    }

    let filePath = await path.join(dirPath, fileName);

    await writeFile(filePath, data)
      .then(() => {
        info(`Successfully wrote value to cloud storage: ${data.toString()}`);
      })
      .catch((error) => {
        warn(`Error writing value to cloud storage: ${error}`);
      });

    // await writeData({ ...args, data: new TextEncoder().encode(value) })
    //   .then(() => {
    //     info(`Successfully wrote value to cloud storage: ${value}`);
    //   })
    //   .catch((error) => {
    //     warn(`Error writing value to cloud storage: ${error}`);
    //   });

    await getFileInfo();
  }

  async function deleteTheBackup() {
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

    // await deleteBackup({ fileUri: `${args.pathUri}/${args.fileName}` })
    //   .then((res) => {
    //     info(`Successfully deleted backup: ${res}`);
    //     enabled = false;
    //   })
    //   .catch((error) => {
    //     warn(`Error deleting backup: ${error}`);
    //   });
  }

  async function enable() {
    // await dispatch({ type: '[Backup] Enable' });
    await writeToStorage(new TextEncoder().encode('some_awesome_32_character_string'))
      .then(() => {
        info('Successfully wrote to storage');
        enabled = true;
      })
      .catch((error) => {
        warn(`Error writing to storage: ${error}`);
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

    await exists('backup.txt', { baseDir: BaseDirectory.AppLocalData }).then(async (res) => {
      if (res) {
        getFileInfo();
      }
    });
    // .then((fileAttributes) => {
    //   info(`location: "${args.pathUri}/${args.fileName}", attributes: ${JSON.stringify(fileAttributes)}`);
    //   return fileAttributes;
    // })
    // .catch((error) => {
    //   warn(`Error checking cloud backup exists: ${error}`);
    //   warn(`${JSON.stringify(error)}`);
    //   return undefined;
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
    <SettingsEntry icon={CloudFillIcon} title={'Automatic cloud backups'} hasCaretRight={false}>
      <Switch
        active={enabled}
        on:change={async () => {
          if (enabled) {
            await deleteTheBackup();
          } else {
            await enable();
          }
        }}
      />
    </SettingsEntry>
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
            <div class="text-xs font-medium text-slate-400">{fileInfo?.size} bytes</div>
          </div>
        {/if}
      </div>
      <Button label="Back up now" on:click={async () => await createBackup()} />
    {/if}
    <pre class="text-xs text-slate-400">permissions: {permissions}</pre>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
