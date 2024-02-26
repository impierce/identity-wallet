<script lang="ts">
  import type { HistoryCredential } from '@bindings/HistoryCredential';

  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import { state } from '$src/stores';

  export let title: string;
  export let date: string;
  export let credentials: HistoryCredential[] = [];
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
    {new Date(date).toLocaleString($state.locale, {
      dateStyle: 'medium',
      timeStyle: 'medium',
    })}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] rounded-xl border border-white border-y-gray-200 p-[3px] dark:border-dark dark:bg-dark">
      {#each credentials as credential}
        <ListItemCard
          id={credential.image_id}
          title={credential.title}
          description={credential.sub_title}
          hasBorderRadius={false}
        />
      {/each}
    </div>
  {/if}
</div>
