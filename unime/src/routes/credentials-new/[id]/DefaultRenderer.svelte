<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  export let credential: DisplayCredential;

  const hiddenFields: string[] = ['type'];

  $: fields = Object.keys(credential.data.credentialSubject).filter((field) => !hiddenFields.includes(field));
</script>

{#if fields}
  <div class="flex flex-col gap-4">
    {#each fields as field}
      <div class="rounded-xl bg-background-alt px-4 py-2">
        <h2 class="font-semibold">{field}</h2>
        <p class="overflow-x-auto">{credential.data.credentialSubject[field]}</p>
      </div>
    {/each}
  </div>
{/if}

<style>
  /* Hide scrollbar. */
  p::-webkit-scrollbar {
    display: none;
  }
</style>
