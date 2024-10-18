<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import DataUrlImageRenderer from './DataUrlImageRenderer.svelte';

  export let credential: DisplayCredential;

  // If you add a field, add a comment why that field should be hidden.
  //
  // `enrichment`: custom metadata field related for NGDIL demo.
  const hideFields: string[] = [
    'enrichment',
    'id',
    'type',

    // TODO: to be removed once the backend properly propagates the to be displayed fields.
    // SD-JWT claims
    'iss',
    'nbf',
    'exp',
    'status',
    'iat',
    'sub',
    '_sd_alg',
    'cnf',
    // 'vct',
  ];

  function isDataUrl(value: unknown): boolean {
    return typeof value === 'string' && value.startsWith('data:image/');
  }
  // `fields` does not have to be reactive because `credential` never changes while component is mounted.
  let fields = Object.keys(credential.data.credentialSubject).filter((field) => !hideFields.includes(field));
</script>

{#if fields}
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
