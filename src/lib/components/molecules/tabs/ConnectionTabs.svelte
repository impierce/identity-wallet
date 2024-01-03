<script lang="ts">
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  import { createTabs, melt } from '@melt-ui/svelte';

  const {
    elements: { root, list, content, trigger },
    states: { value },
  } = createTabs({ defaultValue: 'summary' });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });

  const triggers = [
    { id: 'summary', title: 'Summary' },
    { id: 'data', title: 'Data' },
    { id: 'activity', title: 'Activity' },
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
        class="trigger relative m-1 px-3 py-2 text-xs font-semibold text-slate-500 data-[state=active]:text-white dark:text-slate-400 dark:data-[state=active]:text-slate-800"
      >
        <p class="relative z-10">
          {triggerItem.title}
        </p>
        {#if $value === triggerItem.id}
          <div
            in:send={{ key: 'trigger' }}
            out:receive={{ key: 'trigger' }}
            class="absolute left-0 top-0 h-full w-full rounded-lg bg-primary"
          />
        {/if}
      </button>
    {/each}
  </div>

  <div use:melt={$content('summary')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="summary" />
  </div>

  <div use:melt={$content('data')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="data" />
  </div>

  <div use:melt={$content('activity')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="activity" />
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
