<script lang="ts">
  import { onDestroy } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';

  import Button from '$lib/components/atoms/Button.svelte';
  import Checkbox from '$lib/components/atoms/Checkbox.svelte';
  import Image from '$lib/components/atoms/Image.svelte';
  import PaddedIcon from '$lib/components/atoms/PaddedIcon.svelte';
  import ListItemCard from '$lib/components/molecules/ListItemCard.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { error, state } from '$lib/stores';
  import { hash } from '$lib/utils';

  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import SealCheck from '~icons/ph/seal-check-fill';

  // TypeScript does not know that the `current_user_prompt` is of type `share-credentials`.
  // Extract the type from `CurrentUserPrompt`.
  type IsShareCredentialsPrompt<T> = T extends { type: 'share-credentials' } ? T : never;
  type ShareCredentialsPrompt = IsShareCredentialsPrompt<CurrentUserPrompt>;

  const { client_name, logo_uri, options } = $state.current_user_prompt as ShareCredentialsPrompt;

  let selected_credentials = $state.credentials?.filter((c) => options.indexOf(c.id) > -1);

  let loading = false;

  $: imageId = logo_uri ? hash(logo_uri) : '_';

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
  <TopNavBar title={$LL.SCAN.SHARE_CREDENTIALS.NAVBAR_TITLE()} on:back={() => history.back()} disabled={loading} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    <!-- Header -->
    {#if logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver">
        <Image id={imageId} isTempAsset={true} />
      </div>
    {:else}
      <PaddedIcon icon={PlugsConnected} />
    {/if}
    <div class="text-center">
      <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
        {client_name}
      </p>
      <!-- <p class="pt-[10px] text-sm font-medium text-slate-500">
        {hostname}
      </p> -->
    </div>

    <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SCAN.SHARE_CREDENTIALS.DESCRIPTION()}
    </p>

    <div class="w-full">
      <div class="flex items-center">
        <SealCheck class="mr-2 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
          {$LL.SCAN.SHARE_CREDENTIALS.REQUESTED()}
        </p>
      </div>

      <!-- Credentials selection -->
      <!-- <div class="w-full space-y-2 rounded-2xl bg-white p-3"></div> -->
      <div
        class="mt-3 w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
      >
        <div class="flex w-full flex-col space-y-2">
          {#each selected_credentials as credential}
            <ListItemCard id={credential.id} title={credential.display_name}>
              <div slot="right" class="mr-2">
                <Checkbox checked={true} disabled={true} />
              </div>
            </ListItemCard>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.SHARE_CREDENTIALS.APPROVE()}
      on:click={() => {
        loading = true;
        dispatch({
          type: '[Authenticate] Credentials selected',
          payload: {
            credential_uuids: selected_credentials.map((c) => c.id),
          },
        });
      }}
      {loading}
    />
    <Button
      label={$LL.CANCEL()}
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
        goto('/me');
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
