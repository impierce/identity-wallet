<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import DataUrlImageRenderer from './DataUrlImageRenderer.svelte';

  export let credential: DisplayCredential;

  // If you add a field, add a comment why that field should be hidden.
  //
  // `enrichment`: custom metadata field related for NGDIL demo.
  const hideFields: string[] = ['enrichment', 'id', 'type'];

  function isDataUrl(value: unknown): boolean {
    return typeof value === 'string' && value.startsWith('data:image/');
  }
  // `fields` does not have to be reactive because `credential` never changes while component is mounted.
  let fields = Object.keys(credential.data.credentialSubject).filter((field) => !hideFields.includes(field));
</script>

<!--
  SD-JWT credentials (`dc+sd-jwt` or `vc+sd-jwt`) can include `display` metadata from the issuer.
  If `display_claims` is available, we use it to render the claims with their intended names and order.
  For all other formats, we fall back to iterating over the raw claims in `credentialSubject`.
-->
{#if credential.format.format === 'dc+sd-jwt' || credential.format.format === 'vc+sd-jwt'}
  {#if credential.display_claims}
    <div class="flex flex-col gap-4">
      {#each credential.display_claims as displayClaim}
        {#if isDataUrl(displayClaim.value)}
          <DataUrlImageRenderer key={displayClaim.key} dataUrl={displayClaim.value} />
        {:else}
          <div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
            <h2 class="font-medium text-text-alt">{displayClaim.key}</h2>
            <p class="overflow-x-auto">{displayClaim.value}</p>
          </div>
        {/if}
      {/each}
    </div>
  {/if}
{:else if fields}
  <div class="flex flex-col gap-4">
    {#each fields as field}
      {#if isDataUrl(credential.data.credentialSubject[field])}
        <DataUrlImageRenderer key={field} dataUrl={credential.data.credentialSubject[field]} />
      {:else}
        <div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
          <h2 class="font-medium text-text-alt">{field}</h2>
          <p class="overflow-x-auto">{credential.data.credentialSubject[field]}</p>
        </div>
      {/if}
    {/each}
  </div>
{/if}

<style>
  /* Hide scrollbar. */
  p::-webkit-scrollbar {
    display: none;
  }
</style>
