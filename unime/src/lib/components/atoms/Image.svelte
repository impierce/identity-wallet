<script lang="ts">
  import { onMount, SvelteComponent } from 'svelte';

  import { twMerge } from 'tailwind-merge';

  import { debug, warn } from '@tauri-apps/plugin-log';

  import { getImageAsset } from '$lib/utils';

  import User from '~icons/ph/user-light';

  export let id: string | null = null;

  export let isTempAsset: boolean = false;

  let assetUrlPromise: Promise<string | null> = getImageAsset(id!!);

  let assetUrl: string | null = null;

  let fallbackComponent: SvelteComponent | null = null;

  // async loading function?: getImageAsset

  async function loadImage() {
    getImageAsset(id!!, isTempAsset).then((a) => {
      assetUrl = a;
    });
  }

  // TODO: needed?
  let loading = false;

  onMount(() => {
    // getImageAsset
    loadImage();
    loading = true;
  });
</script>

<!--
@component
Displays an image (loaded from disk) or a fallback component.

### Props
- id
- iconFallback (_default_: `'User'`)
- isTempAsset (_default_: `false`)
- imgClass (_overwrite Tailwind classes, default size_: `100%`)
- iconClass (_overwrite Tailwind classes, default size_: `18px`)

### Slots
- fallback (_default_: `User` icon)

### Usage
```tsx
<Image id={'3cf73ecb'} iconFallback={'Bank'} imgClass="h-[64px] w-[64px]" />
```
-->
{#if assetUrl}
  <img
    src={assetUrl}
    alt="img_{id}"
    class={twMerge('max-h-full w-full overflow-hidden bg-white object-contain', $$props.imgClass)}
    on:error={() => {
      id = null;
      warn(`could not load image: ${id}`);
    }}
    on:load={() => debug(`image successfully loaded: ${assetUrl}`)}
    data-testid="image"
  />
{:else}
  <slot name="fallback">
    <svelte:component
      this={User}
      class={twMerge('h-[18px] w-[18px] text-slate-800 dark:text-grey', $$props.iconClass)}
      data-testid="icon"
    />
  </slot>
{/if}
