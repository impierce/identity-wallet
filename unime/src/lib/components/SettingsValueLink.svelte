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
  <div class="text-sm font-medium text-primary">
    {value}
  </div>
</a>
