<script lang="ts">
  /* TODO: extract to ui-component */
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import LL from '$i18n/i18n-svelte';
  import { slide } from 'svelte/transition';

  import { createAccordion, melt } from '@melt-ui/svelte';

  import Button from '$lib/components/atoms/Button.svelte';
  import { CaretDownBoldIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  import type { Goal } from '../../types';

  const {
    elements: { content, item, trigger, root },
    helpers: { isSelected },
  } = createAccordion({});

  let goal: Goal = $state?.user_journey?.goals.find((g) => g.id === parseInt($page.params.id));
</script>

<div class="flex h-full flex-col items-center justify-evenly space-y-8 bg-silver p-6 dark:bg-navy">
  <p class="text-center text-2xl font-semibold text-slate-700 dark:text-grey">{goal?.label}</p>

  <div class="mx-auto flex w-full grow flex-col space-y-4" {...$root}>
    <!-- {#each items as { id, title, description }, i} -->
    {#each goal?.faqs as faq}
      <div
        use:melt={$item(faq.id.toString())}
        class="overflow-hidden rounded-xl border
             border-slate-200 transition-colors focus-within:relative
            focus-within:z-10 focus-within:ring focus-within:ring-primary dark:border-slate-600"
      >
        <h2 class="flex">
          <button
            use:melt={$trigger(faq.id.toString())}
            class="flex h-12 flex-1 cursor-pointer items-center justify-between
                 bg-white px-5 text-base font-medium leading-none
                 text-slate-800 transition-colors hover:bg-opacity-95 focus:!ring-0 dark:bg-dark dark:text-grey"
          >
            <div class="flex w-full items-center justify-between">
              <p class="text-[13px]/[24px] font-medium">{faq.title}</p>
              <CaretDownBoldIcon />
            </div>
          </button>
        </h2>
        {#if $isSelected(faq.id.toString())}
          <div
            class="overflow-hidden bg-white text-[12px]/[14px] font-medium text-slate-500 dark:bg-dark dark:text-slate-300"
            use:melt={$content(faq.id.toString())}
            transition:slide
          >
            <div class="px-5 pb-4">
              {faq.content}
            </div>
          </div>
        {/if}
      </div>
    {/each}
  </div>

  <Button label={$LL.CONTINUE()} on:click={() => goto('/goals/2/step/0')} />
</div>
