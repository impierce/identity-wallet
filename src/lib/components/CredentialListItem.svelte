<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import Checkbox from '$lib/components/atoms/Checkbox.svelte';
  import Image from '$lib/components/Image.svelte';
  import { getImageAsset } from '$lib/utils';

  export let id: string | undefined = undefined; // TODO: should not be able to be undefined
  export let title = '';
  export let description = '';
  export let type: 'data' | 'badge' = 'data';

  // TODO: use a generic slot instead?
  export let hasCheckbox = false;
  export let isChecked = false;
  export let isDisabled = false;
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
- description
- type

### Slots

- action
-->
{#await assetUrlPromise then assetUrl}
  <button
    class="flex h-[64px] w-full items-center justify-start rounded-xl bg-white p-[7px] dark:bg-dark"
    on:click={() => dispatch('click')}
  >
    <!-- Icon -->
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
      <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
        {description}
      </p>
    </div>
    <!-- Checkbox (optional) -->
    {#if hasCheckbox}
      <div class="mr-2">
        <!-- <Checkbox checked={$isChecked('system')} /> -->
        <Checkbox checked={isChecked} disabled={isDisabled} />
      </div>
    {/if}
  </button>
{/await}
