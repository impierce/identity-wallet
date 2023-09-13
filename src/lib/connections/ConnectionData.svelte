<script lang="ts">
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import House from '~icons/ph/house-light';

  import CredentialListEntry from '../components/CredentialListEntry.svelte';

  const data = [];
</script>

{#if data.length > 0}
  <div class="flex w-full flex-col items-center justify-center space-y-4 py-6">
    <div class="h-[75px] w-[75px] rounded-3xl border bg-silver p-2">
      <!-- <img src={'/tauri.svg'} alt={connection.domain} /> -->
    </div>

    <div class="text-center text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
      Data shared
    </div>
  </div>
  <div
    class="flex w-full flex-col rounded-2xl border border-slate-200 bg-white p-1 dark:border-slate-600 dark:bg-dark"
  >
    <CredentialListEntry
      title="Email address"
      description="ferris.rustacean@example.org"
      color="bg-purple-100"
    >
      <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span>
    </CredentialListEntry>
    <CredentialListEntry
      title="Home address"
      description="Sunset Rd 1337, Iron Islands"
      color="bg-green-100"
    >
      <span slot="icon"><House class="h-6 w-6" /></span>
    </CredentialListEntry>
  </div>
{:else}
  <div class="flex h-full flex-col items-center justify-center">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">No data yet.</p>
  </div>
{/if}
