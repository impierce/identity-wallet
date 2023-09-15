<script lang="ts">
  import { TopNavigation } from '@impierce/ui-components';

  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';

  import Keyboard from '~icons/ph/keyboard-fill';
</script>

<TopNavigation on:back={() => history.back()} title="Backup and recovery" />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Keyboard} title="Your DID" todo />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
