<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { createCheckbox, melt } from '@melt-ui/svelte';

  import { CheckBoldIcon } from '$lib/icons';

  export let checked = false;
  export let disabled = false;
  // With `readonly=true`, the checkbox state can only be changed from outside the component.
  export let readonly = false;

  const dispatch = createEventDispatcher();

  const {
    elements: { root, input },
    helpers: { isChecked },
    states: { checked: checkedState },
  } = createCheckbox({
    defaultChecked: checked,
    disabled,
  });

  $: $checkedState = checked;

  // Emit change event when checked value is updated from within the component when not readonly
  $: if (!readonly) {
    dispatch('change', $checkedState, {});
  }
</script>

<!-- There is no elegant way to apply `use:melt` conditionally. Therefore duplication. -->
{#if !readonly}
  <button
    use:melt={$root}
    class="data-disabled:opacity-50 dark:text-dark flex h-6 w-6 appearance-none items-center justify-center
            rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
            {$isChecked ? 'bg-primary border-none' : 'bg-transparent'}"
  >
    {#if $isChecked}
      <CheckBoldIcon class="h-3 w-3" />
    {/if}
    <input use:melt={$input} />
  </button>
{:else}
  <button
    class="data-disabled:opacity-50 dark:text-dark flex h-6 w-6 appearance-none items-center justify-center
        rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
        {$isChecked ? 'bg-primary border-none' : 'bg-transparent'}"
  >
    {#if $isChecked}
      <CheckBoldIcon class="h-3 w-3" />
    {/if}
    <input use:melt={$input} />
  </button>
{/if}
