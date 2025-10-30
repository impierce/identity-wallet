<script lang="ts">
  import type { Component } from 'svelte';

  import type { SVGAttributes } from 'svelte/elements';

  export let icon: Component<SVGAttributes<SVGSVGElement>>;
</script>

<!--
@component
An icon with a padded background in the primary color.

### Props

- icon
-->
<div class="flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-primary">
  <svelte:component this={icon} class="h-7 w-7 text-background-alt" />
</div>
