<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';
  import { writable } from 'svelte/store';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import {
    ActionSheet,
    Button,
    CredentialValidityLine,
    Image,
    ListItemImage,
    PaddedIcon,
    SelectionIndicator,
    Switch,
    TopNavBar,
  } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    CaretRightBoldIcon,
    ClockCounterClockwiseIcon,
    EyeRegularIcon,
    MagnifyingGlassIcon,
    PlugsConnectedFillIcon,
    SealCheckFillIcon,
    WarningCircleFillIcon,
  } from '$lib/icons';
  import { cancelUserFlow, useBackNavigation } from '$lib/navigation';
  import { error, state } from '$lib/stores';
  import { formatDateWithWrittenMonth, hash } from '$lib/utils';
  import { getCredentialValidity } from '$lib/utils/credentialValidity';

  import CredentialClaimsRenderer from '../../credentials/[id]/CredentialClaimsRenderer.svelte';
  import CredentialHeader from '../../credentials/[id]/CredentialHeader.svelte';
  import CredentialOverview from '../../credentials/[id]/CredentialOverview.svelte';

  // TypeScript does not know that the `current_user_prompt` is of type `share-credentials`.
  // Extract the type from `CurrentUserPrompt`.
  type IsShareCredentialsPrompt<T> = T extends { type: 'share-credentials' } ? T : never;
  type ShareCredentialsPrompt = IsShareCredentialsPrompt<CurrentUserPrompt>;

  // TODO: client_name should fallback to hostname if no display name is provided
  const { client_name, logo_uri, requested_credentials, is_interactive } =
    $state.current_user_prompt as ShareCredentialsPrompt;

  // Above this number of options, the picker offers search and "Select all".
  const SEARCH_THRESHOLD = 3;

  const credentialsById = new Map(($state.credentials ?? []).map((c) => [c.id, c]));

  const isValid = (credential: DisplayCredential) => getCredentialValidity(credential).state === 'valid';
  const isExpired = (credential: DisplayCredential) => getCredentialValidity(credential).state === 'expired';
  // Expired credentials can still be shared (after a warning), revoked, suspended and not yet valid ones cannot.
  const isSelectable = (credential: DisplayCredential) => isValid(credential) || isExpired(credential);

  const formatDate = (isoDate: string) => formatDateWithWrittenMonth(isoDate, $state.profile_settings.locale);

  // Valid credentials first, each group most recently issued first.
  function candidatesFor(requested: RequestedCredential): DisplayCredential[] {
    return requested.options
      .map((id) => credentialsById.get(id))
      .filter((c): c is DisplayCredential => c !== undefined)
      .sort(
        (a, b) =>
          Number(isValid(b)) - Number(isValid(a)) ||
          Number(isSelectable(b)) - Number(isSelectable(a)) ||
          (b.metadata.date_issued ?? '').localeCompare(a.metadata.date_issued ?? ''),
      );
  }

  const candidates = Object.fromEntries(requested_credentials.map((r) => [r.query_id, candidatesFor(r)]));

  const claimText = (value: unknown) => (typeof value === 'string' || typeof value === 'number' ? String(value) : '');

  // The first claim whose value differs between the options, which tells otherwise identical credentials apart.
  function distinguishingClaim(credential: DisplayCredential, requested: RequestedCredential): string | undefined {
    const others = candidates[requested.query_id].filter((c) => c.id !== credential.id);
    if (others.length === 0) return undefined;
    const claim = credential.display_claims.find((claim) => {
      const value = claimText(claim.value);
      const samePath = (c: DisplayCredential) =>
        c.display_claims.find((other) => other.path.join('/') === claim.path.join('/'));
      return value && value !== credential.display_name && others.some((c) => claimText(samePath(c)?.value) !== value);
    });
    return claim ? claimText(claim.value) : undefined;
  }

  // The credentials last shared with this verifier, otherwise the most recently issued valid one.
  function defaultSelection(requested: RequestedCredential): string[] {
    const valid = candidates[requested.query_id].filter(isValid).map((c) => c.id);
    const lastShared = valid.filter((id) => requested.last_shared.includes(id));
    if (lastShared.length > 0) {
      return requested.multiple ? lastShared : [lastShared[0]];
    }
    return valid.slice(0, 1);
  }

  let selection: Record<string, string[]> = Object.fromEntries(
    requested_credentials.map((r) => [r.query_id, defaultSelection(r)]),
  );

  // Optional requests can be left out with their toggle.
  let included: Record<string, boolean> = Object.fromEntries(requested_credentials.map((r) => [r.query_id, true]));

  const available = requested_credentials.filter((r) => candidates[r.query_id].some(isSelectable));

  $: selections = available
    .filter((r) => included[r.query_id] && selection[r.query_id].length > 0)
    .map((r) => ({ query_id: r.query_id, credential_uuids: selection[r.query_id] }));

  $: expiredSelected = selections
    .flatMap((s) => s.credential_uuids)
    .map((id) => credentialsById.get(id))
    .filter((c): c is DisplayCredential => c !== undefined && isExpired(c));

  // The request whose credentials are being picked, with the picked ids that are only kept on "Use".
  let picker: { requested: RequestedCredential; selected: string[]; search: string } | undefined;
  // A credential opened from the picker with "View details".
  let details: DisplayCredential | undefined;

  function openPicker(requested: RequestedCredential) {
    picker = { requested, selected: [...selection[requested.query_id]], search: '' };
  }

  function toggle(credential: DisplayCredential) {
    if (!picker || !isSelectable(credential)) return;
    const { requested, selected } = picker;
    if (!requested.multiple) {
      picker.selected = [credential.id];
    } else if (selected.includes(credential.id)) {
      picker.selected = selected.filter((id) => id !== credential.id);
    } else {
      picker.selected = [...selected, credential.id];
    }
  }

  function savePicker() {
    if (!picker) return;
    selection = { ...selection, [picker.requested.query_id]: picker.selected };
    picker = undefined;
  }

  function matchesSearch(credential: DisplayCredential, search: string) {
    const query = search.trim().toLowerCase();
    if (!query) return true;
    return [
      credential.display_name,
      credential.issuer_name,
      ...credential.display_claims.map((c) => claimText(c.value)),
    ]
      .filter(Boolean)
      .some((text) => text.toLowerCase().includes(query));
  }

  $: pickerCandidates = picker ? candidates[picker.requested.query_id] : [];
  $: pickerVisible = picker ? pickerCandidates.filter((c) => matchesSearch(c, picker!.search)) : [];
  $: pickerValidIds = pickerCandidates.filter(isValid).map((c) => c.id);
  $: allValidSelected =
    !!picker && pickerValidIds.length > 0 && pickerValidIds.every((id) => picker!.selected.includes(id));

  function toggleSelectAll() {
    if (!picker) return;
    picker.selected = allValidSelected ? [] : pickerValidIds;
  }

  function chooseFromDetails(credential: DisplayCredential) {
    if (picker && !picker.selected.includes(credential.id)) {
      picker.selected = picker.requested.multiple ? [...picker.selected, credential.id] : [credential.id];
    }
    details = undefined;
  }

  function typeLabel(requested: RequestedCredential): string {
    const type = requested.types.find((t) => t !== 'VerifiableCredential') ?? requested.types[0] ?? requested.query_id;
    const name = type.split(/[/#:]/).pop() || type;
    return name.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/[_-]/g, ' ');
  }

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
    const requested = available.find((r) => selection[r.query_id].includes(expiredSelected[0]?.id));
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
    <div class="flex grow flex-col gap-7 px-4 pb-7">
      <CredentialHeader {credential} actions={false}>
        <p class="w-full truncate px-4 text-center font-semibold text-slate-700 dark:text-grey">
          {credential.display_name}
        </p>
      </CredentialHeader>
      <CredentialOverview {credential} />
      <CredentialClaimsRenderer {credential} />
    </div>
    {#if isSelectable(credential)}
      <div class="sticky bottom-0 left-0 flex flex-col rounded-t-2xl bg-white p-6 dark:bg-dark">
        <Button
          label={$LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_THIS()}
          on:click={() => chooseFromDetails(credential)}
        />
      </div>
    {/if}
  </div>
{:else if picker}
  {@const multiple = picker.requested.multiple}
  <div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
    <TopNavBar
      title={multiple
        ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.NAVBAR_TITLE_MULTIPLE()
        : $LL.SCAN.SHARE_CREDENTIALS.PICKER.NAVBAR_TITLE()}
      on:back={() => (picker = undefined)}
      class="sticky top-0 z-10"
    />

    <div class="flex grow flex-col gap-3 px-4 pt-3 pb-5">
      <p class="text-[13px]/[24px] font-semibold text-slate-500 dark:text-slate-300">
        {multiple
          ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_MULTIPLE()
          : $LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_ONE()}
      </p>

      {#if pickerCandidates.length > SEARCH_THRESHOLD}
        <div class="flex items-center gap-3">
          <label
            class="flex h-12 grow items-center gap-[10px] rounded-xl border border-grey bg-white px-4 dark:border-slate-600 dark:bg-dark"
          >
            <MagnifyingGlassIcon class="size-[18px] shrink-0 text-slate-400" />
            <input
              type="search"
              bind:value={picker.search}
              placeholder={$LL.SCAN.SHARE_CREDENTIALS.PICKER.SEARCH({ count: pickerCandidates.length })}
              class="w-full bg-transparent text-[13px]/[24px] font-medium text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-grey"
            />
          </label>
          {#if multiple}
            <button class="flex shrink-0 items-center gap-2 pr-1" on:click={toggleSelectAll}>
              <SelectionIndicator checked={allValidSelected} multiple />
              <span class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
                {$LL.SCAN.SHARE_CREDENTIALS.PICKER.SELECT_ALL()}
              </span>
            </button>
          {/if}
        </div>
      {/if}

      {#each pickerVisible as credential (credential.id)}
        {@const selectable = isSelectable(credential)}
        {@const checked = picker.selected.includes(credential.id)}
        <div
          class="flex flex-col rounded-xl border bg-white dark:bg-dark {checked
            ? 'border-primary ring-1 ring-primary'
            : 'border-grey dark:border-slate-600'}"
        >
          <button
            class="flex w-full items-start gap-3 px-[14px] pt-[14px] pb-[10px] text-left"
            disabled={!selectable}
            on:click={() => toggle(credential)}
          >
            <div class="pt-3 {selectable ? '' : 'opacity-40'}">
              <SelectionIndicator {checked} {multiple} />
            </div>
            <div class={selectable ? '' : 'opacity-50'}>
              <ListItemImage
                id={credential.id}
                type={credential.data?.type?.includes('OpenBadgeCredential') ? 'badge' : 'data'}
                icon={credential.metadata.icon}
              />
            </div>
            <div class="flex min-w-0 grow flex-col gap-0.5">
              <p
                class="text-[13px]/[24px] font-semibold {selectable
                  ? 'text-slate-800 dark:text-grey'
                  : 'text-slate-400'}"
              >
                {credential.display_name}
              </p>
              <p class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">{credential.issuer_name}</p>
              <CredentialValidityLine {credential} />
              {#if picker.requested.last_shared.includes(credential.id)}
                <div class="pt-1">
                  <span
                    class="inline-flex items-center gap-1 rounded-full bg-silver px-2 py-0.5 text-[12px]/[20px] font-medium text-slate-500 dark:bg-navy dark:text-slate-300"
                  >
                    <ClockCounterClockwiseIcon class="size-3" />
                    {$LL.SCAN.SHARE_CREDENTIALS.PICKER.LAST_SHARED({ name: client_name })}
                  </span>
                </div>
              {/if}
            </div>
          </button>
          <button
            class="flex items-center gap-[6px] border-t border-grey px-[14px] py-[10px] text-primary dark:border-slate-600"
            on:click={() => (details = credential)}
          >
            <EyeRegularIcon class="size-4" />
            <span class="grow text-left text-[12px]/[20px] font-semibold">
              {$LL.SCAN.SHARE_CREDENTIALS.PICKER.VIEW_DETAILS()}
            </span>
            <CaretRightBoldIcon class="size-3" />
          </button>
        </div>
      {/each}
    </div>

    <div class="sticky bottom-0 left-0 flex flex-col rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button
        label={multiple
          ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.USE_MULTIPLE({
              selected: picker.selected.length,
              total: pickerCandidates.length,
            })
          : $LL.SCAN.SHARE_CREDENTIALS.PICKER.USE_ONE()}
        disabled={picker.selected.length === 0}
        on:click={savePicker}
      />
    </div>
  </div>
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

      {#if available.length > 0}
        <div class="w-full">
          <div class="flex items-center">
            <SealCheckFillIcon class="mr-2 text-primary" />
            <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.SHARE_CREDENTIALS.REQUESTED()}
            </p>
          </div>

          <div class="mt-3 flex w-full flex-col space-y-[15px]">
            {#each available as requested (requested.query_id)}
              {@const selected = selection[requested.query_id]
                .map((id) => credentialsById.get(id))
                .filter((c) => c !== undefined)}
              {@const first = selected[0]}
              {@const options = candidates[requested.query_id].length}
              {@const detail = first && selected.length === 1 ? distinguishingClaim(first, requested) : undefined}
              <div class="flex flex-col rounded-xl border border-grey bg-white dark:border-slate-600 dark:bg-dark">
                <div class="flex items-center gap-3 p-3 {included[requested.query_id] ? '' : 'opacity-60'}">
                  <ListItemImage
                    id={first?.id ?? requested.query_id}
                    type={first?.data?.type?.includes('OpenBadgeCredential') ? 'badge' : 'data'}
                    icon={first?.metadata.icon}
                  />
                  <div class="flex min-w-0 grow flex-col gap-0.5">
                    <p class="truncate text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
                      {first?.display_name ?? typeLabel(requested)}
                    </p>
                    {#if selected.length > 1}
                      <p class="text-[12px]/[20px] font-medium text-slate-700 dark:text-slate-300">
                        {$LL.SCAN.SHARE_CREDENTIALS.SELECTED_COUNT({ count: selected.length })}
                      </p>
                    {:else if detail}
                      <p class="truncate text-[12px]/[20px] font-medium text-slate-700 dark:text-slate-300">{detail}</p>
                    {/if}
                    {#if first}
                      <p class="truncate text-[12px]/[20px] font-medium text-slate-400">
                        {first.issuer_name}{first.metadata.date_issued && selected.length === 1
                          ? ` · ${$LL.SCAN.SHARE_CREDENTIALS.ISSUED({ date: formatDate(first.metadata.date_issued) })}`
                          : ''}
                      </p>
                      {#if isExpired(first)}
                        <CredentialValidityLine credential={first} />
                      {/if}
                    {/if}
                  </div>
                  {#if !requested.required}
                    <div class="border-l border-grey pl-[15px] dark:border-slate-600">
                      <Switch
                        checked={included[requested.query_id]}
                        onCheckedChange={({ next }) => {
                          included = { ...included, [requested.query_id]: next };
                          return next;
                        }}
                      >
                        <span class="sr-only">{first?.display_name ?? typeLabel(requested)}</span>
                      </Switch>
                    </div>
                  {/if}
                </div>
                {#if options > 1 || selected.length === 0}
                  <div
                    class="flex items-center gap-2 border-t border-grey py-2 pr-[10px] pl-[14px] dark:border-slate-600"
                  >
                    <p class="grow text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
                      {$LL.SCAN.SHARE_CREDENTIALS.OPTIONS_AVAILABLE({ count: options })}
                    </p>
                    <button
                      class="h-9 rounded-xl border border-slate-200 bg-white px-4 text-[13px]/[24px] font-medium text-slate-800 dark:border-slate-600 dark:bg-dark dark:text-grey"
                      on:click={() => openPicker(requested)}
                    >
                      {$LL.SCAN.SHARE_CREDENTIALS.CHANGE()}
                    </button>
                  </div>
                {/if}
              </div>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Controls -->
    <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button
        label={$LL.SCAN.SHARE_CREDENTIALS.APPROVE()}
        on:click={approve}
        disabled={selections.length === 0}
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
