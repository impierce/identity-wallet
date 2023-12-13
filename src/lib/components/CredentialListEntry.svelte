<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';

  import { getImageAsset } from '../connections/utils';

  export let id: string | undefined = undefined; // TODO: should not be able to be undefined
  export let title = '';
  export let description = '';
  export let color: string | undefined = undefined;
  export let type: 'data' | 'badge' = 'data';

  let assetUrl = '';

  onMount(async () => {
    assetUrl = await getImageAsset(id!!);
  });
</script>

<!--
@component
List representation of a credential. Input parameters are:

- title
- description
- icon
- color

-->
<button
  class="flex h-[64px] items-center justify-start rounded-xl bg-white p-[7px] pr-[24px] dark:bg-dark"
  on:click={() => (type == 'data' ? goto(`/credentials/${id}`) : goto(`/badges/${id}`))}
>
  <!-- Icon -->
  <div class="mr-[15px] w-[50px] {color} flex h-[50px] flex-col items-center justify-center overflow-hidden rounded-lg">
    {#if !assetUrl}
      <slot name="icon" />
    {:else}
      <img
        src={assetUrl}
        alt="icon"
        class="w-full bg-white object-cover dark:bg-dark"
        on:error={() => (assetUrl = '')}
      />
    {/if}
  </div>
  <!-- Text -->
  <div class="flex grow flex-col items-start overflow-x-auto text-left">
    <p class="w-full truncate text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
      {title}
    </p>
    <p class="max-w-[180px] truncate text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
      {description}
    </p>
  </div>
</button>
