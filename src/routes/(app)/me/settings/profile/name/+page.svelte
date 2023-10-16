<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import Button from '$src/lib/components/Button.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let name = $state.active_profile?.name;
</script>

<div class="content-height relative flex flex-col">
  <TopNavigation title="Change profile name" on:back={() => history.back()} />
  <div class="flex grow flex-col items-center bg-silver p-8 dark:bg-navy">
    <input
      class="w-[280px] rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
      placeholder="Enter a profile name"
      bind:value={name}
    />
  </div>
  <div class="absolute bottom-0 w-full rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button
      label="Update"
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
