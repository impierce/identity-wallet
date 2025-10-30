<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import { TopNavBar } from '$lib/components';
  import { ANIMATION_DURATION as duration } from '$lib/constants';
  import { dispatch } from '$lib/dispatcher';
  import { CheckBoldIcon } from '$lib/icons';
  import { disabledLocales, locales } from '$lib/locales';
  import { navigationDirection, state } from '$lib/stores';

  let selected = $derived(locales.find((l) => l.locale === $state?.profile_settings.locale));

  const x = $derived($navigationDirection === 'down' ? 32 : -32);
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.LANGUAGE.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="bg-silver dark:bg-navy flex flex-col space-y-[10px] px-4 py-5" in:fly={{ x, duration, opacity: 1 }}>
  {#each locales as l}
    <button
      class="dark:bg-dark flex h-14 items-center space-x-4 rounded-xl bg-white p-4
          {disabledLocales.includes(l.locale) ? 'opacity-30 grayscale' : ''}"
      onclick={() => dispatch({ type: '[Settings] Set locale', payload: { locale: l.locale } })}
      disabled={disabledLocales.includes(l.locale)}
    >
      <p class="dark:text-grey grow text-left text-[13px]/[24px] font-medium text-slate-800">
        {l.displayName}
      </p>
      {#if disabledLocales.includes(l.locale)}
        <div class="dark:text-grey text-[13px]/[24px] font-medium text-slate-800">
          {$LL.SETTINGS.APP.LANGUAGE.COMING_SOON()}
        </div>
      {/if}
      {#if selected && l.locale === selected.locale}
        <CheckBoldIcon class="text-primary h-5 w-5" />
      {/if}
    </button>
  {/each}
</div>
