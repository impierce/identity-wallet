<script lang="ts">
  export let data: {};

  // "id" not relevant for display (metadata)
  delete data['id'];
</script>

<!--
@component
Display a credential subject as a list of key-value pairs.

Disclaimer: Displaying nested values is currently only supported for one level.
More deeply nested values will be displayed as a stringified JSON object.
-->
<div class="space-y-2">
  {#each Object.entries(data) as entry}
    <div class="flex flex-col items-end">
      <!-- Key -->
      <div class="text-xs font-medium text-slate-400">{entry[0]}</div>

      <!-- Value -->
      {#if typeof entry[1] !== 'object'}
        <div class="break-all text-end font-semibold text-slate-500">{entry[1]}</div>
      {:else}
        <!-- Nested value -->
        {#each Object.entries(entry[1]) as subEntry}
          <div class="flex flex-col items-end">
            <div class="text-xs font-medium text-slate-400">{subEntry[0]}</div>
            <div class="break-all">{JSON.stringify(subEntry[1])}</div>
          </div>
        {/each}
      {/if}
    </div>
    <!-- <div class="break-all"> -->
    <!-- {entry[0]}: {JSON.stringify(entry[1])} -->
    <!-- </div> -->
  {/each}
</div>
