<script lang="ts">
  import type { DisplayCredential } from '@bindings/display-credential/DisplayCredential';

  import LL from '$src/i18n/i18n-svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';

  import X from '~icons/ph/x-bold';

  export let recentSearches: DisplayCredential[] = [];
</script>

{#each recentSearches as recentSearch}
  <div class="p-5">
    <p class="pb-[10px] text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SEARCH.RECENT_SEARCHES()}
    </p>
    <ListItemCard id={recentSearch.id} title={recentSearch.display_name} description={recentSearch.issuer_name}>
      <!-- TODO: dispatch action: remove recent search -->
      <button slot="right" class="mr-1 rounded-full p-3 hover:bg-silver dark:hover:bg-navy" on:click={() => {}}>
        <X class="h-4 w-4 text-slate-800 dark:text-grey" />
      </button>
    </ListItemCard>
  </div>
{/each}
