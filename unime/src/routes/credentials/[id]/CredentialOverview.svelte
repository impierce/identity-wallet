<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { BankLightIcon, SealCheckRegularIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { formatDate, getImageAsset } from '$lib/utils';

  export let credential: DisplayCredential;

  // Url to cached issuer logo (if available).
  let issuerLogoUrl: string | null = null;

  function determineIssuerName() {
    if (credential.issuer_name) {
      return credential.issuer_name;
    }
    return credential.data.issuer?.name ?? credential.data.issuer;
  }

  onMount(async () => {
    if (credential.connection_id) {
      issuerLogoUrl = await getImageAsset(credential.connection_id);
    }
  });
</script>

<div class="grid grid-cols-2 gap-4 bg-background-alt text-xs font-medium">
  <div class="flex flex-col items-center gap-1">
    <div>{$LL.CREDENTIAL.DETAILS.VALID()}</div>
    <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background py-5 text-text-alt">
      <SealCheckRegularIcon class="h-7 w-7" />
    </div>
    {#if credential.data.issuanceDate}
      <div>
        {formatDate(credential.data.issuanceDate, $appState.profile_settings.locale)}
      </div>
    {/if}
  </div>
  <div class="flex flex-col items-center gap-1">
    <div>{$LL.CREDENTIAL.DETAILS.ISSUED_BY()}</div>
    <svelte:element
      this={credential.connection_id ? 'button' : 'div'}
      on:click={credential.connection_id ? () => goto(`/activity/connection/${credential.connection_id}`) : undefined}
      role={credential.connection_id ? 'button' : undefined}
      class="grid h-20 place-items-center self-stretch rounded-xl bg-background text-text-alt"
    >
      {#if issuerLogoUrl}
        <!-- Background is always white since most logos are designed for light backgrounds -->
        <img src={issuerLogoUrl} alt="Issuer logo" class="h-12 w-12 rounded-xl bg-white object-contain p-1.5" />
      {:else}
        <BankLightIcon class="h-7 w-7" />
      {/if}
    </svelte:element>
    <div class="break-all">{determineIssuerName()}</div>
  </div>
</div>
