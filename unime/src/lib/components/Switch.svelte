<script lang="ts">
  import type { Snippet } from 'svelte';

  import { createSwitch, melt } from '@melt-ui/svelte';

  interface Props {
    children: Snippet;
    initialChecked?: boolean;
    onchange: (checked: boolean) => void;
  }

  let { initialChecked = false, onchange, children }: Props = $props();

  const {
    elements: { root },
  } = createSwitch({
    defaultChecked: initialChecked,
    onCheckedChange: ({ next }) => {
      onchange(next);
      return next;
    },
  });

  const id = crypto.randomUUID();

  // If we made `initialChecked` reactive with `$effect`, we might get an endless loop.
  // This happens when `onchange` triggers a backend state update that is fed into `checked.`
</script>

<div class="flex items-center justify-between gap-2">
  <label {id} class="grow">
    {@render children()}
  </label>
  <button
    use:melt={$root}
    aria-labelledby={id}
    class="group relative h-7 w-11 rounded-full bg-text transition-colors data-[state=checked]:bg-primary"
  >
    <span
      class="m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-background transition group-data-[state=checked]:translate-x-[18px]"
    ></span>
  </button>
</div>
