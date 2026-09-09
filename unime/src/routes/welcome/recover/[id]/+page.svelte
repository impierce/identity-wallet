<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import LL from '$i18n/i18n-svelte';

  import { Button, PaddedIcon, TopNavBar } from '$lib/components';
  import { tryDispatch } from '$lib/dispatcher';
  import { EyeClosedRegularIcon, EyeRegularIcon, VaultFillIcon } from '$lib/icons';

  import { recovery } from '../store';

  let showPassword = false;
  let password = '';
  let unlocking = false;
  let failed = false;

  // Route params are typed as possibly absent; this route cannot match without one.
  $: id = page.params.id ?? '';

  // Backups are sealed with the profile password, so this is the same secret the
  // user signs in with — not a separate one.
  async function unlock() {
    unlocking = true;
    failed = false;
    try {
      // Decrypts and summarises the archive; nothing on disk changes until the
      // user confirms on the next screen.
      await tryDispatch({ type: '[Backup] Preview', payload: { id, password } });
      recovery.set({ id, password });
      await goto(`/welcome/recover/${id}/preview`);
    } catch {
      // A wrong password and a damaged file are indistinguishable by design, so
      // the message covers both without guessing.
      failed = true;
    } finally {
      unlocking = false;
    }
  }
</script>

<TopNavBar title={$LL.ONBOARDING.WELCOME.RECOVER_PROFILE()} on:back={() => history.back()} />
<div class="mt-8 grow p-4">
  <div class="flex w-full justify-center">
    <PaddedIcon icon={VaultFillIcon} />
  </div>
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
        bind:value={password}
        on:keydown={(e) => {
          if (e.key === 'Enter' && password && !unlocking) unlock();
        }}
      />
      <div class="absolute right-3 top-0 flex h-full items-center">
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
      <p class="mb-4 w-[240px] text-center text-[13px]/[20px] font-medium text-rose-500" role="alert">
        {"We couldn't unlock this backup. Check your password and try again."}
      </p>
    {/if}

    <div class="w-[240px]">
      <Button label={unlocking ? 'Unlocking…' : 'Unlock backup'} disabled={!password || unlocking} on:click={unlock} />
    </div>
  </div>
</div>
