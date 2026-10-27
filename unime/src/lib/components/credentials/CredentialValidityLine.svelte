<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { SealCheckFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateWithWrittenMonth } from '$lib/utils';
  import { getCredentialValidity } from '$lib/utils/credentialValidity';

  import CredentialValidityIcon from './CredentialValidityIcon.svelte';

  export let credential: DisplayCredential;

  $: validity = getCredentialValidity(credential);

  const formatDate = (isoDate: string) => formatDateWithWrittenMonth(isoDate, $state.profile_settings.locale);
</script>

<!--
@component
One line with the validity of a credential: the icon of the credential list plus a short text.
-->
<div class="flex items-center gap-1 text-[12px]/[20px] font-medium">
  {#if validity.state === 'valid'}
    <SealCheckFillIcon class="size-3 shrink-0 text-green-600 dark:text-green-500" />
    <span class="text-green-700 dark:text-green-500">
      {validity.validUntil
        ? $LL.SCAN.SHARE_CREDENTIALS.VALIDITY.VALID_UNTIL({ date: formatDate(validity.validUntil) })
        : $LL.CREDENTIAL.DETAILS.NEVER_EXPIRES()}
    </span>
  {:else}
    <CredentialValidityIcon {validity} class="size-3 shrink-0" />
    {#if validity.state === 'revoked' || validity.state === 'suspended'}
      <span class="text-red-700 dark:text-red-500">
        {validity.state === 'revoked'
          ? $LL.SCAN.SHARE_CREDENTIALS.VALIDITY.REVOKED()
          : $LL.SCAN.SHARE_CREDENTIALS.VALIDITY.SUSPENDED()}
      </span>
    {:else if validity.state === 'expired'}
      <span class="text-orange-700 dark:text-orange-500">
        {$LL.SCAN.SHARE_CREDENTIALS.VALIDITY.EXPIRED_ON({ date: formatDate(validity.validUntil) })}
      </span>
    {:else}
      <span class="text-ex-grey-2 dark:text-text-alt">
        {$LL.SCAN.SHARE_CREDENTIALS.VALIDITY.VALID_FROM({ date: formatDate(validity.validFrom) })}
      </span>
    {/if}
  {/if}
</div>
