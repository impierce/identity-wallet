<script lang="ts">
  import type { SvelteComponent } from 'svelte';

  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';

  // TODO: type accordingly
  // `typeof SvelteComponent` is not compatible with imports used in unplugin-icons,
  // namely `SvelteComponent<SvelteHTMLElements['svg']>`
  export let icon: any;
  export let title: string;
  export let description: string | undefined = undefined;
</script>

<!--
@component
Highlighted message with padded icon.

### Props

- icon
- title
- description (optional)
-->
<div class="flex flex-col items-center justify-center space-y-[15px] text-center">
  <PaddedIcon {icon} />
  <p class="text-[22px]/[30px] font-semibold tracking-tight text-slate-700 dark:text-grey">{title}</p>
  {#if description}
    <p class="w-[280px] text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">{description}</p>
  {/if}
</div>
