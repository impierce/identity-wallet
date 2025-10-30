<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { Button, TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState, navigationDirection } from '$lib/stores';

  let name = $state($appState.profile_settings.profile?.name);

  const parentRoute = '/me/settings/profile';

  beforeNavigate(({ type, cancel }) => {
    if (type === 'popstate') {
      cancel();
      goto(parentRoute);
    }
  });

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar
  on:back={() => goto(parentRoute)}
  title={$LL.SETTINGS.PROFILE.PROFILE_NAME.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<div class="content-height relative flex flex-col" in:fly={{ x, duration, opacity: 1 }}>
  <div class="bg-silver dark:bg-navy flex grow flex-col items-center p-8">
    <input
      class="dark:bg-dark w-[280px] rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:text-slate-300"
      placeholder={$LL.SETTINGS.PROFILE.PROFILE_NAME.INPUT_PLACEHOLDER()}
      bind:value={name}
    />
  </div>
  <div class="dark:bg-dark absolute bottom-0 z-10 w-full rounded-t-3xl bg-white p-6">
    <Button
      label={$LL.SETTINGS.PROFILE.PROFILE_NAME.CONFIRM()}
      on:click={async () => {
        await dispatch({ type: '[Settings] Update profile', payload: { name } });
        goto(parentRoute);
      }}
      disabled={!name}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, title bar: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
