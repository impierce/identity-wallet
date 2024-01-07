<script lang="ts">
  import { goto } from '$app/navigation';

  import { colors, icons } from '$lib/credentials/customization/utils';
  import CredentialListItem from '$src/lib/components/CredentialListItem.svelte';
  import NoMatch from '$src/lib/components/molecules/NoMatch.svelte';
  import NoQuery from '$src/lib/components/molecules/NoQuery.svelte';
  import Search from '$src/lib/components/Search.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let searchTerm: string | undefined;
  $: indices = $state.user_data_query;
  $: credentials = $state.credentials.filter((cred) => indices.includes(cred.id));
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
      <!-- using "key" to destroy & recreate the complete credentials list to enforce a refresh of logos -->
      {#key indices}
        {#each credentials as credential}
          <!-- <CredentialListItem
            id={credential.id}
            title={credential.metadata.display.name ??
              credential.data.credentialSubject.achievement?.name ??
              credential.data.type.at(-1)}
            description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
            type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
          >
          </CredentialListItem> -->

          <CredentialListItem
            id={credential.id}
            title={credential.metadata.display.name ??
              credential.data.credentialSubject.achievement?.name ??
              credential.data.type.at(-1)}
            description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
            type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
            on:click={() =>
              credential.data.type.includes('OpenBadgeCredential')
                ? goto(`/badges/${credential.id}`)
                : goto(`/credentials/${credential.id}`)}
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
