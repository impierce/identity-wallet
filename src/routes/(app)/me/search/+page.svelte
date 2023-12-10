<script lang="ts">
  import { colors, icons } from '$lib/credentials/customization/utils';
  import CredentialListEntry from '$src/lib/components/CredentialListEntry.svelte';
  import NoMatch from '$src/lib/components/molecules/NoMatch.svelte';
  import NoQuery from '$src/lib/components/molecules/NoQuery.svelte';
  import Search from '$src/lib/components/Search.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let searchTerm: string | undefined;
  $: indices = $state.user_data_query;
  $: credentials = $state.credentials.filter((cred) => indices.includes(cred.id));

  // TODO: known issue regarding reactive variables:
  // quick "flash" of all credentials before filter is applied and correct results are shown
  // further investigation: redefine variables? adjust if/else in HTML?
</script>

<div class="content-height bg-silver dark:bg-navy">
  <div class="p-4">
    <Search
      on:value={(e) => {
        searchTerm = e.detail;
        dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', search_term: e.detail } });
      }}
    ></Search>
  </div>
  {#if !searchTerm}
    <div class="pt-12">
      <NoQuery />
    </div>
  {:else if credentials.length == 0}
    <div class="pt-12">
      <NoMatch />
    </div>
  {:else}
    <div class="w-full space-y-2 p-5">
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
    </div>
  {/if}
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
