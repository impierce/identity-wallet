<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CheckBoldIcon } from '$lib/icons';
  import { disabledLocales, locales } from '$lib/locales';
  import { state } from '$lib/stores';

  $: selected = locales.find((l) => l.locale === $state?.profile_settings.locale);
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.LANGUAGE.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col space-y-[10px] bg-silver px-4 py-5 dark:bg-navy">
  {#each locales as l}
    <button
      class="flex h-14 items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark
          {disabledLocales.includes(l.locale) ? 'opacity-30 grayscale' : ''}"
      on:click={() => dispatch({ type: '[Settings] Set locale', payload: { locale: l.locale } })}
      disabled={disabledLocales.includes(l.locale)}
    >
      <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
        {l.displayName}
      </p>
      {#if disabledLocales.includes(l.locale)}
        <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {$LL.SETTINGS.APP.LANGUAGE.COMING_SOON()}
        </div>
      {/if}
      {#if selected && l.locale === selected.locale}
        <CheckBoldIcon class="h-5 w-5 text-primary" />
      {/if}
    </button>
  {/each}
</div>
