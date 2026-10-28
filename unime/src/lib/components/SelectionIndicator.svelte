<script lang="ts">
  import { CheckBoldIcon } from '$lib/icons';

  export let checked = false;
  // Round like a radio button for a single choice, square like a checkbox for several.
  export let multiple = false;
</script>

<!--
@component
A purely visual selection indicator for a list option. The surrounding button handles the click.

### Props
- checked
- multiple
-->
<div
  class="flex size-6 min-w-6 items-center justify-center text-white dark:text-dark
    {multiple ? 'rounded-md' : 'rounded-full'}
    {checked
    ? 'bg-primary'
    : `border-[1.5px] ${multiple ? 'border-slate-300' : 'border-slate-200'} dark:border-slate-600`}"
>
  {#if checked}
    <CheckBoldIcon class="h-3 w-3" />
  {/if}
</div>
