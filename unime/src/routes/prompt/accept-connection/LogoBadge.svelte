<script lang="ts">
  import { twMerge } from 'tailwind-merge';

  import { Image } from '$lib/components';
  import { hash } from '$lib/utils';

  export let logoUri: string | null | undefined = undefined;
  export let initials: string;
  // A whole `bg-*` class string.
  export let tint: string;

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
  class={twMerge('flex shrink-0 items-center justify-center overflow-hidden', showBadge ? tint : 'bg-white', className)}
>
  {#if imageId}
    <Image id={imageId} isTempAsset={true} bind:useFallback imgClass="size-full object-contain">
      <span slot="fallback" class={twMerge('font-semibold text-white', textClass)}>{initials}</span>
    </Image>
  {:else}
    <span class={twMerge('font-semibold text-white', textClass)}>{initials}</span>
  {/if}
</div>
