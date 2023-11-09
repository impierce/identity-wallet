<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { createSwitch, melt } from '@melt-ui/svelte';

  export let active: boolean = false;

  const dispatch = createEventDispatcher();

  const {
    elements: { root, input },
    states: { checked },
  } = createSwitch({
    defaultChecked: active,
  });

  checked.subscribe((c) => {
    dispatch('change', c);
  });
</script>

<button
  use:melt={$root}
  class="relative h-7 w-11 rounded-full bg-primary/25 transition-colors data-[state=checked]:bg-primary"
>
  <span class="thumb m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-white transition"></span>
</button>
<input use:melt={$input} />

<style>
  .thumb {
    /* TODO: refactor :global([...]) to inline tailwind in <span> */
  }

  :global([data-state='checked']) .thumb {
    transform: translateX(calc(2.75rem - 1.25rem - 0.125rem - 4px));
  }
</style>
