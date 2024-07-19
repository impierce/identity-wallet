<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { createCheckbox, melt } from '@melt-ui/svelte';

  import { CheckBoldIcon } from '$lib/icons';

  export let checked = false;
  export let disabled = false;

  const dispatch = createEventDispatcher();

  const {
    elements: { root, input },
    helpers: { isChecked },
    states: { checked: _checked }, // renaming inner "checked", because the export value is also called "checked"
  } = createCheckbox({
    defaultChecked: checked,
    disabled,
  });

  // react when checked value is updated from the outside
  $: {
    _checked.set(checked);
  }

  // emit change event when checked value is update from within the component
  _checked.subscribe((checked) => {
    dispatch('change', checked, {});
  });
</script>

<button
  use:melt={$root}
  class="flex h-6 w-6 appearance-none items-center justify-center rounded-md border-[1.5px]
            border-slate-300 p-[6px] text-white data-[disabled]:opacity-50
            {$isChecked ? 'border-none bg-primary' : 'bg-transparent'}"
  id="checkbox"
>
  {#if $isChecked}
    <CheckBoldIcon class="h-3 w-3" />
  {/if}
  <input use:melt={$input} />
</button>
