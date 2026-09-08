<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Button, Image, ListItemCard, PaddedIcon, TopNavBar } from '$lib/components';
  import { tryDispatch } from '$lib/dispatcher';
  import { VaultFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateTime } from '$lib/utils';

  import { recovery } from '../../store';

  // Produced by `[Backup] Preview` on the previous screen. Nothing has been
  // written to disk yet.
  $: preview = $state.backup_preview;
  $: backup = ($state.backups ?? []).find((candidate) => candidate.id === preview?.id);

  let restoring = false;
  let failed = false;

  function plural(count: number, noun: string) {
    return `${count} ${count === 1 ? noun : `${noun}s`}`;
  }

  async function restore() {
    if (!$recovery) return;
    restoring = true;
    failed = false;
    try {
      await tryDispatch({
        type: '[Backup] Restore',
        payload: { id: $recovery.id, password: $recovery.password },
      });
      recovery.set(null);
      await goto('/me');
    } catch {
      failed = true;
    } finally {
      restoring = false;
    }
  }

  function goBack() {
    recovery.set(null);
    goto('/welcome/recover');
  }
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={goBack} />
<div class="mt-8 flex grow flex-col">
  <div class="grow p-4">
    <div class="flex w-full justify-center">
      <PaddedIcon icon={VaultFillIcon} />
    </div>
    <div class="px-2 pb-8 pt-4">
      <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
        {"Here's a"} <span class="text-primary">{'preview'}</span>
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {"You'll recover the following data"}
      </p>
    </div>

    {#if preview}
      <div class="grow space-y-2">
        <ListItemCard
          id={''}
          title={'User profile'}
          description={preview.profileName ?? 'No profile name'}
        />
        <ListItemCard id={''} title={'Credentials'} description={plural(preview.credentials, 'item')}>
          <div
            slot="image"
            class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
          >
            <Image id={'credentials'} iconFallback={'CertificateLight'} imgClass="p-1" />
          </div>
        </ListItemCard>
        <ListItemCard id={''} title={'Connections'} description={plural(preview.connections, 'item')}>
          <div
            slot="image"
            class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
          >
            <Image id={'connections'} iconFallback={'BankLight'} imgClass="p-1" />
          </div>
        </ListItemCard>
        <ListItemCard id={''} title={'Images'} description={plural(preview.assets, 'file')}>
          <div
            slot="image"
            class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-silver p-1 dark:bg-navy"
          >
            <Image id={'images'} iconFallback={'IdentificationBadgeLight'} imgClass="p-1" />
          </div>
        </ListItemCard>
      </div>

      <div class="px-2 pt-6 text-[12px]/[20px] font-medium text-slate-400">
        {#if backup}
          <p>Created {formatDateTime(backup.modifiedAt, $state.profile_settings.locale)}</p>
        {/if}
        <p>Written by UniMe {preview.appVersion}</p>
        <p class="pt-2 text-rose-500">Restoring replaces everything currently in this app.</p>
      </div>
    {:else}
      <p class="px-2 text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {'This backup is no longer unlocked. Go back and enter your password again.'}
      </p>
    {/if}

    {#if failed}
      <p class="px-2 pt-4 text-[13px]/[20px] font-medium text-rose-500" role="alert">
        {"We couldn't restore this backup. Nothing has been changed."}
      </p>
    {/if}
  </div>

  <!-- Bottom actions -->
  <div class="z-10 space-y-3 rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button
      label={restoring ? 'Restoring…' : 'Restore'}
      disabled={!preview || !$recovery || restoring}
      on:click={restore}
    />
    <Button label={'Go back'} variant="secondary" disabled={restoring} on:click={goBack} />
  </div>
</div>
