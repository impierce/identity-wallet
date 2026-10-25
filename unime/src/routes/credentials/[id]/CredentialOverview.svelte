<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { Avatar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    BankLightIcon,
    CalendarPlusFillIcon,
    SealCheckFillIcon,
    SealQuestionRegularIcon,
    WarningCircleFillIcon,
    XCircleFillIcon,
  } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { calculateInitials, formatDateWithWrittenMonth, getImageAsset } from '$lib/utils';

  import { getCredentialValidity } from './credentialValidity';

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

  $: validity = getCredentialValidity(credential);

  function formatValidityDate(isoDate: string) {
    return formatDateWithWrittenMonth(isoDate, $appState.profile_settings.locale);
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
    await dispatch({ type: '[Credential] Refresh status', payload: { credential_id: credential.id } });
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
    {#if validity.state === 'revoked' || validity.state === 'suspended'}
      <div>{validity.state === 'revoked' ? $LL.CREDENTIAL.DETAILS.REVOKED() : $LL.CREDENTIAL.DETAILS.SUSPENDED()}</div>
      <div class="grid h-20 place-items-center self-stretch rounded-xl bg-red-50 text-red-600 dark:bg-background">
        <XCircleFillIcon class="size-7" />
      </div>
      <div class="text-red-700 dark:text-red-500">{$LL.CREDENTIAL.DETAILS.BY_THE_ISSUER()}</div>
    {:else if validity.state === 'expired'}
      <div>{$LL.CREDENTIAL.DETAILS.EXPIRED()}</div>
      <div class="grid h-20 place-items-center self-stretch rounded-xl bg-orange-50 text-orange-600 dark:bg-background">
        <WarningCircleFillIcon class="size-7" />
      </div>
      <div class="text-orange-700 dark:text-orange-500">
        {$LL.CREDENTIAL.DETAILS.EXPIRED_SINCE({ date: formatValidityDate(validity.validUntil) })}
      </div>
    {:else if validity.state === 'not_yet_valid'}
      <div>{$LL.CREDENTIAL.DETAILS.NOT_VALID_YET()}</div>
      <div
        class="grid h-20 place-items-center self-stretch rounded-xl bg-grey text-ex-grey-1 dark:bg-background dark:text-text-alt"
      >
        <CalendarPlusFillIcon class="size-7" />
      </div>
      <div class="text-ex-grey-2 dark:text-text-alt">
        {$LL.CREDENTIAL.DETAILS.VALID_FROM({ date: formatValidityDate(validity.validFrom) })}
      </div>
    {:else}
      {#if isSelfIssued()}
        <div>{$LL.CREDENTIAL.DETAILS.UNVERIFIED()}</div>
        <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background text-text-alt">
          <SealQuestionRegularIcon class="size-7" />
        </div>
      {:else}
        <div>{$LL.CREDENTIAL.DETAILS.VALID()}</div>
        <div class="grid h-20 place-items-center self-stretch rounded-xl bg-green-50 text-green-600 dark:bg-background">
          <SealCheckFillIcon class="size-7" />
        </div>
      {/if}
      <div>
        {validity.validUntil
          ? $LL.CREDENTIAL.DETAILS.VALID_UNTIL({ date: formatValidityDate(validity.validUntil) })
          : $LL.CREDENTIAL.DETAILS.NEVER_EXPIRES()}
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
      {:else if isSelfIssued()}
        <Avatar {initials} picture={$appState.profile_settings.profile?.picture} />
      {:else}
        <BankLightIcon class="h-7 w-7" />
      {/if}
    </svelte:element>
    <div class="break-all">{determineIssuerName()}</div>
  </div>
</div>
