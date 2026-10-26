<script lang="ts">
  import { onMount } from 'svelte';

  import { twMerge } from 'tailwind-merge';

  import { debug, warn } from '@tauri-apps/plugin-log';

  import {
    BankLightIcon,
    BankRegularIcon,
    CertificateLightIcon,
    HouseLightIcon,
    HouseRegularIcon,
    IdentificationBadgeLightIcon,
    IdentificationBadgeRegularIcon,
    UserLightIcon,
  } from '$lib/icons';
  import { getImageAsset } from '$lib/utils';

  const icons = {
    Bank: BankRegularIcon,
    BankLight: BankLightIcon,
    CertificateLight: CertificateLightIcon,
    UserLight: UserLightIcon,
    IdentificationBadge: IdentificationBadgeRegularIcon,
    IdentificationBadgeLight: IdentificationBadgeLightIcon,
    House: HouseRegularIcon,
    HouseLight: HouseLightIcon,
  };
  type Icon = keyof typeof icons;

  export let id: string;
  export let iconFallback: Icon = 'UserLight';
  export let isTempAsset = false;
  let assetUrl: string | null = null;

  // useFallback is reactive since the assetUrl is updated onMount when the image is loaded from the backend
  export let useFallback: boolean = false;
  $: useFallback = assetUrl === null;

  export let isSquare: boolean = false;

  function onLoad(event: Event) {
    const { naturalWidth, naturalHeight } = event.currentTarget as HTMLImageElement;
    isSquare = naturalHeight > 0 && Math.abs(naturalWidth / naturalHeight - 1) < 0.1;
    debug(`Image successfully loaded: ${assetUrl}`);
  }

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
- iconFallback (_default_: `'UserLight'`)
- isTempAsset (_default_: `false`)
- isSquare (_bindable_, `true` once a loaded image has a square aspect ratio)
- imgClass (_overwrite Tailwind classes, default size_: `100%`)
- iconClass (_overwrite Tailwind classes, default size_: `18px`)

### Slots
- fallback (_default_: `UserLight` icon)

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
    on:load={onLoad}
    data-testid="image"
  />
{:else}
  <slot name="fallback">
    <svelte:component
      this={icons[iconFallback]}
      class={twMerge('size-[18px] text-slate-800 dark:text-grey', $$props.iconClass)}
      data-testid="icon"
    />
  </slot>
{/if}
