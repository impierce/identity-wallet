<script lang="ts">
  import { onDestroy } from 'svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import Checkbox from '$src/lib/components/atoms/Checkbox.svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';

  import DownloadSimple from '~icons/ph/download-simple-fill';

  // TODO: generate binding in core
  /* eslint-disable @typescript-eslint/no-explicit-any */
  interface CredentialOffer {
    credential_issuer: string;
    credentials: any[];
    grants: any;
  }
  /* eslint-enable @typescript-eslint/no-explicit-any */

  let credential_offer: CredentialOffer = $state.current_user_prompt?.credential_offer;

  let issuer_name: string = $state.current_user_prompt?.issuer_name;

  let all_offer_indices: number[] = credential_offer.credentials.map((_, i: number) => i);

  onDestroy(async () => {
    dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={$LL.SCAN.CREDENTIAL_OFFER.NAVBAR_TITLE()} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl">
      <!-- TODO: should fallback to <PaddedIcon> instead of icon -->
      <Image
        id={'client_0'}
        isTempAsset={true}
        iconClass="dark:text-slate-800"
        imgClass="flex w-full items-center justify-center overflow-hidden rounded-3xl p-2"
      >
        <div slot="fallback">
          <PaddedIcon icon={DownloadSimple} />
        </div>
      </Image>
    </div>
    <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
      {#if issuer_name}
        {issuer_name}
      {:else}
        {new URL(credential_offer.credential_issuer).hostname}
      {/if}
    </p>

    <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SCAN.CREDENTIAL_OFFER.DESCRIPTION()}
    </p>

    <div
      class="mt-3 w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
    >
      {#each credential_offer.credentials as credential, index}
        <!-- TODO: careful with long list! -->
        <ListItemCard
          id={`credential_${index}`}
          title={$state?.current_user_prompt?.display?.at(index)?.name ?? credential.credential_definition.type.at(-1)}
          isTempAsset={true}
        >
          <div slot="right" class="mr-2">
            <Checkbox checked={true} disabled={true} />
          </div>
        </ListItemCard>
      {/each}
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.CREDENTIAL_OFFER.ACCEPT()}
      on:click={() => {
        dispatch({
          type: '[Credential Offer] Selected',
          payload: {
            offer_indices: all_offer_indices,
          },
        });
      }}
    />
    <Button
      label={$LL.REJECT()}
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
      }}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
