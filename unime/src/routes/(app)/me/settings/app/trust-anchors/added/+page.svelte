<script lang="ts">
  import { goto } from '$app/navigation';

  import { Button, TopNavBar } from '$lib/components';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import TrustAnchorRow from '../TrustAnchorRow.svelte';

  const navigateUp = useParentRoute('/me/settings/app/trust-anchors');

  const anchors = $derived($appState.trust_anchors.anchors.filter((anchor) => anchor.source.type === 'added'));
</script>

<TopNavBar on:back={navigateUp} title="Added by you" class="sticky top-0 z-10" />

<div class="flex flex-col gap-[10px] bg-silver px-4 py-5 dark:bg-navy">
  {#each anchors as anchor (anchor.id)}
    <TrustAnchorRow {anchor} />
  {:else}
    <p class="py-4 text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      You haven't added any anchors.
    </p>
  {/each}
  <Button label="Add trust anchor" on:click={() => goto('/me/settings/app/trust-anchors/add')} />
</div>
