<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import ClaimRenderer from './(renderers)/ClaimRenderer.svelte';
  import { getDisplayClaimKey } from './displayClaim';

  export let credential: DisplayCredential;

  // If you add a field, add a comment why that field should be hidden.
  //
  // `enrichment`: custom metadata field related for NGDIL demo.
  const hideFields: string[] = ['enrichment', 'id', 'type'];

  // `fields` does not have to be reactive because `credential` never changes while component is mounted.
  let fields = Object.keys(credential.data.credentialSubject).filter((field) => !hideFields.includes(field));
</script>

<!--
  SD-JWT credentials (`dc+sd-jwt` or `vc+sd-jwt`) can include `display` metadata from the issuer.
  If `display_claims` is available, we use it to render the claims with their intended names and order.
  If no display metadata is available, we fall back to iterating over the raw claims in `credentialSubject`.
-->
{#if (credential.format.format === 'dc+sd-jwt' || credential.format.format === 'vc+sd-jwt') && credential.display_claims.length > 0}
  <div class="flex flex-col gap-4">
    {#each credential.display_claims as displayClaim}
      <ClaimRenderer key={getDisplayClaimKey(displayClaim)} value={displayClaim.value} />
    {/each}
  </div>
{:else if fields}
  <div class="flex flex-col gap-4">
    {#each fields as field}
      <ClaimRenderer key={field} value={credential.data.credentialSubject[field]} />
    {/each}
  </div>
{/if}
