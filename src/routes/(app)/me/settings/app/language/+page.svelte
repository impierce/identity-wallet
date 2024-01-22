<script>
  import { locales } from '$lib/app/locales';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';

  $: selected = locales.find((l) => l.locale === $state?.locale);
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.LANGUAGE.NAVBAR_TITLE()} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="flex flex-col space-y-[10px] px-4 py-5">
    {#each locales as language}
      <button
        class="flex h-14 items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
        on:click={() => dispatch({ type: '[Settings] Set locale', locale: language.locale })}
      >
        <svelte:component this={language.flag} class="h-5 w-5 rounded-full" />
        <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
          {language.displayName}
        </p>
        {#if selected && language.locale === selected.locale}
          <Check class="h-5 w-5 text-primary" />
        {/if}
      </button>
    {/each}
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
