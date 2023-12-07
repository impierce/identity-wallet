<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';

  import { appDataDir, join } from '@tauri-apps/api/path';
  import { convertFileSrc } from '@tauri-apps/api/primitives';

  export let id: string | undefined = undefined; // TODO: should not be able to be undefined
  export let title = '';
  export let description = '';
  export let color: string | undefined = undefined;

  $: assetUrl = undefined;

  onMount(async () => {
    const appDataDirPath = await appDataDir();

    const filePath = await join(appDataDirPath, `assets/${id}.svg`);
    // console.log({ filePath });
    assetUrl = convertFileSrc(filePath);
    console.log({ assetUrl });
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
  on:click={() => goto(`/credentials/${id}`)}
>
  <!-- Icon -->
  <div class="mr-[15px] w-[50px] {color} flex h-[50px] flex-col items-center justify-center overflow-hidden rounded-lg">
    <!-- <slot name="icon" /> -->
    <img src={assetUrl} alt="icon" class="w-full object-cover" />
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
