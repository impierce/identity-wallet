<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { PinInput } from 'melt/builders';

  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';

  import { Button, Checkbox, Image, ListItemCard, PaddedIcon, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { DownloadSimpleFillIcon } from '$lib/icons';
  import { state as appState, error } from '$lib/stores';
  import { hash } from '$lib/utils';

  // TypeScript does not know that the `current_user_prompt` is of type `credential-offer`.
  // Extract the type from `CurrentUserPrompt`.
  type IsCredentialOfferPrompt<T> = T extends { type: 'credential-offer' } ? T : never;
  type CredentialOfferPrompt = IsCredentialOfferPrompt<CurrentUserPrompt>;

  const { credential_configurations, issuer_name, logo_uri, tx_code } =
    $appState.current_user_prompt as CredentialOfferPrompt;

  // let credential_configurations: Record<string, CredentialConfiguration> =
  //   $state.current_user_prompt?.credential_configurations;

  let all_credential_configuration_ids: string[] = Object.keys(credential_configurations);

  let loading = $state(false);

  let complete = $state(false);

  const imageId = logo_uri ? hash(logo_uri) : '_';

  const pinInput = new PinInput({
    type: tx_code?.input_mode ?? 'numeric',
    maxLength: tx_code?.length ?? 6,
    placeholder: '',
    allowPaste: false,
    onValueChange() {
      complete = pinInput.value.length === tx_code?.length;
    },
  });

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

<div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.CREDENTIAL_OFFER.NAVBAR_TITLE()}
    on:back={() => history.back()}
    disabled={loading}
    class="sticky top-0 z-10"
  />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    {#if logo_uri}
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
      {#each Object.values(credential_configurations) as credential_configuration}
        <!-- TODO: bug: long list is not correctly displayed -->
        <ListItemCard
          id={hash(credential_configuration.display?.at(0)?.logo?.uri ?? '')}
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

    <!-- PIN Code -->
    {#if tx_code}
      <!-- <div class="flex grow flex-col items-center justify-center space-x-6 p-4"> -->
      <div>
        <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
          A PIN code is required to claim the credentials.
        </p>

        {#if tx_code.length && tx_code.length <= 6}
          <!-- PIN input -->
          <div {...pinInput.root} class="mt-6 flex items-center justify-center gap-2 font-mono">
            {#each pinInput.inputs as input}
              <input
                {...input}
                class="size-12 rounded-xl border border-slate-300 bg-background-alt text-center text-2xl font-semibold text-text-alt outline-none focus:border-primary disabled:cursor-not-allowed dark:border-slate-500"
              />
            {/each}
          </div>
        {:else}
          <!-- If length is not provided or longer than 6, fall back to a simple text input field. -->
          <input
            class="mt-6 w-full rounded-xl border border-slate-300 bg-background-alt px-3 py-3 text-[14px]/[22px] font-medium text-slate-800 dark:border-slate-600 dark:text-grey"
            placeholder={'Enter PIN code'}
            bind:value={pinInput.value}
            oninput={() => {
              // Marks the input as complete if it holds any value.
              complete = pinInput.value.length > 0;
            }}
          />
        {/if}
      </div>
    {/if}
  </div>

  <!-- `sticky` is relative to the nearest scrolling ancestor, which is the enclosing `div` above and not the viewport. -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.CREDENTIAL_OFFER.ACCEPT()}
      disabled={tx_code && !complete}
      on:click={() => {
        loading = true;
        dispatch({
          type: '[Credential Offer] Selected',
          payload: {
            credential_configuration_ids: all_credential_configuration_ids,
            tx_code: tx_code ? pinInput.value : undefined,
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
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
