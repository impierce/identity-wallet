<script lang="ts">
  import markdownit from 'markdown-it';
  import { slide } from 'svelte/transition';

  import { createAccordion, melt } from '@melt-ui/svelte';

  import { CaretDownBoldIcon } from '$lib/icons';

  const md = markdownit();

  const {
    elements: { content, item, trigger, root },
    helpers: { isSelected },
  } = createAccordion({});

  type Item = {
    id: string;
    title: string;
    description: string;
  };

  export let items: Item[] = [];
</script>

<div class="mx-auto flex max-w-full flex-col space-y-4" {...$root}>
  {#each items as { id, title, description }}
    <div
      use:melt={$item(id)}
      class="overflow-hidden rounded-xl transition-colors focus-within:relative focus-within:z-10 focus-within:ring-3 focus-within:ring-primary dark:border-slate-600"
    >
      <h2 class="flex">
        <button
          use:melt={$trigger(id)}
          class="hover:bg-opacity-95 flex min-h-12 flex-1 cursor-pointer items-center justify-between bg-white text-base leading-none font-medium text-slate-800 transition-colors focus:ring-0! dark:bg-background dark:text-grey"
        >
          <div class="flex w-full items-center justify-between py-2">
            <p class="text-left text-lg font-bold">{title}</p>
            <CaretDownBoldIcon
              class="size-4 transition-transform"
              style={`transform: ${$isSelected(id) ? 'rotate(180deg)' : 'rotate(0deg)'};`}
            />
          </div>
        </button>
      </h2>

      {#if $isSelected(id)}
        <div
          use:melt={$content(id)}
          class="overflow-hidden bg-white text-sm text-slate-500 dark:bg-background dark:text-slate-300"
          transition:slide
        >
          <div class="pb-4">
            {@html md.render(description)}
          </div>
        </div>
      {/if}
    </div>
  {/each}
</div>
