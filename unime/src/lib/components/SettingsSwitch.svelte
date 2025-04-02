<script lang="ts">
  import type { Snippet } from 'svelte';

  import Switch from './Switch.svelte';

  interface Props {
    checked?: boolean;
    disabled?: boolean;
    onchange: (checked: boolean) => void;
    icon?: Snippet;
    children: Snippet;
  }

  let { icon, checked = false, disabled = false, onchange, children }: Props = $props();
</script>

<div
  data-component="SettingsSwitch"
  class="flex h-14 w-full items-center gap-4 rounded-xl bg-background-alt p-4 {disabled
    ? 'pointer-events-none opacity-50'
    : ''}"
>
  {#if icon}
    <div class="flex items-center justify-center">
      {@render icon()}
    </div>
  {/if}
  <div class="grow">
    <Switch {checked} {onchange}>
      <span class="text-sm font-medium text-text">
        {@render children()}
      </span>
    </Switch>
  </div>
</div>
