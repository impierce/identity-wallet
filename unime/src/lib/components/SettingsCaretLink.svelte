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
  class="bg-background-alt flex h-14 w-full items-center gap-4 rounded-xl p-4 {disabled
    ? 'pointer-events-none opacity-50'
    : ''}"
>
  <div class="flex items-center justify-center">
    {@render icon()}
  </div>
  <div class="text-text grow text-sm font-medium">
    {@render children()}
  </div>
  <div class="flex items-center justify-center">
    <CaretRightBoldIcon class="text-text h-4 w-4"></CaretRightBoldIcon>
  </div>
</a>
