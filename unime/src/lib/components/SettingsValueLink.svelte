<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    href: string;
    value: string;
    disabled?: boolean;
    icon: Snippet;
    children: Snippet;
  }

  let { href, value, disabled = false, icon, children }: Props = $props();
</script>

<a
  data-component="SettingsValueLink"
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
  <div class="text-primary text-sm font-medium">
    {value}
  </div>
</a>
