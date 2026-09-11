<script lang="ts">
  import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';

  import { hashIndex } from '$lib/utils';

  import LogoBadge from './LogoBadge.svelte';

  export let certification: LinkedVerifiableCredentialData;

  let className = '';
  export { className as class };
  export let textClass = 'text-[18px]/[24px]';

  // Whole class strings: Tailwind only emits classes it finds literally in the source.
  const TINTS = ['bg-primary', 'bg-slate-700', 'bg-slate-400'];

  $: logoUri = certification.credential.metadata.icon ?? certification.credential.issuer_logo_uri;

  $: name = certification.credential.display_name;

  // "ISO 27001 Certified" -> "IS". The standard leads the name, so the first word identifies it;
  // one letter per word would give "IC".
  $: initials = (name.match(/[\p{L}\p{N}]+/u)?.[0] ?? '?').slice(0, 2).toUpperCase();

  $: tint = TINTS[hashIndex(name, TINTS.length)];
</script>

<!--
@component
The mark of a single certification.

### Props
- certification
- class (_sizing and rounding_)
- textClass (_default_: `'text-[18px]/[24px]'`)
-->
<LogoBadge {logoUri} {initials} {tint} class={className} {textClass} />
