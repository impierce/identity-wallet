<script lang="ts">
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  import { createTabs, melt } from '@melt-ui/svelte';

  import SlidersHorizontal from '~icons/ph/sliders-horizontal';

  const {
    elements: { root, list, content, trigger },
    states: { value },
  } = createTabs({ defaultValue: 'all' });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });

  const triggers = [
    { id: 'all', title: 'All' },
    { id: 'data', title: 'Data' },
    { id: 'badges', title: 'Badges' },
  ];
</script>

<div use:melt={$root} class="flex h-full flex-col overflow-hidden">
  <div class="flex">
    <div
      use:melt={$list}
      class="flex h-[39px] shrink-0 grow overflow-x-auto rounded-xl bg-white dark:bg-dark"
      aria-label="Manage your credentials"
    >
      {#each triggers as triggerItem}
        <button
          use:melt={$trigger(triggerItem.id)}
          class="trigger relative m-1 rounded-lg px-3 py-2 text-xs font-medium text-slate-500 first:font-semibold data-[state=active]:bg-primary data-[state=active]:text-white dark:text-slate-400 dark:data-[state=active]:text-slate-800"
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

    <button class="ml-3 flex h-10 w-10 items-center justify-center rounded-xl bg-white font-semibold dark:bg-dark">
      <SlidersHorizontal class="h-5 w-5 text-black dark:text-white" />
    </button>
  </div>

  <div use:melt={$content('all')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="all" />
  </div>

  <div use:melt={$content('data')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="data" />
  </div>

  <div use:melt={$content('badges')} class="hide-scrollbar grow overflow-y-scroll">
    <slot name="badges" />
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
