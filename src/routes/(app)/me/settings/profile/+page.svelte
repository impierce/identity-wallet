<script lang="ts">
  import { goto } from '$app/navigation';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';

  import Keyboard from '~icons/ph/keyboard-fill';
  import SmileyWink from '~icons/ph/smiley-wink-fill';
  import Trash from '~icons/ph/trash-fill';
</script>

<TopNavigation on:back={() => history.back()} title="My Profile" />
<div class="flex min-h-full flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry
      icon={Keyboard}
      title="Profile name"
      on:click={() => goto('/me/settings/profile/name')}
    />
    <!-- on:click={() => goto('/welcome/customize/name')} -->
    <SettingsEntry icon={SmileyWink} title="Display picture" todo />
    <!-- on:click={() => goto('/welcome/customize/avatar')} -->

    <BottomDrawer
      titleText="Reset app"
      descriptionText="Are you sure you want to reset the entire app and remove all data?"
    >
      <!-- Delete profile (based on SettingsEntry) -->
      <button
        slot="trigger"
        let:trigger
        class="flex items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
        use:melt={trigger}
        on:click={() => {}}
      >
        <svelte:component this={Trash} class="h-5 w-5 text-rose-400" />
        <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800">Delete profile</p>
      </button>

      <div slot="content" class="flex flex-col">
        <!-- more content -->
        <button
          class="w-full rounded-lg bg-red-600 px-4 py-2 text-white"
          on:click={() => dispatch({ type: '[App] Reset' })}>Yes, delete everything</button
        >
      </div>
      <button
        slot="close"
        let:close
        use:melt={close}
        class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700">Cancel</button
      >
    </BottomDrawer>
  </div>
</div>
