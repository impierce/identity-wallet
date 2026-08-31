<script lang="ts">
  import { twMerge } from 'tailwind-merge';

  import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';

  import { Image } from '$lib/components';
  import { hash } from '$lib/utils';

  export let certification: LinkedVerifiableCredentialData;

  let className = '';
  export { className as class };
  // Sized by the caller: the initials have to shrink along with the badge.
  export let textClass = 'text-[18px]/[24px]';

  // Whole class strings, never assembled from fragments: Tailwind only emits classes it can
  // find literally in the source.
  const TINTS = ['bg-primary', 'bg-slate-700', 'bg-slate-400'];

  $: logoUri = certification.credential.metadata.icon ?? certification.credential.issuer_logo_uri;
  $: imageId = logoUri ? hash(logoUri) : undefined;

  $: name = certification.credential.display_name;

  // "ISO 27001 Certified" -> "IS". Certification names lead with the standard they attest to, so
  // the first word is the identifying part; taking one letter per word would give "IC".
  $: initials = (name.match(/[\p{L}\p{N}]+/u)?.[0] ?? '?').slice(0, 2).toUpperCase();

  // Derived from the name so a certification keeps the same colour wherever it appears, and so a
  // stack of badges reads as several distinct issuers rather than one repeated.
  $: tint = TINTS[[...name].reduce((sum, character) => sum + character.charCodeAt(0), 0) % TINTS.length];

  // <Image> reports whether it fell back to the slot, so the badge can drop its tint behind a
  // real logo rather than colouring the transparent edges of it.
  let useFallback = false;

  $: showBadge = !imageId || useFallback;
</script>

<!--
@component
The mark of a single certification: the issuer's logo when the backend has one on disk, a
tinted monogram of the certification's name when it does not.

### Props
- certification
- class (_sizing and rounding, the caller owns both_)
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
