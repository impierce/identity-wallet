<script lang="ts">
  import { CredentialListEntry } from '@impierce/ui-components';
  import Star from '~icons/lucide/star';
  import Heart from '~icons/ph/heart-fill';
  import Clapperboard from '~icons/lucide/clapperboard';
  import FilmSlate from '~icons/ph/film-slate-light';
  import AirplaneTilt from '~icons/ph/airplane-tilt-light';
  import LL from '$i18n/i18n-svelte';

  let favorite_credentials: any[] = [
    {
      title: 'Avatar: The Way of Water',
      description: 'Downtown Cinema',
      icon: FilmSlate,
      color: 'bg-amber-100'
    },
    {
      title: 'Flight #1337 to Pandora',
      description: 'Pandora Airlines',
      icon: AirplaneTilt,
      color: 'bg-teal-100'
    },
    { title: 'something 1', description: 'make the list longer' },
    { title: 'something 2', description: 'make it even longer' }
  ];

  // favorite_credentials = [];
</script>

{#if favorite_credentials.length > 0}
  <div class="pb-6">
    <div class="flex items-center pb-2">
      <Heart class="mr-2 text-indigo-500" />
      <p class="font-medium text-slate-600">{$LL.FAVORITES()}</p>
    </div>
    <div class="space-y-2">
      {#each favorite_credentials as favorite}
        <CredentialListEntry
          title={favorite.title}
          description={favorite.description}
          color={favorite.color}
        >
          <span slot="icon"><svelte:component this={favorite.icon} class="h-6 w-6" /></span>
          <div slot="drawer-body">
            <div class="bg-orange-100 p-4">content</div>
          </div>
        </CredentialListEntry>
      {/each}
    </div>
  </div>
{/if}
