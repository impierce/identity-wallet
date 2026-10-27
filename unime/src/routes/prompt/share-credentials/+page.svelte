<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { writable } from 'svelte/store';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { ActionSheet, Button, Image, PaddedIcon, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { PlugsConnectedFillIcon, SealCheckFillIcon, WarningCircleFillIcon } from '$lib/icons';
  import { cancelUserFlow, useBackNavigation } from '$lib/navigation';
  import { error, state } from '$lib/stores';
  import { formatDateWithWrittenMonth, hash } from '$lib/utils';

  import CredentialDetails from '../../credentials/[id]/CredentialDetails.svelte';
  import CredentialPicker from './CredentialPicker.svelte';
  import RequestCard from './RequestCard.svelte';
  import { isSelectable } from './utils';

  // TypeScript does not know that the `current_user_prompt` is of type `share-credentials`.
  // Extract the type from `CurrentUserPrompt`.
  type IsShareCredentialsPrompt<T> = T extends { type: 'share-credentials' } ? T : never;
  type ShareCredentialsPrompt = IsShareCredentialsPrompt<CurrentUserPrompt>;

  // TODO: client_name should fallback to hostname if no display name is provided
  const { client_name, logo_uri, requested_credentials, is_interactive } =
    $state.current_user_prompt as ShareCredentialsPrompt;

  const credentialsById = new Map(($state.credentials ?? []).map((c) => [c.id, c]));

  // The state of each option as determined by the backend (valid, expired or unavailable).
  const optionStates = new Map(requested_credentials.flatMap((r) => r.options.map((o) => [o.id, o.state])));
  const stateRank = { valid: 0, expired: 1, unavailable: 2 };

  // Valid credentials first, then expired, then unavailable, each group most recently issued first.
  function candidatesFor(requested: RequestedCredential): DisplayCredential[] {
    return requested.options
      .map((option) => credentialsById.get(option.id))
      .filter((c): c is DisplayCredential => c !== undefined)
      .sort(
        (a, b) =>
          stateRank[optionStates.get(a.id)!] - stateRank[optionStates.get(b.id)!] ||
          (b.metadata.date_issued ?? '').localeCompare(a.metadata.date_issued ?? ''),
      );
  }

  const candidates = Object.fromEntries(requested_credentials.map((r) => [r.query_id, candidatesFor(r)]));

  // The credentials last shared with this verifier, otherwise the most recently issued valid one.
  function defaultSelection(requested: RequestedCredential): string[] {
    const valid = candidates[requested.query_id].map((c) => c.id).filter((id) => optionStates.get(id) === 'valid');
    const lastShared = valid.filter((id) => requested.last_shared.includes(id));
    if (lastShared.length > 0) {
      return requested.multiple ? lastShared : [lastShared[0]];
    }
    return valid.slice(0, 1);
  }

  let selection: Record<string, string[]> = Object.fromEntries(
    requested_credentials.map((r) => [r.query_id, defaultSelection(r)]),
  );

  $: selections = requested_credentials
    .filter((r) => selection[r.query_id].length > 0)
    .map((r) => ({ query_id: r.query_id, credential_uuids: selection[r.query_id] }));

  $: expiredSelected = selections
    .flatMap((s) => s.credential_uuids)
    .filter((id) => optionStates.get(id) === 'expired')
    .map((id) => credentialsById.get(id))
    .filter((c): c is DisplayCredential => c !== undefined);

  // The request whose credentials are being picked, with the picked ids that are only kept on "Use".
  let picker: { requested: RequestedCredential; selected: string[]; search: string } | undefined;
  // A credential opened from the picker with "View details".
  let details: DisplayCredential | undefined;

  function openPicker(requested: RequestedCredential) {
    picker = { requested, selected: [...selection[requested.query_id]], search: '' };
  }

  function chooseFromDetails(credential: DisplayCredential) {
    if (picker && !picker.selected.includes(credential.id)) {
      picker.selected = picker.requested.multiple ? [...picker.selected, credential.id] : [credential.id];
    }
    details = undefined;
  }

  const formatDate = (isoDate: string) => formatDateWithWrittenMonth(isoDate, $state.profile_settings.locale);

  let loading = false;
  const expiredWarningOpen = writable(false);

  $: imageId = logo_uri ? hash(logo_uri) : '_';

  // Going back closes the details or the picker, or cancels sharing just like the "Cancel" button (which is disabled
  // while loading).
  const cancel = useBackNavigation(() => {
    if (details) {
      details = undefined;
    } else if (picker) {
      picker = undefined;
    } else if (!loading) {
      cancelUserFlow();
    }
  });

  function share() {
    loading = true;
    dispatch({
      type: '[Authenticate] Credentials selected',
      payload: {
        selections,
        is_interactive,
      },
    });
  }

  function approve() {
    if (expiredSelected.length > 0) {
      $expiredWarningOpen = true;
    } else {
      share();
    }
  }

  function chooseAnother() {
    $expiredWarningOpen = false;
    const requested = requested_credentials.find((r) => selection[r.query_id].includes(expiredSelected[0]?.id));
    if (requested) openPicker(requested);
  }

  // When an error is received, cancel the flow and redirect to the "me" page
  const unsubscribe = error.subscribe((err) => {
    if (err) {
      loading = false;
      dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
    }
  });

  onDestroy(async () => {
    unsubscribe();
    // TODO: is onDestroy also called when user accepts since the component itself is destroyed?
    dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

{#if details}
  {@const credential = details}
  <div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-background-alt">
    <TopNavBar title={$LL.CREDENTIAL.NAVBAR_TITLE()} on:back={() => (details = undefined)} class="sticky top-0 z-10" />
    <div class="grow">
      <CredentialDetails {credential} actions={false}>
        <p class="w-full truncate px-4 text-center font-semibold text-slate-700 dark:text-grey">
          {credential.display_name}
        </p>
      </CredentialDetails>
    </div>
    {#if isSelectable(optionStates.get(credential.id))}
      <div class="sticky bottom-0 left-0 flex flex-col rounded-t-2xl bg-white p-6 dark:bg-dark">
        <Button
          label={$LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_THIS()}
          on:click={() => chooseFromDetails(credential)}
        />
      </div>
    {/if}
  </div>
{:else if picker}
  <CredentialPicker
    requested={picker.requested}
    candidates={candidates[picker.requested.query_id]}
    bind:selected={picker.selected}
    bind:search={picker.search}
    clientName={client_name}
    on:back={() => (picker = undefined)}
    on:details={({ detail }) => (details = detail)}
    on:use={({ detail }) => {
      if (picker) selection = { ...selection, [picker.requested.query_id]: detail };
      picker = undefined;
    }}
  />
{:else}
  <div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
    <TopNavBar
      title={$LL.SCAN.SHARE_CREDENTIALS.NAVBAR_TITLE()}
      on:back={cancel}
      disabled={loading}
      class="sticky top-0 z-10"
    />

    <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
      <!-- Header -->
      {#if logo_uri}
        <div class="flex h-[75px] w-[75px] items-center justify-center">
          <Image id={imageId} isTempAsset={true} imgClass="size-full rounded-3xl bg-transparent object-contain" />
        </div>
      {:else}
        <PaddedIcon icon={PlugsConnectedFillIcon} />
      {/if}
      <div class="text-center">
        <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
          {client_name}
        </p>
      </div>

      <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SCAN.SHARE_CREDENTIALS.DESCRIPTION()}
      </p>

      <div class="w-full">
        <div class="flex items-center">
          <SealCheckFillIcon class="mr-2 text-primary" />
          <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
            {$LL.SCAN.SHARE_CREDENTIALS.REQUESTED()}
          </p>
        </div>

        <div class="mt-3 flex w-full flex-col space-y-[15px]">
          {#each requested_credentials as requested (requested.query_id)}
            <RequestCard
              {requested}
              candidates={candidates[requested.query_id]}
              selected={selection[requested.query_id]
                .map((id) => credentialsById.get(id))
                .filter((c): c is DisplayCredential => c !== undefined)}
              on:change={() => openPicker(requested)}
            />
          {/each}
        </div>
      </div>
    </div>

    <!-- Controls -->
    <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button
        label={$LL.SCAN.SHARE_CREDENTIALS.APPROVE()}
        on:click={approve}
        disabled={selections.length < requested_credentials.length}
        {loading}
      />
      <Button label={$LL.CANCEL()} variant="secondary" on:click={cancel} disabled={loading} />
    </div>
  </div>
{/if}

<ActionSheet
  titleText={$LL.SCAN.SHARE_CREDENTIALS.EXPIRED_WARNING.TITLE()}
  descriptionText={expiredSelected[0]
    ? $LL.SCAN.SHARE_CREDENTIALS.EXPIRED_WARNING.DESCRIPTION({
        name: expiredSelected[0].display_name,
        date: formatDate(expiredSelected[0].metadata.expiration_date ?? ''),
        verifier: client_name,
      })
    : ''}
  open={expiredWarningOpen}
>
  <div slot="icon" class="mb-4 grid size-[75px] place-items-center rounded-3xl bg-orange-50 text-orange-600">
    <WarningCircleFillIcon class="size-7" />
  </div>
  <div slot="content" class="flex w-full flex-col gap-[10px] pt-6">
    <Button
      label={$LL.SCAN.SHARE_CREDENTIALS.EXPIRED_WARNING.CONFIRM()}
      on:click={() => {
        $expiredWarningOpen = false;
        share();
      }}
    />
    <Button label={$LL.SCAN.SHARE_CREDENTIALS.EXPIRED_WARNING.CANCEL()} variant="secondary" on:click={chooseAnother} />
  </div>
</ActionSheet>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
