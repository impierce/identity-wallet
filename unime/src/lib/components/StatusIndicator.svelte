<script lang="ts">
  import { fade } from 'svelte/transition';

  import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';
  import { createPopover, melt } from '@melt-ui/svelte';

  import { Image } from '$lib/components';
  import { hash } from '$lib/utils';

  import Check from '~icons/ph/check-bold';
  import QuestionMark from '~icons/ph/question-mark-bold';
  import X from '~icons/ph/x-bold';

  export let status: ValidationStatus;
  export let title: string;
  export let description: string | undefined = undefined;
  export let logoUrl: string | undefined = undefined;

  const {
    elements: { trigger, content, arrow },
    states: { open },
  } = createPopover();
</script>

<!-- `use:melt` should be applied only when $$slots.popover exists. -->
<!-- This would require an #if statement in combination with duplicating the outer `div` (without use:melt). -->
<div
  use:melt={$trigger}
  class="flex items-center gap-2 rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
>
  <div class="flex grow flex-col">
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
      {title}
    </p>
    {#if description}
      <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{description}</p>
    {/if}
  </div>

  {#if logoUrl}
    <Image id={hash(logoUrl)} iconFallback="Certificate" isTempAsset={true} imgClass="h-10 w-10" />
  {/if}

  {#if status === 'Success'}
    <Check class="text-emerald-500" />
  {:else if status === 'Failure'}
    <X class="text-rose-500" />
  {:else}
    <QuestionMark class="text-slate-400 dark:text-slate-300" />
  {/if}
</div>

<!-- Render popover only when there is a popover slot. -->
{#if $$slots.popover && $open}
  <div
    use:melt={$content}
    transition:fade={{ duration: 200 }}
    class="z-10 w-1/2 rounded-2xl bg-blue p-[20px] text-white shadow-md dark:border dark:border-slate-500"
  >
    <div use:melt={$arrow} class="dark:border-l dark:border-t dark:border-slate-500" />
    <slot name="popover" />
  </div>
{/if}
