<script lang="ts">
  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';
  import type { HistoryCredential } from '@bindings/HistoryCredential';

  export let title: string;
  export let date: string;
  export let credentials: HistoryCredential[] = [];
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500">
    {new Date(date).toLocaleString($state.locale, {
      dateStyle: 'medium',
      timeStyle: 'medium',
    })}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] rounded-xl border border-slate-200 bg-white p-[3px] dark:border-slate-600">
      {#each credentials as credential}
        <ListItemCard id={credential.image_id} title={credential.title} description={credential.sub_title}>
          <!-- <span slot="icon"><EnvelopeSimple class="h-6 w-6" /></span> -->
        </ListItemCard>
      {/each}
    </div>
  {/if}
</div>
