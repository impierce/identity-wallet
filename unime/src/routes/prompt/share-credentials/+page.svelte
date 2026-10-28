<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { writable } from 'svelte/store';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { ActionSheet, Button, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { WarningCircleFillIcon } from '$lib/icons';
  import { cancelUserFlow, useBackNavigation } from '$lib/navigation';
  import { error, pendingInteraction, state } from '$lib/stores';
  import { formatDateWithWrittenMonth } from '$lib/utils';

  import CredentialDetails from '../../credentials/[id]/CredentialDetails.svelte';
  import CredentialPicker from './CredentialPicker.svelte';
  import { claimRows, claimText, defaultDisclosure, satisfiesRequest, sharedAlongside } from './disclosure';
  import RequestCard from './RequestCard.svelte';
  import ReviewShare from './ReviewShare.svelte';
  import { distinguishingClaim, isSelectable } from './utils';
  import VerifierHeader from './VerifierHeader.svelte';

  // TypeScript does not know that the `current_user_prompt` is of type `share-credentials`.
  // Extract the type from `CurrentUserPrompt`.
  type IsShareCredentialsPrompt<T> = T extends { type: 'share-credentials' } ? T : never;
  type ShareCredentialsPrompt = IsShareCredentialsPrompt<CurrentUserPrompt>;

  // TODO: client_name should fallback to hostname if no display name is provided
  const { client_name, logo_uri, connection_url, domain_validation, requested_credentials, is_interactive } =
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

  const optionOf = (requested: RequestedCredential, id: string | undefined) =>
    requested.options.find((option) => option.id === id);

  // The fields of the first chosen credential, which the card of the request shows.
  function rowsOf(requested: RequestedCredential, ids: string[]) {
    const option = optionOf(requested, ids[0]);
    const credential = credentialsById.get(ids[0]);
    return option && credential ? claimRows(requested, option, credential) : [];
  }

  const initialDisclosure = (requested: RequestedCredential, ids: string[]) => {
    const option = optionOf(requested, ids[0]);
    return option ? defaultDisclosure(requested, option) : [];
  };

  // Cards with a choice to make start expanded.
  const initialExpanded = (requested: RequestedCredential, ids: string[]) =>
    rowsOf(requested, ids).some((row) => !row.required);

  // The requested claims (by index) that are shared, per credential query.
  let disclosure: Record<string, number[]> = Object.fromEntries(
    requested_credentials.map((r) => [r.query_id, initialDisclosure(r, selection[r.query_id])]),
  );
  let expanded: Record<string, boolean> = Object.fromEntries(
    requested_credentials.map((r) => [r.query_id, initialExpanded(r, selection[r.query_id])]),
  );

  function select(requested: RequestedCredential, ids: string[]) {
    selection = { ...selection, [requested.query_id]: ids };
    disclosure = { ...disclosure, [requested.query_id]: initialDisclosure(requested, ids) };
    expanded = { ...expanded, [requested.query_id]: initialExpanded(requested, ids) };
  }

  // The result sheet clears the pending interaction once the outcome is handled or the sheet is dismissed: restore the
  // controls. Minimizing keeps it set while the request is still in flight.
  // Interactive requests never set a pending interaction and continue with another prompt instead.
  $: if (!is_interactive && !$pendingInteraction) loading = false;

  $: selections = requested_credentials
    .filter((r) => selection[r.query_id].length > 0)
    .map((r) => ({
      query_id: r.query_id,
      credential_uuids: selection[r.query_id],
      ...(optionOf(r, selection[r.query_id][0])?.selective_disclosure
        ? { disclosed_claims: disclosure[r.query_id] }
        : {}),
    }));

  $: satisfied = requested_credentials.every((r) =>
    selection[r.query_id].every((id) => {
      const option = optionOf(r, id);
      return !option || satisfiesRequest(r, option, disclosure[r.query_id]);
    }),
  );

  // What the review lists: the shared fields per chosen credential, and the switched-off fields and the credentials
  // that were not chosen.
  $: review = (() => {
    const locale = $state.profile_settings.locale;
    const shared = requested_credentials.flatMap((r) =>
      selection[r.query_id].flatMap((id) => {
        const credential = credentialsById.get(id);
        if (!credential) return [];
        const option = optionOf(r, id);
        const fields = [
          ...rowsOf(r, [id]).filter(
            (row) => row.required || (row.index !== undefined && disclosure[r.query_id].includes(row.index)),
          ),
          ...(option ? sharedAlongside(r, option, credential, disclosure[r.query_id]) : []),
        ].map((row) => ({ label: row.label, value: claimText(row.value, locale) }));
        return [{ credential, fields }];
      }),
    );
    const notShared = requested_credentials.flatMap((r) => [
      ...selection[r.query_id].flatMap((id) =>
        rowsOf(r, [id])
          .filter((row) => !row.required && row.index !== undefined && !disclosure[r.query_id].includes(row.index))
          .map((row) => ({ label: row.label, detail: credentialsById.get(id)?.display_name ?? '' })),
      ),
      ...candidates[r.query_id]
        .filter((c) => !selection[r.query_id].includes(c.id) && isSelectable(optionStates.get(c.id)))
        .map((c) => ({
          label: distinguishingClaim(c, candidates[r.query_id]) ?? c.display_name,
          detail: $LL.SCAN.SHARE_CREDENTIALS.REVIEW.NOT_CHOSEN(),
        })),
    ]);
    return { shared, notShared };
  })();

  $: expiredSelected = selections
    .flatMap((s) => s.credential_uuids)
    .filter((id) => optionStates.get(id) === 'expired')
    .map((id) => credentialsById.get(id))
    .filter((c): c is DisplayCredential => c !== undefined);

  // The request whose credentials are being picked, with the picked ids that are only kept on "Use".
  let picker: { requested: RequestedCredential; selected: string[]; search: string } | undefined;
  // A credential opened from the picker with "View details".
  let details: DisplayCredential | undefined;
  // Whether the last check before sharing is shown.
  let reviewing = false;

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

  // Going back closes the details, the picker or the review, or declines sharing just like the "Decline" button (which
  // is disabled while loading).
  const cancel = useBackNavigation(() => {
    if (details) {
      details = undefined;
    } else if (picker) {
      picker = undefined;
    } else if (reviewing && !loading) {
      reviewing = false;
    } else if (!loading) {
      cancelUserFlow();
    }
  });

  function share() {
    loading = true;
    const action = {
      type: '[Authenticate] Credentials selected',
      payload: {
        selections,
        is_interactive,
      },
    } as const;
    // Interactive requests continue with another prompt instead of a result.
    if (!is_interactive) pendingInteraction.set({ kind: 'presentation', counterparty: client_name, retry: action });
    dispatch(action);
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
    reviewing = false;
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
      if (picker) select(picker.requested, detail);
      picker = undefined;
    }}
  />
{:else if reviewing}
  <ReviewShare
    clientName={client_name}
    logoUri={logo_uri}
    connectionUrl={connection_url}
    domainValidation={domain_validation}
    shared={review.shared}
    notShared={review.notShared}
    {loading}
    on:back={() => (reviewing = false)}
    on:share={approve}
    on:decline={() => {
      if (!loading) cancelUserFlow();
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

    <div class="flex grow flex-col gap-4 px-4 pt-2 pb-6">
      <VerifierHeader
        title={$LL.SCAN.SHARE_CREDENTIALS.WANTS_TO_SEE({ name: client_name })}
        logoUri={logo_uri}
        connectionUrl={connection_url}
        domainValidation={domain_validation}
      />

      <div class="flex w-full flex-col space-y-4">
        {#each requested_credentials as requested (requested.query_id)}
          <RequestCard
            {requested}
            candidates={candidates[requested.query_id]}
            selected={selection[requested.query_id]
              .map((id) => credentialsById.get(id))
              .filter((c): c is DisplayCredential => c !== undefined)}
            disclosed={disclosure[requested.query_id]}
            bind:expanded={expanded[requested.query_id]}
            on:change={() => openPicker(requested)}
            on:disclose={({ detail }) => (disclosure = { ...disclosure, [requested.query_id]: detail })}
          />
        {/each}
      </div>
    </div>

    <!-- Controls -->
    <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button
        label={$LL.SCAN.SHARE_CREDENTIALS.REVIEW_AND_SHARE()}
        on:click={() => (reviewing = true)}
        disabled={selections.length < requested_credentials.length || !satisfied}
      />
      <Button label={$LL.SCAN.SHARE_CREDENTIALS.DECLINE()} variant="secondary" on:click={cancel} disabled={loading} />
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
