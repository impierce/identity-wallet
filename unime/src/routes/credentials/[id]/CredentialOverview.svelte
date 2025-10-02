<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { Avatar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { BankLightIcon, SealCheckRegularIcon, SealQuestionRegularIcon, SealWarningRegularIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { calculateInitials, formatDate, getImageAsset } from '$lib/utils';

  export let credential: DisplayCredential;

  // Url to cached issuer logo (if available).
  let issuerLogoUrl: string | null = null;

  let initials: string | undefined = undefined;

  // TODO: this shouldn't be determined by the frontend ==> add a metadata field: `self_issued: bool`
  function isSelfIssued() {
    if (credential.data.issuer === credential.data.credentialSubject.id) {
      return true;
    }
    if (Object.values($appState.dids).includes(credential.data.issuer)) {
      return true;
    }
    return false;
  }

  function determineIssuerName() {
    if (credential.issuer_name) {
      return credential.issuer_name;
    }
    if (isSelfIssued()) {
      return $LL.CREDENTIAL.DETAILS.SELF_SIGNED();
    }
    return credential.data.issuer?.name ?? credential.data.issuer;
  }

  onMount(async () => {
    await dispatch({ type: '[Credential] Refresh Status', payload: { credential_id: credential.id } });
    if (credential.connection_id) {
      issuerLogoUrl = await getImageAsset(credential.connection_id);
    }
    if ($appState?.profile_settings.profile?.name) {
      initials = calculateInitials($appState?.profile_settings.profile?.name);
    }
  });
</script>

<div class="grid grid-cols-2 gap-4 bg-background-alt text-xs font-medium">
  <div class="flex flex-col items-center gap-1">
    {#if credential.credential_status?.status === 'INVALID'}
      <p class="text-red-700 dark:text-red-500">Invalid</p>
      <div
        class="grid h-20 place-items-center self-stretch rounded-xl bg-red-50 py-5 text-red-700 dark:bg-background dark:text-red-500"
      >
        <SealWarningRegularIcon class="size-7" />
      </div>
    {:else}
      {isSelfIssued() ? $LL.CREDENTIAL.DETAILS.UNVERIFIED() : $LL.CREDENTIAL.DETAILS.VALID()}
      <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background py-5 text-text-alt">
        {#if isSelfIssued()}
          <SealQuestionRegularIcon class="size-7" />
        {:else}
          <SealCheckRegularIcon class="size-7" />
        {/if}
      </div>
      {#if credential.data.issuanceDate}
        <div>
          {formatDate(credential.data.issuanceDate, $appState.profile_settings.locale)}
        </div>
      {/if}
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
      {:else if isSelfIssued()}
        <Avatar {initials} picture={$appState.profile_settings.profile?.picture} />
      {:else}
        <BankLightIcon class="h-7 w-7" />
      {/if}
    </svelte:element>
    <div class="break-all">{determineIssuerName()}</div>
  </div>
</div>
