<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import Clapperboard from '~icons/lucide/clapperboard';
  import Star from '~icons/lucide/star';
  import AirplaneTilt from '~icons/ph/airplane-tilt-light';
  import FilmSlate from '~icons/ph/film-slate-light';
  import Heart from '~icons/ph/heart-straight-fill';
  import User from '~icons/ph/user';

  import type { DisplayCredential } from '../../src-tauri/bindings/display-credential/DisplayCredential';
  import CredentialListEntry from './components/CredentialListEntry.svelte';
  import { colors, icons } from './credentials/customization/utils';

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

  const favorite_credentials: DisplayCredential[] = $state.credentials.filter(
    (c) => c.metadata.is_favorite
  );
</script>

{#if favorite_credentials.length > 0}
  <div class="pb-6">
    <div class="flex items-center pb-2">
      <Heart class="mr-2 text-primary" />
      <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-white">{$LL.FAVORITES()}</p>
    </div>
    <div class="flex flex-col space-y-2">
      {#each favorite_credentials as credential}
        <CredentialListEntry
          id={credential.id}
          title={credential.metadata.display.name || credential.data.type.at(-1)}
          description={credential.data.issuer}
          color={credential.metadata.display.color ||
            colors.at(
              credential.id
                .match(/[0-9]+/)
                .at(0)
                .at(0) % 8 // TODO: omits last value (white)
            )}
        >
          <span slot="icon">
            <svelte:component
              this={icons[credential.metadata.display.icon] || icons['User']}
              class="h-[18px] w-[18px] text-slate-800"
            />
          </span>
        </CredentialListEntry>
      {/each}
    </div>
  </div>
{/if}
