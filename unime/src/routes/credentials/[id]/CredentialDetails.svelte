<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import CredentialClaimsRenderer from './CredentialClaimsRenderer.svelte';
  import CredentialHeader from './CredentialHeader.svelte';
  import CredentialOverview from './CredentialOverview.svelte';

  export let credential: DisplayCredential;
  // Hides the favorite button and the menu, e.g. when the credential is inspected while sharing.
  export let actions = true;
</script>

<!--
@component
The details of a credential: its header with the title in the default slot, its validity and issuer, and its claims.
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
  <CredentialClaimsRenderer {credential} />
</div>
