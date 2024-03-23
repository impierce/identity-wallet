<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/stores';

  import IconMessage from '$lib/components/molecules/IconMessage.svelte';
  import Search from '$lib/components/molecules/Search.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';

  import Ghost from '~icons/ph/ghost-fill';
  import MagnifyingGlass from '~icons/ph/magnifying-glass-fill';

  import RecentSearches from './RecentSearches.svelte';

  let searchTerm: string | null = $page.url.searchParams.get('query');

  $: indices = $state.search_results;
  $: credentials = $state.credentials.filter((cred) => indices?.current.includes(cred.id));
  $: recentSearches = $state.credentials.filter((cred) => indices?.recent_credentials.includes(cred.id));

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
          icon={MagnifyingGlass}
          title={$LL.SEARCH.NO_QUERY.TITLE()}
          description={$LL.SEARCH.NO_QUERY.DESCRIPTION()}
        />
      </div>
    {/if}
    <!-- User has entered something, but there are no results -->
  {:else if credentials.length == 0}
    <div class="pt-12">
      <IconMessage
        icon={Ghost}
        title={$LL.SEARCH.NO_RESULTS.TITLE()}
        description={$LL.SEARCH.NO_RESULTS.DESCRIPTION()}
      />
    </div>
    <!-- User has entered something and there are results.
      Note: We're doing the if/else checks before to prevent "flashing empty results" before the content has loaded. -->
  {:else}
    <div class="w-full space-y-2 p-5">
      <!-- using "key" to destroy & recreate the complete credentials list to enforce a refresh of logos -->
      {#key indices}
        {#each credentials as credential}
          <ListItemCard
            id={credential.id}
            title={credential.display_name}
            description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
            type={credential.data?.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
            on:click={() => {
              dispatch({ type: '[Search] Add Recent', payload: { id: credential.id } });
              credential.data?.type.includes('OpenBadgeCredential')
                ? goto(`/badges/${credential.id}`)
                : goto(`/credentials/${credential.id}`);
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
