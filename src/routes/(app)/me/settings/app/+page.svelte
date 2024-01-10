<script lang="ts">
  import { goto } from '$app/navigation';

  import { dispatch } from '$lib/dispatcher';
  import SettingsEntry from '$src/lib/app/settings/SettingsEntry.svelte';
  import Switch from '$src/lib/components/atoms/Switch.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';

  import ChatCircleText from '~icons/ph/chat-circle-text-fill';
  import Code from '~icons/ph/code-bold';
  import Confetti from '~icons/ph/confetti-fill';
  import Password from '~icons/ph/password-fill';
  import Sun from '~icons/ph/sun-fill';
  import Translate from '~icons/ph/translate-fill';
</script>

<TopNavBar on:back={() => history.back()} title="App Settings" />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Sun} title="Language" hasCaretRight={false} textRight="English" todo />
    <SettingsEntry icon={Sun} title="Theme" on:click={() => goto('/me/settings/app/theme')} />
    <SettingsEntry icon={Password} title="Password" todo />
    <SettingsEntry icon={Confetti} title="Onboarding journey" hasCaretRight={false} textRight="Restart" todo />
    <SettingsEntry icon={ChatCircleText} title="Hints and tips" hasCaretRight={false} textRight="Reset" todo />
    <SettingsEntry icon={Code} title="Developer mode" hasCaretRight={false}>
      <Switch
        active={$state?.dev_mode_enabled}
        on:change={() => dispatch({ type: '[DEV] Set dev mode', payload: { enabled: !$state?.dev_mode_enabled } })}
      />
    </SettingsEntry>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
