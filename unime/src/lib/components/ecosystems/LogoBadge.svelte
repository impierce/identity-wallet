<script lang="ts">
  import { twMerge } from 'tailwind-merge';

  import { hash } from '$lib/utils';

  import Image from '../image/Image.svelte';

  export let logoUri: string | null | undefined = undefined;
  export let initials: string;
  // A whole `bg-*` class string. May carry a `text-*` with it, which the monogram inherits: a pale
  // tint needs dark initials. Defaults to white below, which suits the dark tints.
  export let tint: string;
  export let isTempAsset: boolean | 'fallback' = true;

  let className = '';
  export { className as class };
  export let textClass = 'text-[18px]/[24px]';

  // The backend writes logos to `assets/tmp/<hash(url)>`.
  $: imageId = logoUri ? hash(logoUri) : undefined;

  // <Image> reports its fallback, so the tint drops behind a real logo instead of colouring
  // its transparent edges.
  let useFallback = false;

  $: showBadge = !imageId || useFallback;
</script>

<!--
@component
Logo when the backend has one on disk, tinted monogram when it does not. Callers decide what
the monogram says and which tint it takes.

### Props
- logoUri (optional)
- initials
- tint
- class (_sizing and rounding_)
- textClass (_default_: `'text-[18px]/[24px]'`)
-->
<div
  class={twMerge(
    'flex shrink-0 items-center justify-center overflow-hidden text-white',
    showBadge ? tint : 'bg-white',
    className,
  )}
>
  {#if imageId}
    <!-- Keyed, since <Image> loads only once: a refresh may change the logo. -->
    {#key imageId}
      <Image id={imageId} {isTempAsset} bind:useFallback imgClass="size-full object-contain">
        <span slot="fallback" class={twMerge('font-semibold', textClass)}>{initials}</span>
      </Image>
    {/key}
  {:else}
    <span class={twMerge('font-semibold', textClass)}>{initials}</span>
  {/if}
</div>
