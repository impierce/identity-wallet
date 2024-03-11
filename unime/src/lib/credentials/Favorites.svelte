<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';

  import type { DisplayCredential } from '@bindings/display-credential/DisplayCredential';

  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import Heart from '~icons/ph/heart-straight-fill';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  // let favorite_credentials: any[] = [
  //   {
  //     title: 'Avatar: The Way of Water',
  //     description: 'Downtown Cinema',
  //     icon: FilmSlate,
  //     color: 'bg-amber-100'
  //   },
  //   {
  //     title: 'Flight #1337 to Pandora',
  //     description: 'Pandora Airlines',
  //     icon: AirplaneTilt,
  //     color: 'bg-teal-100'
  //   }
  // ];

  // favorite_credentials = [];

  let favorite_credentials: DisplayCredential[] = $state.credentials.filter((c) => c.metadata.is_favorite);

  onMount(async () => {
    if (credentialType === 'badges') {
      favorite_credentials = favorite_credentials.filter((c) =>
        (c.data.type as string[]).includes('OpenBadgeCredential'),
      );
    } else if (credentialType === 'data') {
      favorite_credentials = favorite_credentials.filter(
        (c) => !(c.data.type as string[]).includes('OpenBadgeCredential'),
      );
    }
  });
</script>

{#if favorite_credentials.length > 0}
  <div class="flex items-center pb-2">
    <Heart class="mr-2 text-primary" />
    <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-white">{$LL.ME.FAVORITES()}</p>
  </div>
  <div class="flex flex-col space-y-2">
    {#each favorite_credentials as credential}
      <ListItemCard
        id={credential.id}
        title={credential.display_name}
        description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
        type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
        on:click={() =>
          credential.data.type.includes('OpenBadgeCredential')
            ? goto(`/badges/${credential.id}`)
            : goto(`/credentials/${credential.id}`)}
      />
    {/each}
  </div>
  <!-- TODO: make conditional? only show when there are also some non-favorite credentials -->
  <!-- Horizontal line -->
  <div class="my-5 h-0.5 bg-grey dark:bg-blue"></div>
{/if}
