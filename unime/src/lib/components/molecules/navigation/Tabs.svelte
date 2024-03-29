<script lang="ts">
  import { cubicInOut } from 'svelte/easing';
  import { crossfade } from 'svelte/transition';

  import { createTabs, melt } from '@melt-ui/svelte';

  export let triggers: string[];

  const {
    elements: { root, list, content, trigger },
    states: { value },
  } = createTabs({ defaultValue: triggers[0] });

  const [send, receive] = crossfade({
    duration: 250,
    easing: cubicInOut,
  });
</script>

<!--
@component
A tab component.

### Props

- triggers

### Usage
```tsx
<Tabs triggers={['First', 'Second', 'Third']} />
```
-->
<div use:melt={$root} class="flex flex-col">
  <div use:melt={$list} class="flex h-[39px] shrink-0 overflow-x-auto rounded-xl bg-white dark:bg-dark">
    {#each triggers as triggerItem}
      <button
        use:melt={$trigger(triggerItem)}
        class="trigger relative m-1 px-3 py-2 text-xs font-semibold text-slate-500 data-[state=active]:text-white dark:text-slate-400 dark:data-[state=active]:text-slate-800"
      >
        <p class="relative z-10">
          {triggerItem}
        </p>
        {#if $value === triggerItem}
          <div
            in:send={{ key: 'trigger' }}
            out:receive={{ key: 'trigger' }}
            class="absolute left-0 top-0 h-full w-full rounded-lg bg-primary"
          />
        {/if}
      </button>
    {/each}
  </div>
</div>

<!-- TODO: No dynamic slots possible in Svelte! -->
<!-- See issue: https://github.com/sveltejs/svelte/issues/3480 -->
<!-- {#each triggers as triggerItem}
  <div use:melt={$content(triggerItem)} class="hide-scrollbar grow overflow-y-scroll">
    <slot name={triggerItem} />
  </div>
{/each} -->

<!-- Workaround: use indexes -->
<div use:melt={$content(triggers[0])} class="hide-scrollbar grow overflow-y-scroll">
  <slot name="0" />
</div>

<div use:melt={$content(triggers[1])} class="hide-scrollbar grow overflow-y-scroll">
  <slot name="1" />
</div>

<div use:melt={$content(triggers[2])} class="hide-scrollbar grow overflow-y-scroll">
  <slot name="2" />
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
