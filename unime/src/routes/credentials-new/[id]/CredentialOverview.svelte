<script lang="ts">
  import { onMount } from 'svelte';

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

  $: console.dir(credential);
</script>

<div class="grid grid-cols-2 gap-4 text-xs">
  <div class="flex flex-col items-center gap-1">
    <div>{$LL.BADGE.DETAILS.VALID()}</div>
    <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background-alt py-5 text-text-alt">
      <SealCheckRegularIcon class="h-7 w-7" />
    </div>
    {#if credential.data}
      <div>
        {formatDate(credential.data.issuanceDate, $appState.profile_settings.locale)}
      </div>
    {/if}
  </div>
  <div class="flex flex-col items-center gap-1">
    <div>{$LL.BADGE.DETAILS.ISSUED_BY()}</div>
    <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background-alt py-5 text-text-alt">
      {#if issuerLogoUrl}
        <img src={issuerLogoUrl} alt="Issuer logo" class="h-10 w-10 object-contain" />
      {:else}
        <BankLightIcon class="h-7 w-7" />
      {/if}
    </div>
    <div class="break-all">{determineIssuerName()}</div>
  </div>
</div>
