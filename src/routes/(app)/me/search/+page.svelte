<script lang="ts">

import { colors, icons } from '$lib/credentials/customization/utils';
  import Favorites from '$src/lib/Favorites.svelte';
  import Search from '$src/lib/components/Search.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import NoQuery from '$src/lib/credentials/NoQuery.svelte';
  import NoMatch from '$src/lib/credentials/NoMatch.svelte';
  import { state } from '$src/stores';
  import CredentialListEntry from '$src/lib/components/CredentialListEntry.svelte';
  let searchTerm: string | undefined;
  $: indices = $state.user_data_query;
  $: credentials = $state.credentials.filter((cred) => indices.includes(cred.id));
</script>

<div class="content-height dark:bg-navy">
  <Search
    on:value={(e) => {
      searchTerm = e.detail;
      console.log(e);
      dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', search_term: e.detail } });
    }}
  ></Search>
  {#if !searchTerm}
    <NoQuery />
  {:else if credentials.length == 0}
    <NoMatch />
  {:else}
    {#each credentials as credential}
      <CredentialListEntry
        id={credential.id}
        title={credential.metadata.display.name || credential.data.type.at(-1)}
        description={credential.data.issuer}
        color={credential.metadata.display.color ||
          colors.at(
            credential.id
              .match(/[0-9]+/)
              .at(0)
              .at(0) % 8, // TODO: omits last value (white)
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
  {/if}
</div>

<!--Show search results?-->

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
