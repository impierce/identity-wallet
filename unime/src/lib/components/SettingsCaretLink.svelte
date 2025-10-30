<script lang="ts">
  import type { Snippet } from 'svelte';

  import { CaretRightBoldIcon } from '$lib/icons';

  interface Props {
    href: string;
    disabled?: boolean;
    icon: Snippet;
    children: Snippet;
  }

  let { href, disabled = false, icon, children }: Props = $props();
</script>

<a
  data-component="SettingsCaretLink"
  {href}
  class="flex h-14 w-full items-center gap-4 rounded-xl bg-background-alt p-4 {disabled
    ? 'pointer-events-none opacity-50'
    : ''}"
>
  <div class="flex items-center justify-center">
    {@render icon()}
  </div>
  <div class="grow text-sm font-medium text-text">
    {@render children()}
  </div>
  <div class="flex items-center justify-center">
    <CaretRightBoldIcon class="h-4 w-4 text-text"></CaretRightBoldIcon>
  </div>
</a>
