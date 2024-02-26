<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';

  let name = $state.profile_settings.profile?.name;
</script>

<div class="content-height relative flex flex-col">
  <TopNavBar title={$LL.SETTINGS.PROFILE.PROFILE_NAME.NAVBAR_TITLE()} on:back={() => history.back()} />
  <div class="flex grow flex-col items-center bg-silver p-8 dark:bg-navy">
    <input
      class="w-[280px] rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
      placeholder={$LL.SETTINGS.PROFILE.PROFILE_NAME.INPUT_PLACEHOLDER()}
      bind:value={name}
    />
  </div>
  <div class="absolute bottom-0 w-full rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SETTINGS.PROFILE.PROFILE_NAME.CONFIRM()}
      on:click={async () => {
        await dispatch({ type: '[Settings] Update profile', payload: { name } });
        goto('/me/settings');
      }}
      disabled={!!!name}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
