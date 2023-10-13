<script lang="ts">
  import { goto } from '$app/navigation';

  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';
  import { developer_mode } from '$src/stores';

  import ChatCircleText from '~icons/ph/chat-circle-text-fill';
  import Code from '~icons/ph/code-fill';
  import Confetti from '~icons/ph/confetti-fill';
  import Eye from '~icons/ph/eye-fill';
  import LockSimpleOpen from '~icons/ph/lock-key-fill';
  import Password from '~icons/ph/password-fill';
  import Sun from '~icons/ph/sun-fill';
  import Translate from '~icons/ph/translate-fill';
</script>

<TopNavigation on:back={() => history.back()} title="App Settings" />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Translate} title="Language" hasCaretRight={false} textRight="English" todo />
    <SettingsEntry icon={Sun} title="Theme" on:click={() => goto('/me/settings/app/theme')} />
    <SettingsEntry icon={LockSimpleOpen} title="Security" on:click={() => goto('/me/settings/app/security')} />
    <SettingsEntry icon={Confetti} title="Onboarding journey" hasCaretRight={false} textRight="Restart" todo />
    <SettingsEntry icon={ChatCircleText} title="Hints and tips" hasCaretRight={false} textRight="Reset" todo />
    <!-- TODO: DEV_MODE -->
    <!-- <SettingsEntry
      icon={Code}
      title="Developer mode"
      hasCaretRight={false}
      textRight={$developer_mode ? 'On' : 'Off'}
      on:click={() => developer_mode.set(!$developer_mode)}
    /> -->
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
