<script lang="ts">
  import { onDestroy } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { Button, Checkbox, Image, ListItemCard, PaddedIcon, Radio, Switch, TopNavBar } from '$lib/components';
  import CredentialValidityIcon from '$lib/components/credentials/CredentialValidityIcon.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { CaretUpDownBoldIcon, PlugsConnectedFillIcon, SealCheckFillIcon } from '$lib/icons';
  import { cancelUserFlow, useBackNavigation } from '$lib/navigation';
  import { error, state } from '$lib/stores';
  import { hash } from '$lib/utils';
  import { getCredentialValidity } from '$lib/utils/credentialValidity';
  import { formatIso8601 } from '$lib/utils/date';

  // TypeScript does not know that the `current_user_prompt` is of type `share-credentials`.
  // Extract the type from `CurrentUserPrompt`.
  type IsShareCredentialsPrompt<T> = T extends { type: 'share-credentials' } ? T : never;
  type ShareCredentialsPrompt = IsShareCredentialsPrompt<CurrentUserPrompt>;

  // TODO: client_name should fallback to hostname if no display name is provided
  const { client_name, logo_uri, requested_credentials, is_interactive } =
    $state.current_user_prompt as ShareCredentialsPrompt;

  const credentialsById = new Map(($state.credentials ?? []).map((c) => [c.id, c]));

  const isValid = (credential: DisplayCredential) => getCredentialValidity(credential).state === 'valid';

  // Valid credentials first, each group most recently issued first.
  function candidatesFor(requested: RequestedCredential): DisplayCredential[] {
    return requested.options
      .map((id) => credentialsById.get(id))
      .filter((c): c is DisplayCredential => c !== undefined)
      .sort(
        (a, b) =>
          Number(isValid(b)) - Number(isValid(a)) ||
          (b.metadata.date_issued ?? '').localeCompare(a.metadata.date_issued ?? ''),
      );
  }

  function issuedLabel(credential: DisplayCredential): string {
    const { date_issued } = credential.metadata;
    return date_issued
      ? `${credential.issuer_name} · ${formatIso8601(date_issued, $state.profile_settings.locale)}`
      : credential.issuer_name;
  }

  const candidates = Object.fromEntries(requested_credentials.map((r) => [r.query_id, candidatesFor(r)]));

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

  const available = requested_credentials.filter((r) => candidates[r.query_id].some(isValid));

  $: selections = available
    .filter((r) => included[r.query_id] && selection[r.query_id].length > 0)
    .map((r) => ({ query_id: r.query_id, credential_uuids: selection[r.query_id] }));

  // The request whose credentials are being picked, with the picked ids that are only kept on "Save".
  let picker: { requested: RequestedCredential; selected: string[] } | undefined;

  function openPicker(requested: RequestedCredential) {
    picker = { requested, selected: [...selection[requested.query_id]] };
  }

  function toggle(credential: DisplayCredential) {
    if (!picker || !isValid(credential)) return;
    const { requested, selected } = picker;
    if (!requested.multiple) {
      picker.selected = [credential.id];
    } else if (selected.includes(credential.id)) {
      // At least one credential stays selected.
      if (selected.length > 1) picker.selected = selected.filter((id) => id !== credential.id);
    } else {
      picker.selected = [...selected, credential.id];
    }
  }

  function savePicker() {
    if (!picker) return;
    selection = { ...selection, [picker.requested.query_id]: picker.selected };
    picker = undefined;
  }

  function typeLabel(requested: RequestedCredential): string {
    const type = requested.types.find((t) => t !== 'VerifiableCredential') ?? requested.types[0] ?? requested.query_id;
    const name = type.split(/[/#:]/).pop() || type;
    return name.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/[_-]/g, ' ');
  }

  let loading = false;

  $: imageId = logo_uri ? hash(logo_uri) : '_';

  // Going back closes the picker, or cancels sharing just like the "Cancel" button (which is disabled while loading).
  const cancel = useBackNavigation(() => {
    if (picker) {
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

{#if picker}
  <div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
    <TopNavBar
      title={$LL.SCAN.SHARE_CREDENTIALS.PICKER.NAVBAR_TITLE()}
      on:back={() => (picker = undefined)}
      class="sticky top-0 z-10"
    />

    <div class="flex grow flex-col space-y-[15px] px-4 py-5">
      {#each candidates[picker.requested.query_id] as credential (credential.id)}
        {@const validity = getCredentialValidity(credential)}
        {@const checked = picker.selected.includes(credential.id)}
        <button
          class="flex w-full items-center gap-[10px] rounded-xl border border-grey bg-white p-[15px] text-left disabled:opacity-60 dark:border-slate-600 dark:bg-dark"
          disabled={validity.state !== 'valid'}
          on:click={() => toggle(credential)}
        >
          {#if picker.requested.multiple}
            <div class="pointer-events-none">
              <Checkbox {checked} readonly disabled={validity.state !== 'valid'} />
            </div>
          {:else}
            <Radio {checked} disabled={validity.state !== 'valid'} />
          {/if}
          <div class="flex grow flex-col">
            <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">{credential.display_name}</p>
            <p class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">{issuedLabel(credential)}</p>
          </div>
          <CredentialValidityIcon {validity} />
        </button>
      {/each}
    </div>

    <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button label={$LL.SCAN.SHARE_CREDENTIALS.PICKER.SAVE()} on:click={savePicker} />
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
              {@const choosable = candidates[requested.query_id].length > 1}
              <div
                class="flex items-center rounded-xl border border-grey bg-white pr-[10px] dark:border-slate-600 dark:bg-dark"
              >
                <div class="min-w-0 grow {included[requested.query_id] ? '' : 'opacity-60'}">
                  <ListItemCard
                    id={first?.id ?? requested.query_id}
                    title={first?.display_name ?? typeLabel(requested)}
                    description={selected.length > 1
                      ? $LL.SCAN.SHARE_CREDENTIALS.SELECTED_COUNT({ count: selected.length })
                      : first?.issuer_name}
                    type={first?.data?.type?.includes('OpenBadgeCredential') ? 'badge' : 'data'}
                    icon={first?.metadata.icon}
                    on:click={() => choosable && openPicker(requested)}
                  >
                    <div slot="right" class="mr-2">
                      {#if choosable}
                        <CaretUpDownBoldIcon class="size-4 text-primary" />
                      {/if}
                    </div>
                  </ListItemCard>
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
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- Controls -->
    <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
      <Button
        label={$LL.SCAN.SHARE_CREDENTIALS.APPROVE()}
        on:click={share}
        disabled={selections.length === 0}
        {loading}
      />
      <Button label={$LL.CANCEL()} variant="secondary" on:click={cancel} disabled={loading} />
    </div>
  </div>
{/if}

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
