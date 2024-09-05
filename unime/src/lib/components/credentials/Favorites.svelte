<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { ListItemCard } from '$lib/components';
  import { HeartStraightFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  // after filtering, favorite_credentials becomes reactive and gets sorted as well
  let favorite_credentials: DisplayCredential[];

  $: {
    // Filter out non-favorites, then filter by type (if applicable).
    favorite_credentials = $state.credentials.filter((c) => c.metadata.is_favorite);
    if (credentialType === 'badges') {
      favorite_credentials = favorite_credentials.filter((c) =>
        (c.data.type as string[]).includes('OpenBadgeCredential'),
      );
    } else if (credentialType === 'data') {
      favorite_credentials = favorite_credentials.filter(
        (c) => !(c.data.type as string[]).includes('OpenBadgeCredential'),
      );
    }
  }
</script>

{#if favorite_credentials.length > 0}
  <div class="flex items-center pb-2">
    <HeartStraightFillIcon class="mr-2 text-primary" />
    <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-white">{$LL.ME.FAVORITES()}</p>
  </div>
  <div class="flex flex-col space-y-2">
    {#each favorite_credentials as credential (credential.id)}
      <ListItemCard
        id={credential.id}
        title={credential.display_name}
        description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
        type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
        on:click={() => goto(`/credential/${credential.id}`)}
      />
    {/each}
  </div>
  <!-- TODO: make conditional? only show when there are also some non-favorite credentials -->
  <!-- Horizontal line -->
  <div class="my-5 h-0.5 bg-grey dark:bg-blue"></div>
{/if}
