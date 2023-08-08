<script lang="ts">
  import { CredentialListEntry } from '@impierce/ui-components';
  import Star from '~icons/lucide/star';
  import Heart from '~icons/ph/heart-fill';
  import Clapperboard from '~icons/lucide/clapperboard';
  import FilmSlate from '~icons/ph/film-slate-light';
  import LL from '$i18n/i18n-svelte';

  let favorite_credentials: any[] = [
    {
      title: 'Avatar: The Way of Water',
      description: 'Downtown Cinema'
    }
  ];
</script>

{#if favorite_credentials.length > 0}
  <div class="pb-6">
    <div class="flex items-center pb-2">
      <Heart class="mr-2 text-indigo-500" />
      <p class="font-medium text-slate-600">{$LL.FAVORITES()}</p>
    </div>
    <!-- <div class="pt-2"> -->
    {#each favorite_credentials as favorite}
      <CredentialListEntry title={favorite.title} description={favorite.description} color="bg-amber-100">
        <span slot="icon"><FilmSlate class="h-6 w-6" /></span>
        <div slot="drawer-body">
          <div class="bg-orange-100 p-4">content</div>
        </div>
      </CredentialListEntry>
    {/each}
  </div>
{/if}
