<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import Image from '$lib/components/atoms/Image.svelte';

  const dispatch = createEventDispatcher();

  export let id: string;
  export let title: string;
  export let description: string | undefined = undefined;
  export let type: 'data' | 'badge' = 'data';
  export let isTempAsset = false;
</script>

<!--
@component
A list entry in a card style.
Can be used for credentials, connections, etc.

### Props

- id
- title
- description (optional)
- type
- isTempAsset

### Slots
- image
- right

### Usage
```tsx
<ListItemCard id={'3cf73ecb'} />
```
-->
<button
  class="flex h-16 w-full items-center justify-start rounded-xl bg-white p-2 dark:bg-dark"
  on:click={() => dispatch('click')}
>
  <!-- min-h-[64px] needed? -->
  <!-- Image or icon -->
  <slot name="image">
    <!-- TODO: if Image does not load, then `bg-silver dark:bg-navy` -->
    <div class="mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg bg-white p-1">
      <Image
        {id}
        iconFallback={type === 'data' ? 'User' : 'Certificate'}
        {isTempAsset}
        iconClass="dark:text-slate-800"
      />
    </div>
  </slot>
  <!-- Text -->
  <div class="flex grow flex-col items-start overflow-x-auto text-left">
    <p class="line-clamp-2 w-full pr-4 text-[13px]/[18px] font-medium text-slate-800 dark:text-grey">
      {title}
    </p>
    {#if description}
      <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
        {description}
      </p>
    {/if}
  </div>
  <!-- Right slot (e.g. for a checkbox)-->
  <slot name="right" />
</button>
