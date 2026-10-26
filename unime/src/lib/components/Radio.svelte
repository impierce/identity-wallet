<script lang="ts">
  import { CheckBoldIcon } from '$lib/icons';

  export let checked = false;
  export let disabled = false;
</script>

<!--
@component
A round selection indicator for picking one option from a list. The surrounding element handles the click.

### Props
- checked
- disabled
-->
<div
  role="radio"
  aria-checked={checked}
  aria-disabled={disabled}
  class="flex size-6 min-w-6 items-center justify-center rounded-full text-white dark:text-dark
    {checked ? 'bg-primary' : 'border-[1.5px] border-slate-200 dark:border-slate-600'}
    {disabled ? 'opacity-50' : ''}"
>
  {#if checked}
    <CheckBoldIcon class="h-3 w-3" />
  {/if}
</div>
