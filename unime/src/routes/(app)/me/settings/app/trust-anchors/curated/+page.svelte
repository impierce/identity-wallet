<script lang="ts">
  import { TopNavBar } from '$lib/components';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import TrustAnchorRow from '../TrustAnchorRow.svelte';

  const navigateUp = useParentRoute('/me/settings/app/trust-anchors');

  const anchors = $derived($appState.trust_anchors.anchors.filter((anchor) => anchor.source.type === 'curated'));
</script>

<TopNavBar on:back={navigateUp} title="Curated by Impierce" class="sticky top-0 z-10" />

<div class="flex flex-col gap-[10px] bg-silver px-4 py-5 dark:bg-navy">
  <p class="text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
    Impierce curates these anchors for interoperability testing. They come with the app and can be enabled and disabled,
    but not changed or removed.
  </p>
  {#each anchors as anchor (anchor.id)}
    <TrustAnchorRow {anchor} />
  {:else}
    <p class="py-4 text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      No curated anchors in this version.
    </p>
  {/each}
</div>
