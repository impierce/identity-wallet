<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import { melt } from '@melt-ui/svelte';

  import Button from '$lib/components/atoms/Button.svelte';
  import BottomDrawer from '$lib/components/molecules/dialogs/BottomDrawer.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import DE from '~icons/flag/de-1x1';
  import NL from '~icons/flag/nl-1x1';
  import US from '~icons/flag/us-1x1';

  const languages: {
    locale: string;
    flag: typeof SvelteComponent<SvelteHTMLElements['svg']>;
    displayName: string;
  }[] = [
    { locale: 'en', flag: US, displayName: 'English (US)' },
    { locale: 'nl', flag: NL, displayName: 'Nederlands' },
    { locale: 'de', flag: DE, displayName: 'Deutsch' },
  ];

  $: selected = languages.find((l) => l.locale === $state?.locale) ?? languages.at(0)!!;

  let isOpen = false;
</script>

<BottomDrawer titleText={$LL.ONBOARDING.WELCOME.SELECT_LANGUAGE()} {isOpen}>
  <button
    slot="trigger"
    use:melt={trigger}
    let:trigger
    on:click={() => (isOpen = true)}
    class="flex w-fit items-center justify-center rounded-lg border border-grey bg-silver px-[15px] py-3"
  >
    <div class="pr-[10px]">
      <svelte:component this={selected.flag} class="h-5 w-5 rounded-full" />
    </div>
    <div class="text-[13px]/[24px] font-medium text-slate-800">{selected.displayName}</div>
  </button>

  <div slot="content" class="flex w-full flex-col space-y-[5px]">
    {#each languages as language}
      <button
        on:click={() => {
          dispatch({ type: '[Settings] Set locale', locale: language.locale });
          isOpen = false;
        }}
        class="flex items-center rounded-lg p-[10px] {language.locale === selected.locale
          ? 'border border-grey bg-silver'
          : ''}"
      >
        <div class="pr-[10px]">
          <svelte:component this={language.flag} class="h-5 w-5 rounded-full" />
        </div>
        <div class="text-[13px]/[24px] font-medium text-slate-800">{language.displayName}</div>
      </button>
    {/each}
  </div>

  <div class="mt-6 w-full" slot="close" let:close>
    <Button label={$LL.CLOSE()} trigger={close} />
  </div>
</BottomDrawer>
