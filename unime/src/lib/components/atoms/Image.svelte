<script lang="ts">
  import { onMount } from 'svelte';

  import { twMerge } from 'tailwind-merge';

  import { debug, warn } from '@tauri-apps/plugin-log';

  import { BankLightIcon, CertificateLightIcon, UserLightIcon } from '$lib/icons';
  import { getImageAsset } from '$lib/utils';

  const icons = {
    Bank: BankLightIcon,
    Certificate: CertificateLightIcon,
    User: UserLightIcon,
  };
  type Icon = keyof typeof icons;

  export let id: string;
  export let iconFallback: Icon = 'User';
  export let isTempAsset = false;
  let assetUrl: string | null = null;

  async function loadImage() {
    getImageAsset(id, isTempAsset).then((url) => {
      assetUrl = url;
    });
  }

  onMount(() => {
    loadImage();
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
    on:error={() => warn(`Could not load image: ${id}`)}
    on:load={() => debug(`Image successfully loaded: ${assetUrl}`)}
    data-testid="image"
  />
{:else}
  <slot name="fallback">
    <svelte:component
      this={icons[iconFallback]}
      class={twMerge('h-[18px] w-[18px] text-slate-800 dark:text-grey', $$props.iconClass)}
      data-testid="icon"
    />
  </slot>
{/if}
