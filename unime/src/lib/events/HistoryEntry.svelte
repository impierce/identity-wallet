<script lang="ts">
  import type { DisplayCredential } from 'src-tauri/identity_wallet/bindings/display-credential/DisplayCredential';

  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import type { Connection } from '$lib/connections/types';

  // export let connection: Connection;
  export let title: string;
  export let timestamp: string;
  export let credentials: DisplayCredential[] = [];
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500">
    {new Date(timestamp).toLocaleString('en-US', {
      dateStyle: 'medium',
      timeStyle: 'medium',
    })}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] rounded-xl border border-slate-200 bg-white p-[3px] dark:border-slate-600">
      {#each credentials as credential}
        <ListItemCard
          id={credential.id}
          title={credential.metadata.display.name ?? credential.data.type.at(-1)}
          description="Lorem ipsum dolor"
        >
          <!-- <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span> -->
        </ListItemCard>
      {/each}
    </div>
  {/if}
</div>
