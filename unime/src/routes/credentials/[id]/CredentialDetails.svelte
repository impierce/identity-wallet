<script lang="ts">
  import { onMount } from 'svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { dispatch } from '$lib/dispatcher';

  import CredentialClaimsRenderer from './CredentialClaimsRenderer.svelte';
  import CredentialEcosystems from './CredentialEcosystems.svelte';
  import CredentialHeader from './CredentialHeader.svelte';
  import CredentialOverview from './CredentialOverview.svelte';

  export let credential: DisplayCredential;
  // Hides the favorite button and the menu, e.g. when the credential is inspected while sharing.
  export let actions = true;

  // What is on screen may be outdated while this is set.
  let refreshing = false;

  // Status and ecosystems are fetched from the issuer's side each time a credential is opened, and on request.
  async function refresh() {
    refreshing = true;
    try {
      await Promise.all([
        dispatch({ type: '[Credential] Refresh status', payload: { credential_id: credential.id } }),
        dispatch({ type: '[Credential] Refresh ecosystems', payload: { credential_id: credential.id } }),
      ]);
    } finally {
      refreshing = false;
    }
  }

  onMount(refresh);
</script>

<!--
@component
The details of a credential: its header with the title in the default slot, its validity and issuer, the ecosystems
endorsing its issuer, and its claims. Opening it refreshes its status and ecosystems.
Used by the credential page and when inspecting a credential while sharing.

### Props
- credential
- actions

### Slots
- default: the title
- header-actions: content placed right after the header, e.g. the edit sheet
-->
<div class="flex min-h-full flex-col gap-7 bg-background-alt px-4 pb-7">
  <CredentialHeader {credential} {actions} on:edit on:showRawData>
    <slot />
  </CredentialHeader>
  <slot name="header-actions" />
  <CredentialOverview {credential} />
  <CredentialEcosystems {credential} {refreshing} on:refresh={refresh} />
  <CredentialClaimsRenderer {credential} />
</div>
