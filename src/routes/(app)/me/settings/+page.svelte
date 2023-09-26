<script lang="ts">
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';
  import { state } from '$src/stores';

  import Envelope from '~icons/ph/envelope-fill';
  import Gear from '~icons/ph/gear-fill';
  import Info from '~icons/ph/info-fill';
  import SignOut from '~icons/ph/sign-out-fill';
  import UserCircle from '~icons/ph/user-circle-fill';
  import Vault from '~icons/ph/vault-fill';
  import WarningCircle from '~icons/ph/warning-circle-fill';
</script>

<TopNavigation on:back={() => history.back()} title="Settings" />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5" in:fly={{ x: 32, opacity: 1 }}>
    <!-- Account -->
    <p class="text-[14px]/[22px] font-medium text-slate-500">{$LL.ACCOUNT()}</p>
    <SettingsEntry icon={UserCircle} title="My profile" on:click={() => goto('/me/settings/profile')} />
    <SettingsEntry icon={Gear} title="App settings" on:click={() => goto('/me/settings/app')} />
    <SettingsEntry icon={Vault} title="Backup and recovery" on:click={() => goto('/me/settings/backup')} todo />
    <SettingsEntry icon={SignOut} title="Log out" hasCaretRight={false} todo />

    <!-- Support -->
    <p class="text-[14px]/[22px] font-medium text-slate-500">{$LL.SUPPORT()}</p>
    <SettingsEntry icon={Info} title="About UniMe" on:click={() => goto('/me/settings/about')} />
    <SettingsEntry icon={Envelope} title="Send feedback" hasCaretRight={false} todo />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
