<script lang="ts">
  import type { Snippet } from 'svelte';

  import { writable } from 'svelte/store';

  import { createSwitch, melt } from '@melt-ui/svelte';

  interface Props {
    children: Snippet;
    checked?: boolean;
    onchange: (checked: boolean) => void;
  }

  let { checked = false, onchange, children }: Props = $props();

  // Provide own store to `createSwitch` and set initial value.
  const checkedStore = writable(checked);

  const {
    elements: { root },
  } = createSwitch({
    // Make switch controlled.
    checked: checkedStore,
    onCheckedChange: ({ next }) => {
      onchange(next);
      return next;
    },
  });

  $effect(() => {
    // Update the switch store when `checked` changes.
    $checkedStore = checked;
  });

  // ID to link `label` and `button`.
  const id = crypto.randomUUID();
</script>

<div data-component="Switch" class="flex items-center justify-between gap-2">
  <label {id} class="grow">
    {@render children()}
  </label>
  <button
    use:melt={$root}
    aria-labelledby={id}
    class="group relative h-7 w-11 rounded-full bg-text/25 transition-colors data-[state=checked]:bg-primary"
  >
    <span
      class="m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-background-alt transition group-data-[state=checked]:translate-x-[18px]"
    ></span>
  </button>
</div>
