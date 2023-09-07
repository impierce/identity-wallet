<script lang="ts">
  import { TopNavigation } from '@impierce/ui-components';

  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';

  import Keyboard from '~icons/ph/keyboard-fill';
</script>

<TopNavigation on:back={() => history.back()} title="App Settings" />
<div class="flex min-h-full flex-col bg-bg-secondary dark:bg-bg-dark-secondary">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Keyboard} title="Your DID" todo />
  </div>
</div>
