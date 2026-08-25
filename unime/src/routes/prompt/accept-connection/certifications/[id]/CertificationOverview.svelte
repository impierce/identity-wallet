<script lang="ts">
  import { onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { BankLightIcon, SealCheckRegularIcon, SealWarningDuotoneIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { formatDate, getImageAsset } from '$lib/utils';

  export let credential: DisplayCredential;

  // The certification's own logo, downloaded to `assets/tmp` by the backend. A wallet
  // credential resolves its issuer logo from the connection instead.
  export let logoId: string | undefined = undefined;

  // Url to cached issuer logo (if available).
  let issuerLogoUrl: string | null = null;

  function determineIssuerName() {
    if (credential.issuer_name) {
      return credential.issuer_name;
    }
    // `data` is `any` on the wire, so guard rather than trust the shape.
    return credential.data?.issuer?.name ?? credential.data?.issuer ?? '';
  }

  onMount(async () => {
    if (logoId) {
      issuerLogoUrl = await getImageAsset(logoId, true);
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

### Props
- credential
- logoId (optional)
-->
<div class="grid grid-cols-2 gap-4 bg-background-alt text-xs font-medium">
  <div class="flex flex-col items-center gap-1">
    {#if credential.credential_status?.status === 'INVALID'}
      <p class="text-red-700 dark:text-red-500">{$LL.CREDENTIAL.DETAILS.INVALID()}</p>
      <div
        class="grid h-20 place-items-center self-stretch rounded-xl bg-red-50 py-5 text-red-700 dark:bg-background dark:text-red-500"
      >
        <SealWarningDuotoneIcon class="size-7" />
      </div>
    {:else}
      {$LL.CREDENTIAL.DETAILS.VALID()}
      <div class="grid h-20 place-items-center self-stretch rounded-xl bg-background py-5 text-text-alt">
        <SealCheckRegularIcon class="size-7" />
      </div>
      {#if credential.metadata.date_issued}
        <div>
          {formatDate(credential.metadata.date_issued, $appState.profile_settings.locale)}
        </div>
      {/if}
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
