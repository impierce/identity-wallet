<script lang="ts">
  import type { Component } from 'svelte';

  import type { SVGAttributes } from 'svelte/elements';

  import { PaddedIcon } from '$lib/components';

  export let icon: Component<SVGAttributes<SVGSVGElement>>;
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
  <p class="dark:text-grey text-[22px]/[30px] font-semibold tracking-tight text-slate-700">{title}</p>
  {#if description}
    <p class="w-[280px] text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">{description}</p>
  {/if}
</div>
