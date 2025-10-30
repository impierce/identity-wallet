<script lang="ts">
  import type { Snippet } from 'svelte';

  import Switch from './Switch.svelte';

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    onCheckedChange?: ({ curr, next }: { curr: boolean; next: boolean }) => boolean;
    icon?: Snippet;
    children: Snippet;
  }

  let { icon, checked = false, disabled = false, onCheckedChange, children }: Props = $props();
</script>

<div
  data-component="SettingsSwitch"
  class="bg-background-alt flex h-14 w-full items-center gap-4 rounded-xl p-4 {disabled
    ? 'pointer-events-none opacity-50'
    : ''}"
>
  {#if icon}
    <div class="flex items-center justify-center">
      {@render icon()}
    </div>
  {/if}
  <div class="grow">
    <Switch {checked} {onCheckedChange}>
      <span class="text-text text-sm font-medium">
        {@render children()}
      </span>
    </Switch>
  </div>
</div>
