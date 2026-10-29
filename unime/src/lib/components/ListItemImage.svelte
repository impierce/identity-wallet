<script lang="ts">
  import { Image } from '$lib/components';
  import { ensureLightIcon } from '$lib/utils/image';

  export let id: string;
  export let type: 'data' | 'badge' = 'data';
  export let isTempAsset = false;
  export let icon: string | undefined = undefined;

  let useFallback = false;
</script>

<!--
@component
The image of a list entry: the logo for the given id, or an icon when there is none.

### Props
- id
- type: picks the fallback icon
- isTempAsset
- icon: overrides the fallback icon
-->
<div
  class={`flex h-12 w-12 min-w-[48px] items-center justify-center overflow-hidden rounded-lg p-1 ${useFallback ? 'bg-silver dark:bg-navy' : 'bg-white'}`}
>
  <!-- useFallback from <Image> (child) is bound to a local variable with the same name to determine which background color to display -->
  <Image
    {id}
    iconFallback={ensureLightIcon(icon) ?? (type === 'data' ? 'UserLight' : 'CertificateLight')}
    {isTempAsset}
    bind:useFallback
  />
</div>
