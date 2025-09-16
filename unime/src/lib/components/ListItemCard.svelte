<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { Image } from '$lib/components';
  import { ensureLightIcon } from '$lib/utils/image';

  const dispatch = createEventDispatcher();

  export let id: string;
  export let title: string;
  export let description: string | undefined = undefined;
  export let type: 'data' | 'badge' = 'data';
  export let isTempAsset = false;
  export let icon: string | undefined = undefined;
  export let isInvalid = false;

  let useFallback = false;
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
  class={`flex h-16 w-full items-center justify-start rounded-xl bg-white p-2 dark:bg-dark ${isInvalid ? 'opacity-60' : ''}`}
  on:click={() => dispatch('click')}
>
  <!-- min-h-[64px] needed? -->
  <!-- Image or icon -->
  <slot name="image">
    <div
      class={`mr-4 flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg p-1 ${useFallback ? 'bg-silver dark:bg-navy' : 'bg-white'}`}
    >
      <!-- useFallback from <Image> (child) is bound to a local variable in <ListItemCard> (parent) with the same name to determine which background color to display -->
      <Image
        {id}
        iconFallback={ensureLightIcon(icon) ?? (type === 'data' ? 'UserLight' : 'CertificateLight')}
        {isTempAsset}
        bind:useFallback
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
