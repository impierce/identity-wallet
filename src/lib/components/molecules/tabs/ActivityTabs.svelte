<script lang="ts">
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  import { createTabs, melt } from '@melt-ui/svelte';

  const {
    elements: { root, list, content, trigger },
    states: { value },
  } = createTabs({ defaultValue: 'connections' });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });

  const triggers = [
    { id: 'connections', title: 'Connections' },
    { id: 'history', title: 'History' },
  ];
</script>

<div use:melt={$root} class="flex h-full flex-col overflow-hidden">
  <div
    use:melt={$list}
    class="flex h-[39px] shrink-0 overflow-x-auto rounded-xl bg-white dark:bg-dark"
    aria-label="Manage your activity"
  >
    {#each triggers as triggerItem}
      <button
        use:melt={$trigger(triggerItem.id)}
        class="trigger relative m-1 rounded-lg px-3 py-2 text-xs font-semibold text-slate-500 data-[state=active]:bg-primary data-[state=active]:text-white dark:text-slate-400 dark:data-[state=active]:text-slate-800"
      >
        {triggerItem.title}
        <!-- Indicator: active navigation item -->
        <!-- {#if $value === triggerItem.id}
            <div
              in:send={{ key: 'trigger' }}
              out:receive={{ key: 'trigger' }}
              class="absolute bottom-1 left-1/2 h-1 w-6 -translate-x-1/2 rounded-full bg-red-500"
            />
          {/if} -->
      </button>
    {/each}
  </div>

  <div use:melt={$content('connections')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="connections" />
  </div>

  <div use:melt={$content('history')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="history" />
  </div>
</div>

<style lang="postcss">
  .trigger {
    flex: 1;

    &:focus {
      position: relative;
    }

    &:focus-visible {
      @apply z-10 ring-2;
    }

    &[data-state='active'] {
      @apply focus:relative;
    }
  }
</style>
