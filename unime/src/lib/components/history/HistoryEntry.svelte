<script lang="ts">
  import { goto } from '$app/navigation';

  import type { HistoryCredential } from '@bindings/history/HistoryCredential';

  import { ListItemCard } from '$lib/components';
  import { state } from '$lib/stores';
  import { formatRelativeDateTime } from '$lib/utils';

  export let title: string;
  export let date: string;
  export let credentials: HistoryCredential[] = [];

  function navigateToCredential(credentialId: string) {
    const credential = $state.credentials.find((cred) => cred.id === credentialId);

    if (credential) {
      if (credential.data.type.includes('OpenBadgeCredential')) {
        goto(`/badges/${credential.id}`);
      } else {
        goto(`/credentials/${credential.id}`);
      }
    }
  }
</script>

<div class="flex flex-col">
  <p class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">{title}</p>
  <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
    {formatRelativeDateTime(date, $state.profile_settings.locale)}
  </p>
  {#if credentials.length > 0}
    <div class="mt-[12px] rounded-xl border border-slate-200 bg-white p-[3px] dark:border-slate-600 dark:bg-dark">
      {#each credentials as credential}
        <ListItemCard
          id={credential.id}
          title={credential.title}
          description={credential.issuer_name}
          on:click={() => navigateToCredential(credential.id)}
        />
      {/each}
    </div>
  {/if}
</div>
