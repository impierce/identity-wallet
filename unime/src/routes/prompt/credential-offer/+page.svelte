<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import { Button, Checkbox, Image, ListItemCard, PaddedIcon, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { DownloadSimpleFillIcon } from '$lib/icons';
  import { error, state } from '$lib/stores';
  import { hash } from '$lib/utils';

  // TODO: generate binding in core
  interface CredentialConfiguration {
    display: object[];
    credential_definition: object;
  }

  let credential_configurations: Record<string, CredentialConfiguration> =
    $state.current_user_prompt?.credential_configurations;

  let issuer_name: string = $state.current_user_prompt?.issuer_name;

  let all_credential_configuration_ids: string[] = Object.keys(credential_configurations);

  let loading = false;

  const imageId = $state.current_user_prompt?.logo_uri ? hash($state.current_user_prompt?.logo_uri) : '_';

  // When an error is received, cancel the flow and redirect to the "me" page
  error.subscribe((err) => {
    if (err) {
      loading = false;
      dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
    }
  });

  onDestroy(async () => {
    // TODO: is onDestroy also called when user accepts since the component itself is destroyed?
    dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={$LL.SCAN.CREDENTIAL_OFFER.NAVBAR_TITLE()} on:back={() => history.back()} disabled={loading} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    {#if $state.current_user_prompt.logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl">
        <Image
          id={imageId}
          isTempAsset={true}
          iconClass="dark:text-slate-800"
          imgClass="flex w-full items-center justify-center overflow-hidden rounded-3xl p-2"
        />
      </div>
    {:else}
      <PaddedIcon icon={DownloadSimpleFillIcon} />
    {/if}
    <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
      {issuer_name}
    </p>

    <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SCAN.CREDENTIAL_OFFER.DESCRIPTION()}
    </p>

    <div
      class="mt-3 w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
    >
      {#each Object.entries(credential_configurations) as [credential_configuration_id, credential_configuration]}
        <!-- TODO: bug: long list is not correctly displayed -->
        <ListItemCard
          id={`credential_${credential_configuration_id}`}
          title={credential_configuration.display?.at(0)?.name ??
            credential_configuration.credential_definition.type.at(-1)}
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
        loading = true;
        dispatch({
          type: '[Credential Offer] Selected',
          payload: {
            credential_configuration_ids: all_credential_configuration_ids,
          },
        });
      }}
      {loading}
    />
    <Button
      label={$LL.REJECT()}
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
      }}
      disabled={loading}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
