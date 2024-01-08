<script lang="ts">
  import { goto } from '$app/navigation';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import SettingsEntry from '$src/lib/settings/SettingsEntry.svelte';

  import Keyboard from '~icons/ph/keyboard-fill';
  import SmileyWink from '~icons/ph/smiley-wink-fill';
  import Trash from '~icons/ph/trash-fill';
</script>

<TopNavigation on:back={() => history.back()} title="My Profile" />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    <SettingsEntry icon={Keyboard} title="Profile name" on:click={() => goto('/me/settings/profile/name')} />
    <!-- on:click={() => goto('/welcome/customize/name')} -->
    <SettingsEntry icon={SmileyWink} title="Display picture" todo />
    <!-- on:click={() => goto('/me/settings/profile/avatar')} -->

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
        <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">Delete profile</p>
      </button>

      <div slot="content" class="w-full pb-[10px] pt-[20px]">
        <button
          class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
          on:click={() => dispatch({ type: '[App] Reset' })}>Yes, delete everything</button
        >
      </div>
      <Button variant="secondary" slot="close" let:close trigger={close} label="No, keep my profile" />
    </BottomDrawer>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
