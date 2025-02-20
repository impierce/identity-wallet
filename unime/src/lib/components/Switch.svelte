<script lang="ts">
  import { createSwitch, melt } from '@melt-ui/svelte';

  let { active = false, onchange } = $props();

  const handleOnCheckedChange = ({ curr, next }: { curr: boolean; next: boolean }) => {
    onchange(next);
    return next;
  };

  const {
    elements: { root, input },
    states: { checked },
  } = createSwitch({
    defaultChecked: active,
    onCheckedChange: handleOnCheckedChange,
  });
</script>

<!-- TODO: This button has no text and should have a label instead to comply with accessibility standards.
     This component requires a `aria-labelledby` to reference the label element. But this would require a bigger refactor.
     Adding a hard-wired `aria-label` silences the error. It's a hack and requires a proper refactor.
-->
<button
  use:melt={$root}
  aria-label="Toggle switch"
  class="group relative h-7 w-11 rounded-full bg-primary/25 transition-colors data-[state=checked]:bg-primary"
>
  <span
    class="m-0.5 block h-5 w-5 translate-x-0.5 rounded-full bg-white transition group-data-[state=checked]:translate-x-[18px] dark:bg-dark"
  ></span>
</button>
<input use:melt={$input} />
