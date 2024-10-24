<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import LL from '$i18n/i18n-svelte';

  import { IconMessage, ListItemCard } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { GhostFillIcon, MagnifyingGlassFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  import RecentSearches from './RecentSearches.svelte';
  import Search from './Search.svelte';

  let searchTerm: string | null = $page.url.searchParams.get('query');

  $: currentSearchResults = $state.search_results.current.map((id) => $state.credentials.find((c) => c.id === id)!);
  $: recentSearches = $state.search_results.recent_credentials.map(
    (id) => $state.credentials.find((c) => c.id === id)!,
  );

  // https://stackoverflow.com/questions/57354001/how-to-focus-on-input-field-loaded-from-component-in-svelte
  let searchInput: HTMLInputElement;
  onMount(() => {
    searchInput.focus();
  });

  function onSearchTermChanged(value: string) {
    searchTerm = value;
    $page.url.searchParams.set('query', value);
    history.replaceState(history.state, '', $page.url);
    dispatch({ type: '[Search] Query', payload: { search_term: value } });
  }
</script>

<div class="content-height bg-silver dark:bg-navy">
  <div class="p-4">
    <Search bind:ref={searchInput} value={searchTerm ?? ''} on:value={(e) => onSearchTermChanged(e.detail)}></Search>
  </div>
  <!-- User has not entered a search term -->
  {#if !searchTerm}
    {#if recentSearches.length > 0}
      <RecentSearches {recentSearches} />
    {:else}
      <div class="pt-12">
        <IconMessage
          icon={MagnifyingGlassFillIcon}
          title={$LL.SEARCH.NO_QUERY.TITLE()}
          description={$LL.SEARCH.NO_QUERY.DESCRIPTION()}
        />
      </div>
    {/if}
    <!-- User has entered something, but there are no results -->
  {:else if currentSearchResults.length == 0}
    <div class="pt-12">
      <IconMessage
        icon={GhostFillIcon}
        title={$LL.SEARCH.NO_RESULTS.TITLE()}
        description={$LL.SEARCH.NO_RESULTS.DESCRIPTION()}
      />
    </div>
    <!-- User has entered something and there are results. -->
    <!-- Note: We're doing the if/else checks before to prevent "flashing empty results" before the content has loaded. -->
  {:else}
    <div class="w-full space-y-2 p-5">
      <!-- Using "key" to destroy & recreate the complete credentials list to enforce a refresh of logos -->
      {#key $state.search_results}
        {#each currentSearchResults as credential}
          <ListItemCard
            id={credential.id}
            title={credential.display_name}
            description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
            type={credential.data?.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
            on:click={() => {
              dispatch({ type: '[Search] Add recent', payload: { id: credential.id } });
              if (credential.data?.type.includes('OpenBadgeCredential')) {
                goto(`/badges/${credential.id}`);
              } else {
                goto(`/credentials/${credential.id}`);
              }
            }}
          />
        {/each}
      {/key}
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px);
  }
</style>
