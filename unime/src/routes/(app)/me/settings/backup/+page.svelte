<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import {
    checkPermissions,
    deleteBackup,
    exists,
    ping,
    writeBytes,
    type FileAttributes,
  } from '@impierce/tauri-plugin-cloud-storage';
  import { info, warn } from '@tauri-apps/plugin-log';

  import { Button, SettingsEntry, Switch, TopNavBar } from '$lib/components';
  import { CloudFillIcon, InfoRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  let enabled = true;

  let permissions: string | null = null;
  let cloud_status: FileAttributes | undefined = undefined;
  let message: string | null = null;

  const args = {
    fileUri: '/Users/daniel/Library/Application Support/com.impierce.unime/backup.txt', // TODO: move to backend
  };

  async function refresh() {
    cloud_status = await exists(args).catch((error) => {
      warn(`Error checking cloud backup exists: ${error}`);
      return undefined;
    });
  }

  async function writeToStorage(value: string) {
    info(`${value}`);
    message = await ping(new Date().toISOString())
      .then((res) => {
        info(`successful ping: ${res}`);
        return res;
      })
      .catch((error) => {
        warn(`Error pinging cloud storage: ${error}`);
        return null;
      });

    await writeBytes({ fileUri: args.fileUri, value })
      .then((res) => {
        info(`Successfully wrote value to cloud storage: ${value}, response: ${res.value}`);
      })
      .catch((error) => {
        warn(`Error writing value to cloud storage: ${error}`);
      });

    await refresh();
  }

  onMount(async () => {
    permissions = await checkPermissions()
      .then((permissions) => {
        info(`Permissions to use cloud storage: ${permissions}`);
        return permissions;
      })
      .catch((error) => {
        warn(`Error checking for permissions to use cloud storage: ${error}`);
        return null; // possibly return "denied"? or does that imply that the check has been successful, but was actively denied?
      });
    console.log(permissions);
    cloud_status = await exists(args).catch((error) => {
      warn(`Error checking cloud backup exists: ${error}`);
      warn(`${JSON.stringify(error)}`);
      return undefined;
    });
    console.log(cloud_status);
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
            await deleteBackup(args)
              .then((res) => {
                info(`Successfully deleted backup: ${res}`);
                enabled = false;
              })
              .catch((error) => {
                warn(`Error deleting backup: ${error}`);
              });
          } else {
            await writeToStorage('some_awesome_32_character_string')
              .then(() => {
                info('Successfully wrote to storage');
                enabled = true;
              })
              .catch((error) => {
                warn(`Error writing to storage: ${error}`);
              });
          }
        }}
      />
    </SettingsEntry>
    {#if enabled}
      <div class="rounded-xl bg-background-alt p-4">
        <div class="mb-2 text-sm font-semibold text-slate-500">{cloud_status?.provider}</div>
        {#if cloud_status?.modificationDate}
          <div class="text-xs font-medium text-slate-400">
            Latest backup on {formatDateTime(cloud_status.modificationDate, $state.profile_settings.locale)}
          </div>
        {/if}
        {#if $state.dev_mode !== 'Off'}
          <div class="mt-2 space-y-2">
            <div class="text-xs font-medium text-slate-400">Location: {args.fileUri}</div>
            <div class="text-xs font-medium text-slate-400">{cloud_status?.size} bytes</div>
          </div>
        {/if}
      </div>
      <Button label="Back up now" on:click={async () => await writeToStorage('foobar')} />
    {/if}
    <pre class="text-xs text-slate-400">permissions: {permissions}</pre>
    <pre class="text-xs text-slate-400">ping response: {message}</pre>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
