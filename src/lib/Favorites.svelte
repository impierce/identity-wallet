<script lang="ts">
  import Star from '~icons/lucide/star';
  import Heart from '~icons/ph/heart-straight-fill';
  import Clapperboard from '~icons/lucide/clapperboard';
  import FilmSlate from '~icons/ph/film-slate-light';
  import AirplaneTilt from '~icons/ph/airplane-tilt-light';
  import LL from '$src/i18n/i18n-svelte';
  import CredentialListEntry from './components/CredentialListEntry.svelte';

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
    }
  ];

  favorite_credentials = [];
</script>

{#if favorite_credentials.length > 0}
  <div class="pb-6">
    <div class="flex items-center pb-2">
      <Heart class="mr-2 text-indigo-500" />
      <p class="font-medium text-slate-600">{$LL.FAVORITES()}</p>
    </div>
    <div class="flex flex-col space-y-2">
      {#each favorite_credentials as favorite}
        <CredentialListEntry
          title={favorite.title}
          description={favorite.description}
          color={favorite.color}
        >
          <span slot="icon"><svelte:component this={favorite.icon} class="h-6 w-6" /></span>
        </CredentialListEntry>
      {/each}
    </div>
  </div>
{/if}
