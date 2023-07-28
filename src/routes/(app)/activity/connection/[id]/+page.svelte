<script lang="ts">
  import { TopNavigation } from '@impierce/ui-components';
  import ConnectionDetails from '$lib/connections/ConnectionDetails.svelte';
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
</script>

<div>
  <TopNavigation on:back={() => goto('/activity')} />
    <div class="p-4">
  <div class="font-mono">id: {$page.params.id}</div>
  <ConnectionDetails />
</div>
</div>
