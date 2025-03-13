<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { getDir } from '@impierce/tauri-plugin-cloud-storage';
  import * as path from '@tauri-apps/api/path';
  import { readDir, stat, type FileInfo } from '@tauri-apps/plugin-fs';
  import { info, warn } from '@tauri-apps/plugin-log';

  import { ListItemCard, TopNavBar } from '$lib/components';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import {
    CaretRightBoldIcon,
    CloudFillIcon,
    LockFillIcon,
    LockKeyFillIcon,
    LockSimpleFillIcon,
    VaultFillIcon,
  } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  interface BackupFile {
    id: string;
    title: string;
    lastModified: Date | null;
  }

  let backups: BackupFile[] = [];

  onMount(async () => {
    let dirPath;
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

    files
      .filter((file) => file.name.endsWith('.dat'))
      .map(async (file, index) => {
        info(`Found backup file: ${file.name}`);
        let filePath = await path.join(dirPath, file.name);
        let lastModified = await stat(filePath).then((fileInfo: FileInfo) => fileInfo.mtime);
        backups.push({ id: index.toString(), title: file.name, lastModified });
        backups = backups;
      });
  });
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
    {#each backups as { id, title, lastModified }}
      <!-- Design derived from <SettingsEntry> -->
      <button
        class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
        on:click={() => goto(`/welcome/recover/${id}`)}
      >
        <svelte:component this={LockSimpleFillIcon} class="h-5 w-5 text-primary" />
        <div class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
          <p>
            {title}
          </p>
          <p class="text-[12px]/[20px] font-normal text-slate-500 dark:text-slate-300">
            {#if lastModified}
              {formatDateTime(lastModified.toISOString(), $state.profile_settings.locale)}
            {/if}
          </p>
        </div>
        <svelte:component this={CaretRightBoldIcon} class="h-4 w-4 text-slate-500" />
      </button>
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
