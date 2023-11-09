<script lang="ts">
  import Switch from '$src/lib/components/atoms/Switch.svelte';
  import Button from '$src/lib/components/Button.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import ThemeSelect from '$src/lib/customize/ThemeSelect.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';
  import { state } from '$src/stores';

  import Eye from '~icons/ph/eye-fill';
  import Password from '~icons/ph/password-fill';

  let currentValue = $state.active_profile?.theme ?? 'system';
  let selected: string;
</script>

<TopNavigation on:back={() => history.back()} title="Security" />
<!-- <div class="h-full bg-silver p-4 dark:bg-navy"> -->
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Password} title="Password" hasCaretRight={false} textRight="Change" todo />
    <SettingsEntry icon={Eye} title="Biometrics" hasCaretRight={false}>
      <Switch active on:change={(e) => console.log(e.detail)} />
    </SettingsEntry>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
