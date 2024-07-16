<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade, slide } from 'svelte/transition';

  import { createAccordion, melt } from '@melt-ui/svelte';

  import Button from '$lib/components/atoms/Button.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { CaretDownBoldIcon } from '$lib/icons';

  const {
    elements: { content, item, trigger, root },
    helpers: { isSelected },
  } = createAccordion({});

  const items = [
    {
      id: 'item-1',
      title: $LL.ONBOARDING.PLEDGE.ITEM_1.TITLE(),
      description: $LL.ONBOARDING.PLEDGE.ITEM_1.DESCRIPTION(),
    },
    {
      id: 'item-2',
      title: $LL.ONBOARDING.PLEDGE.ITEM_2.TITLE(),
      description: $LL.ONBOARDING.PLEDGE.ITEM_2.DESCRIPTION(),
    },
    {
      id: 'item-3',
      title: $LL.ONBOARDING.PLEDGE.ITEM_3.TITLE(),
      description: $LL.ONBOARDING.PLEDGE.ITEM_3.DESCRIPTION(),
    },
  ];
</script>

<TopNavBar title={$LL.ONBOARDING.PLEDGE.NAVBAR_TITLE()} on:back={() => history.back()} />
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.PLEDGE.TITLE_1()} <span class="text-primary">{$LL.ONBOARDING.PLEDGE.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">{$LL.ONBOARDING.PLEDGE.SUBTITLE()}</p>
  </div>
  <!-- Accordion -->
  <div class="mx-auto flex max-w-full flex-col space-y-4" {...$root}>
    {#each items as { id, title, description }}
      <div
        use:melt={$item(id)}
        class="overflow-hidden rounded-xl border
             border-slate-200 transition-colors focus-within:relative
            focus-within:z-10 focus-within:ring focus-within:ring-primary dark:border-slate-600"
      >
        <h2 class="flex">
          <button
            use:melt={$trigger(id)}
            class="flex h-12 flex-1 cursor-pointer items-center justify-between
                 bg-white px-5 text-base font-medium leading-none
                 text-slate-800 transition-colors hover:bg-opacity-95 focus:!ring-0 dark:bg-dark dark:text-grey"
          >
            <div class="flex w-full items-center justify-between">
              <p class="text-[13px]/[24px] font-medium">{title}</p>
              <CaretDownBoldIcon />
            </div>
          </button>
        </h2>
        {#if $isSelected(id)}
          <div
            class="overflow-hidden bg-white text-[12px]/[14px] font-medium text-slate-500 dark:bg-dark dark:text-slate-300"
            use:melt={$content(id)}
            transition:slide
          >
            <div class="px-5 pb-4">
              {description}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>
</div>

<div
  class="space-y-[10px] rounded-t-3xl bg-white p-6 dark:bg-dark"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/terms')} />
</div>
