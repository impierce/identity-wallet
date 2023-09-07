<script lang="ts">
  import { fly } from 'svelte/transition';

  import ChevronDown from '~icons/lucide/chevron-down';
  import ChevronUp from '~icons/lucide/chevron-up';
  import CircleDashed from '~icons/lucide/circle-dashed';

  import type { Login } from './types';

  export let data: Login;

  let showCredentials = false;
</script>

<div class="flex flex-col rounded-lg bg-blue-100 pt-4">
  <span class="px-4 text-start text-sm text-blue-500"
    >You have logged in to <span class="font-semibold">{data.verifier.domain}</span>
    using {data.credentials.length} credentials</span
  >
  <!-- <span class="text-blue-500">{data.issuer.did}</span> -->
  <!-- Dropdown button -->
  <button
    class="flex items-center justify-center rounded-b-lg bg-blue-100 p-2"
    on:click={() => (showCredentials = !showCredentials)}
  >
    {#if showCredentials}
      <ChevronUp class="h-6 w-6 text-blue-500" />
    {:else}
      <ChevronDown class="h-6 w-6 text-blue-500" />
    {/if}
  </button>
</div>

<!-- Credentials -->
{#if showCredentials}
  <div class="space-y-2 px-8 py-2" in:fly={{ y: -24 }}>
    {#each data.credentials as credential}
      <div class="flex rounded-lg bg-violet-100 p-2">
        <div class="rounded-lg bg-violet-200 p-2">
          <CircleDashed class="h-6 w-6 text-violet-500" />
        </div>
        <div class="grow" />
      </div>
    {/each}
  </div>
{/if}
