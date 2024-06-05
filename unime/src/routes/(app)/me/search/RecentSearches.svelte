<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import { dispatch } from '$lib/dispatcher';

  import X from '~icons/ph/x-bold';

  export let recentSearches: DisplayCredential[] = [];
</script>

<div class="p-5">
  <p class="pb-[10px] text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
    {$LL.SEARCH.RECENT_SEARCHES()}
  </p>
  <div class="space-y-[10px]">
    <!-- using "key" to destroy & recreate the complete credentials list to enforce a refresh of logos -->
    {#key recentSearches}
      {#each recentSearches as recentSearch}
        <ListItemCard
          id={recentSearch.id}
          title={recentSearch.display_name}
          description={recentSearch.issuer_name}
          on:click={() => {
            dispatch({ type: '[Search] Add recent', payload: { id: recentSearch.id } });
            recentSearch.data.type.includes('OpenBadgeCredential')
              ? goto(`/badges/${recentSearch.id}`)
              : goto(`/credentials/${recentSearch.id}`);
          }}
        >
          <button
            slot="right"
            class="mr-1 rounded-full p-3 hover:bg-silver dark:hover:bg-navy"
            on:click|stopPropagation={() => {
              dispatch({ type: '[Search] Delete Recent', payload: { id: recentSearch.id } });
            }}
          >
            <X class="h-4 w-4 text-slate-800 dark:text-grey" />
          </button>
        </ListItemCard>
      {/each}
    {/key}
  </div>
</div>
