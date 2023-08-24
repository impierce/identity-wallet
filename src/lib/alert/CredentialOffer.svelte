<script lang="ts">
  export let data: {}; // expects a "credential_definition"

  // remove type "VerifiableCredential" when there are multiple types (irrelevant for display)
  $: if (data['type'].length > 1) {
    const index = data['type'].indexOf('VerifiableCredential', 0);
    if (index > -1) {
      data['type'].splice(index, 1);
    }
  }
</script>

<div class="flex w-full flex-col space-x-2 py-2">
  {#each data.type as type}
    <div class="min-w-min rounded bg-slate-300 px-4 py-2 font-semibold">
      {type}
    </div>
  {/each}
  <div class="flex flex-col space-y-4 pt-4">
    {#each Object.entries(data.credentialSubject) as entry}
      <div class="">
        <!-- {JSON.stringify(entry[1].display)} -->
        {#if entry[1].display?.at(0)?.name}
          {entry[1].display?.at(0)?.name}
        {:else}
          {entry[0]}
        {/if}
      </div>
    {/each}
  </div>
</div>
