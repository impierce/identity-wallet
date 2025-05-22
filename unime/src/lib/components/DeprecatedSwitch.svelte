<script lang="ts">
  // TODO: This legacy switch is kept around to avoid a bigger refactor in the dev mode code.
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

<!-- TODO: Adding a hard-wired `aria-label` to silence accessibility error. -->
<button
  use:melt={$root}
  aria-label="Toggle switch"
  class="group relative h-7 w-11 rounded-full bg-primary/25 transition-colors disabled:opacity-50 data-[state=checked]:bg-primary"
  disabled
>
  <span
    class="m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-white transition group-data-[state=checked]:translate-x-[18px] dark:bg-dark"
  ></span>
</button>
<input use:melt={$input} />
