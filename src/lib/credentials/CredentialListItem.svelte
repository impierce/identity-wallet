<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import Image from '$lib/components/atoms/Image.svelte';
  import { getImageAsset } from '$lib/utils';

  // TODO: make more generic "ListItem" with slots: "image", "right"

  export let id: string | undefined = undefined; // TODO: should not be able to be undefined
  export let title: string;
  export let description: string | undefined = undefined;
  export let type: 'data' | 'badge' = 'data';

  export let isTempAsset = false;

  const dispatch = createEventDispatcher();

  let assetUrlPromise: Promise<string | null> = getImageAsset(id!!);
</script>

<!--
@component
List representation of a credential.

### Props

- id
- title
- description (optional)
- type
- isTempAsset

### Slots

- right
-->
{#await assetUrlPromise then assetUrl}
  <button
    class="flex h-[64px] w-full items-center justify-start rounded-xl bg-white p-[7px] dark:bg-dark"
    on:click={() => dispatch('click')}
  >
    <!-- Image or icon -->
    <div
      class="mr-[15px] flex h-[50px] w-[50px] min-w-[50px] flex-col items-center justify-center overflow-hidden rounded-lg {assetUrl
        ? 'bg-white'
        : 'bg-silver dark:bg-navy'}"
    >
      <Image {id} iconFallback={type === 'data' ? 'User' : 'Certificate'} {isTempAsset} />
    </div>
    <!-- Text -->
    <div class="flex grow flex-col items-start overflow-x-auto text-left">
      <p class="line-clamp-2 w-full pr-[15px] text-[13px]/[18px] font-medium text-slate-800 dark:text-grey">
        {title}
      </p>
      {#if description}
        <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
          {description}
        </p>
      {/if}
    </div>
    <!-- Slot -->
    <slot name="right" />
  </button>
{/await}
