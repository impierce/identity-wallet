<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import {
    BankLightIcon,
    CalendarPlusFillIcon,
    SealCheckFillIcon,
    WarningCircleFillIcon,
    XCircleFillIcon,
  } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { formatDateWithWrittenMonth, getImageAsset, hash } from '$lib/utils';
  import { getCredentialValidity } from '$lib/utils/credentialValidity';

  export let credential: DisplayCredential;
  export let isTempAsset = true;

  // Url to cached issuer logo (if available).
  let issuerLogoUrl: string | null = null;

  $: validity = getCredentialValidity(credential);

  function formatValidityDate(isoDate: string) {
    return formatDateWithWrittenMonth(isoDate, $appState.profile_settings.locale);
  }

  function determineIssuerName() {
    if (credential.issuer_name) {
      return credential.issuer_name;
    }
    // `data` is `any` on the wire, so guard rather than trust the shape.
    return credential.data?.issuer?.name ?? credential.data?.issuer ?? '';
  }

  onMount(async () => {
    if (credential.issuer_logo_uri) {
      issuerLogoUrl = await getImageAsset(hash(credential.issuer_logo_uri), isTempAsset);
    }
  });
</script>

<!--
@component
The status and issuer tiles, mirroring `credentials/[id]/CredentialOverview.svelte`. Kept as a
separate component so the wallet's credential page is not affected by changes made for the
prompt. Three deliberate differences, all forced by the context:

- No `[Credential] Refresh status` dispatch. A certification is not in the wallet; its status
  arrives on the prompt itself.
- No self-issued branch. A certification is always issued by a third party, so the avatar and
  "Unverified" paths are unreachable here.
- The issuer tile does not navigate. Leaving the prompt subtree cancels the connection request.
- The issuer logo comes from `issuer_logo_uri` in `assets/tmp`, not from the connection asset:
  a certification arrives on the prompt, before any connection exists.

### Props
- credential
-->
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
      <div>{$LL.CREDENTIAL.DETAILS.VALID()}</div>
      <div class="grid h-20 place-items-center self-stretch rounded-xl bg-green-50 text-green-600 dark:bg-background">
        <SealCheckFillIcon class="size-7" />
      </div>
      <div>
        {validity.validUntil
          ? $LL.CREDENTIAL.DETAILS.VALID_UNTIL({ date: formatValidityDate(validity.validUntil) })
          : $LL.CREDENTIAL.DETAILS.NEVER_EXPIRES()}
      </div>
    {/if}
  </div>
  <div class="flex flex-col items-center gap-1">
    <div>{$LL.CREDENTIAL.DETAILS.ISSUED_BY()}</div>
    <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background text-text-alt">
      {#if issuerLogoUrl}
        <!-- Background is always white since most logos are designed for light backgrounds -->
        <img src={issuerLogoUrl} alt="Issuer logo" class="h-12 w-12 rounded-xl bg-white object-contain p-1.5" />
      {:else}
        <BankLightIcon class="h-7 w-7" />
      {/if}
    </div>
    <!-- `break-words`, not the credential page's `break-all`: certification issuer names are long
         enough that breaking mid-word splits "Standardization" in half.
         `text-center` because `items-center` on the column only centres the box, which is already
         full width once the name wraps — the lines inside it need centring of their own. -->
    <div class="text-center break-words">{determineIssuerName()}</div>
  </div>
</div>
