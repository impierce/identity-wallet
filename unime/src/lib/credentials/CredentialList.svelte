<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import IconMessage from '$lib/components/molecules/IconMessage.svelte';
  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import { state } from '$lib/stores';

  import Ghost from '~icons/ph/ghost-fill';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  $: credentials = $state.credentials;

  onMount(async () => {
    credentials = $state.credentials.filter((c) => !c.metadata.is_favorite);
    if (credentialType === 'badges') {
      credentials = credentials.filter((c) => (c.data.type as string[]).includes('OpenBadgeCredential'));
    } else if (credentialType === 'data') {
      credentials = credentials.filter((c) => !(c.data.type as string[]).includes('OpenBadgeCredential'));
    }
  });
</script>

{#if credentials?.length > 0}
  <div class="flex flex-col space-y-2">
    {#key credentials}
      {#each credentials as credential}
        <ListItemCard
          id={credential.id}
          title={credential.display_name}
          description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
          type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
          on:click={() =>
            credential.data.type.includes('OpenBadgeCredential')
              ? goto(`/badges/${credential.id}`)
              : goto(`/credentials/${credential.id}`)}
        ></ListItemCard>
      {/each}
    {/key}
  </div>
{:else if $state?.credentials?.length === 0}
  <!-- Only show "No credentials" when there's also no favorites -->
  <div class="flex grow flex-col items-center justify-center">
    <IconMessage icon={Ghost} title={$LL.ME.EMPTY_CREDENTIALS.TITLE()} />
    <p class="w-[280px] pt-[15px] text-center text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
      {$LL.ME.DEMO.TEXT_1()} <span class="font-semibold text-primary">https://demo.ngdil.com</span>
      {$LL.ME.DEMO.TEXT_2()}
    </p>
  </div>
{/if}
