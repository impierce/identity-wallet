<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { IconMessage, ListItemCard } from '$lib/components';
  import { GhostFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  let credentials: DisplayCredential[];

  // Credentials have to be reactive since state can update while component is mounted.
  $: {
    // First filter out favorites, then filter by type (if applicable).
    credentials = $state.credentials.filter((c) => !c.metadata.is_favorite);
    if (credentialType === 'badges') {
      credentials = credentials.filter((c) => (c.data.type as string[]).includes('OpenBadgeCredential'));
    } else if (credentialType === 'data') {
      credentials = credentials.filter((c) => !(c.data.type as string[]).includes('OpenBadgeCredential'));
    }
  }
</script>

{#if credentials?.length > 0}
  <div class="flex flex-col space-y-2">
    <!-- Add credential.id as key to help Svelte update the list correctly. -->
    {#each credentials as credential (credential.id)}
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
  </div>
{:else if $state?.credentials?.length === 0}
  <!-- Only show "No credentials" when there's also no favorites -->
  <div class="flex grow flex-col items-center justify-center">
    <IconMessage icon={GhostFillIcon} title={$LL.ME.EMPTY_CREDENTIALS.TITLE()} />
    <div class="w-[280px] pt-[15px] text-center text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
      {$LL.ME.DEMO()}
      <div class="flex flex-col">
        <p class="font-semibold text-primary">https://selv.iota.org</p>
        <p class="font-semibold text-primary">https://demo.ngdil.com</p>
      </div>
    </div>
  </div>
{/if}
