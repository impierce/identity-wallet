<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { Image } from '$lib/components';
  import { ensureLightIcon } from '$lib/utils/image';

  export let credential: DisplayCredential | undefined;
  export let fallbackId: string;

  let useFallback = false;

  $: isBadge = credential?.data?.type?.includes('OpenBadgeCredential') ?? false;
</script>

<!--
@component
The logo of a credential as shown in credential lists, with the same icon fallback.
-->
<div
  class="flex size-12 min-w-12 items-center justify-center overflow-hidden rounded-lg p-1 {useFallback
    ? 'bg-silver dark:bg-navy'
    : 'bg-white'}"
>
  <Image
    id={credential?.id ?? fallbackId}
    iconFallback={ensureLightIcon(credential?.metadata.icon) ?? (isBadge ? 'CertificateLight' : 'UserLight')}
    bind:useFallback
  />
</div>
