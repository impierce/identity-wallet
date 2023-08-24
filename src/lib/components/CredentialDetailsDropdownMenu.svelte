<script lang="ts">
  import { fly } from 'svelte/transition';

  import { createDropdownMenu, melt } from '@melt-ui/svelte';

  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';

  const {
    elements: { trigger, menu, item, separator, arrow },
    states: { open }
  } = createDropdownMenu({
    forceVisible: true
  });
</script>

<button type="button" class="trigger p-1" use:melt={$trigger} aria-label="Update dimensions">
  <DotsThreeVertical class="h-6 w-6" />
  <span class="sr-only">Open Popover</span>
</button>

{#if $open}
  <div
    class="z-10 flex max-h-[300px] flex-col rounded-2xl bg-white p-3 shadow-md"
    use:melt={$menu}
    transition:fly={{ duration: 150, y: -10 }}
  >
    <button
      class="rounded-xl px-3 py-2 font-medium text-slate-800 hover:bg-slate-100 active:bg-slate-100"
      use:melt={$item}>Change name</button
    >
    <button
      class="rounded-xl px-3 py-2 font-medium text-slate-800 hover:bg-slate-100 active:bg-slate-100"
      use:melt={$item}>Change appearance</button
    >
    <!-- <div class="" use:melt={$item}> -->
    <button
      class="rounded-xl px-3 py-2 font-medium text-red-500 hover:bg-red-100 active:bg-red-100"
      use:melt={$item}>Delete credential</button
    >
    <!-- </div> -->
  </div>
{/if}
<div use:melt={$arrow} />
