<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { createSwitch, melt } from '@melt-ui/svelte';

  export let active = false;

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
  class="group relative h-7 w-11 rounded-full bg-primary/25 transition-colors data-[state=checked]:bg-primary"
>
  <span
    class="m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-white transition group-data-[state=checked]:translate-x-[18px] dark:bg-dark"
  ></span>
</button>
<input use:melt={$input} />
