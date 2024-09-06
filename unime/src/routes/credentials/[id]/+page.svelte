<script lang="ts">
  import { page } from '$app/stores';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { pageTitleStore, state } from '$lib/stores';

  import CredentialHeader from './CredentialHeader.svelte';
  import CredentialOverview from './CredentialOverview.svelte';
  import DefaultRenderer from './DefaultRenderer.svelte';
  import OpenBadgeRenderer from './OpenBadgeRenderer.svelte';

  // Credential cannot be loaded via load function since it's stored in the application state.
  // TODO Credential should be loaded from backend via load function to handle invalid IDs properly.
  function loadCredential() {
    return $state.credentials.find((c) => $page.params.id === c.id);
  }

  let credential: DisplayCredential | undefined = loadCredential();

  // The credential's metadata, e.g., the favorite status, may change.
  // ESLint does not understand the reactive statement.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  $: $state, (credential = loadCredential());

  $: {
    // Use `pageTitleStore` to get page title into layout.
    pageTitleStore.set($LL.CREDENTIAL.NAVBAR_TITLE());
  }
</script>

{#if credential}
  <div class="flex flex-col gap-7 px-4 pb-7">
    <CredentialHeader {credential}>
      <h1 class="text-center font-semibold">
        {credential.data?.credentialSubject?.achievement?.name ?? credential.display_name}
      </h1>
    </CredentialHeader>
    <CredentialOverview {credential} />
    {#if credential.data?.credentialSubject?.achievement}
      <OpenBadgeRenderer {credential} />
    {:else}
      <DefaultRenderer {credential} />
    {/if}
  </div>
{/if}
