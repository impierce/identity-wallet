<script lang="ts">
  import { goto } from '$app/navigation';
  import { Eye, Heart } from 'svelte-heros-v2';
  import { fly } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
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
<div class="flex min-h-full flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5" in:fly={{ x: 32, opacity: 1 }}>
    <!-- Account -->
    <p class="text-[14px]/[22px] font-medium text-slate-500">{$LL.ACCOUNT()}</p>
    <SettingsEntry
      icon={UserCircle}
      title="My profile"
      on:click={() => goto('/me/settings/profile')}
    />
    <SettingsEntry icon={Gear} title="App settings" on:click={() => goto('/me/settings/app')} />
    <SettingsEntry
      icon={Vault}
      title="Backup and recovery"
      on:click={() => goto('/me/settings/backup')}
    />
    <SettingsEntry icon={SignOut} title="Log out" hasCaretRight={false} todo />

    <!-- Support -->
    <p class="text-[14px]/[22px] font-medium text-slate-500">{$LL.SUPPORT()}</p>
    <SettingsEntry icon={Info} title="About UniMe" todo />
    <SettingsEntry icon={Envelope} title="Send feedback" hasCaretRight={false} todo />
    <div class="pb-4" />

    <div
      class="flex w-full flex-col items-center justify-center text-[14px]/[22px] font-medium text-slate-500"
    >
      <div>0.2.9</div>
      <div class="flex items-center pb-4">
        <p>Built with Tauri</p>
        <Heart variation="solid" size="18" class="pl-1" />
      </div>
      <div>GPL-3.0</div>
      <div>2023 Impierce Technologies</div>
    </div>
  </div>
</div>
