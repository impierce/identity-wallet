<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import { incompleteLocales, locales } from '$lib/app/locales';
  import Button from '$lib/components/atoms/Button.svelte';
  import BottomDrawer from '$lib/components/molecules/dialogs/BottomDrawer.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  $: selected = locales.find((l) => l.locale === $state?.locale) ?? locales.at(0)!!;

  let isOpen = false;
</script>

<BottomDrawer titleText={$LL.ONBOARDING.WELCOME.SELECT_LANGUAGE()} {isOpen}>
  <button
    slot="trigger"
    use:melt={trigger}
    let:trigger
    on:click={() => (isOpen = true)}
    class="flex w-fit items-center justify-center rounded-lg border border-grey bg-silver px-[15px] py-3 dark:border-blue dark:bg-navy"
  >
    <div class="pr-[10px]">
      <svelte:component this={selected.flag} class="h-5 w-5 rounded-full" />
    </div>
    <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{selected.displayName}</div>
  </button>

  <div slot="content" class="flex w-full flex-col space-y-[5px]">
    {#each locales as l}
      <button
        on:click={() => {
          dispatch({ type: '[Settings] Set locale', payload: { locale: l.locale } });
          isOpen = false;
        }}
        class="flex items-center rounded-lg border p-[10px]
          {l.locale === selected.locale ? 'border-grey bg-silver dark:border-blue dark:bg-navy' : 'border-transparent'}
          {incompleteLocales.includes(l.locale) ? 'opacity-30 grayscale' : ''}"
        disabled={incompleteLocales.includes(l.locale)}
      >
        <div class="pr-[10px]">
          <svelte:component this={l.flag} class="h-5 w-5 rounded-full" />
        </div>
        <div class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{l.displayName}</div>
        {#if incompleteLocales.includes(l.locale)}
          <div class="ml-auto text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
            {$LL.SETTINGS.APP.LANGUAGE.COMING_SOON()}
          </div>
        {/if}
      </button>
    {/each}
  </div>

  <div class="mt-6 w-full" slot="close" let:close>
    <Button label={$LL.CLOSE()} trigger={close} />
  </div>
</BottomDrawer>

<!-- TODO: find a better (global) solution -->
<style>
  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the drawer is open */
    position: unset !important;
  }
</style>
