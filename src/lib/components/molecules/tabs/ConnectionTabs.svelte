<script lang="ts">
  import { createTabs, melt } from '@melt-ui/svelte';
  import { crossfade } from 'svelte/transition';
  import { cubicInOut } from 'svelte/easing';

  const {
    elements: { root, list, content, trigger },
    states: { value }
  } = createTabs({ defaultValue: 'summary' });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut
  });

  const triggers = [
    { id: 'summary', title: 'Summary' },
    { id: 'data', title: 'Data' },
    { id: 'history', title: 'History' }
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
        class="trigger relative m-1 rounded-xl px-3 py-2 text-xs font-semibold text-slate-500 dark:text-slate-400"
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

  <div use:melt={$content('summary')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="summary" />
  </div>

  <div use:melt={$content('data')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="data" />
  </div>

  <div use:melt={$content('history')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="history" />
  </div>
</div>

<style lang="postcss">
  .trigger {
    /* display: flex;
      align-items: center;
      justify-content: center; */

    cursor: default;
    user-select: none;

    flex: 1;

    &:focus {
      position: relative;
    }

    &:focus-visible {
      @apply z-10 ring-2;
    }

    &[data-state='active'] {
      @apply focus:relative;
      /* TODO: set to white in light mode, dark:slate-800 */
      color: theme('colors.slate.800');
      background-color: theme('colors.primary');
    }
  }
</style>
