<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { Button, CredentialValidityLine, ListItemImage, SelectionIndicator, TopNavBar } from '$lib/components';
  import { CaretRightBoldIcon, ClockCounterClockwiseIcon, EyeRegularIcon, MagnifyingGlassIcon } from '$lib/icons';

  import { isBadge, isSelectable } from './utils';

  export let requested: RequestedCredential;
  // All options of the request, in display order.
  export let candidates: DisplayCredential[];
  // The ids of the chosen credentials; updated in place so a choice made on the details screen is kept.
  export let selected: string[];
  export let clientName: string;
  export let search = '';

  // Above this number of options, the picker offers search and "Select all".
  const SEARCH_THRESHOLD = 3;

  const dispatch = createEventDispatcher<{ back: void; use: string[]; details: DisplayCredential }>();

  const multiple = requested.multiple;
  const stateOf = (credential: DisplayCredential) =>
    requested.options.find((option) => option.id === credential.id)?.state;

  const claimText = (value: unknown) => (typeof value === 'string' || typeof value === 'number' ? String(value) : '');

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

  function toggle(credential: DisplayCredential) {
    if (!isSelectable(stateOf(credential))) return;
    if (!multiple) {
      selected = [credential.id];
    } else if (selected.includes(credential.id)) {
      selected = selected.filter((id) => id !== credential.id);
    } else {
      selected = [...selected, credential.id];
    }
  }

  $: visible = candidates.filter((c) => matchesSearch(c, search));
  $: validIds = candidates.filter((c) => stateOf(c) === 'valid').map((c) => c.id);
  $: allValidSelected = validIds.length > 0 && validIds.every((id) => selected.includes(id));
</script>

<!--
@component
Lets the user choose which of the matching credentials to share for one requested credential type.
-->
<div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={multiple
      ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.NAVBAR_TITLE_MULTIPLE()
      : $LL.SCAN.SHARE_CREDENTIALS.PICKER.NAVBAR_TITLE()}
    on:back={() => dispatch('back')}
    class="sticky top-0 z-10"
  />

  <div class="flex grow flex-col gap-3 px-4 pt-3 pb-5">
    <p class="text-[13px]/[24px] font-semibold text-slate-500 dark:text-slate-300">
      {multiple ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_MULTIPLE() : $LL.SCAN.SHARE_CREDENTIALS.PICKER.CHOOSE_ONE()}
    </p>

    {#if candidates.length > SEARCH_THRESHOLD}
      <div class="flex items-center gap-3">
        <label
          class="flex h-12 grow items-center gap-[10px] rounded-xl border border-grey bg-white px-4 dark:border-slate-600 dark:bg-dark"
        >
          <MagnifyingGlassIcon class="size-[18px] shrink-0 text-slate-400" />
          <input
            type="search"
            bind:value={search}
            placeholder={$LL.SCAN.SHARE_CREDENTIALS.PICKER.SEARCH({ count: candidates.length })}
            class="w-full bg-transparent text-[13px]/[24px] font-medium text-slate-800 placeholder:text-slate-400 focus:outline-hidden dark:text-grey"
          />
        </label>
        {#if multiple}
          <button
            class="flex shrink-0 items-center gap-2 pr-1"
            on:click={() => (selected = allValidSelected ? [] : validIds)}
          >
            <SelectionIndicator checked={allValidSelected} multiple />
            <span class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">
              {$LL.SCAN.SHARE_CREDENTIALS.PICKER.SELECT_ALL()}
            </span>
          </button>
        {/if}
      </div>
    {/if}

    {#each visible as credential (credential.id)}
      {@const selectable = isSelectable(stateOf(credential))}
      {@const checked = selected.includes(credential.id)}
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
              type={isBadge(credential) ? 'badge' : 'data'}
              icon={credential.metadata.icon}
            />
          </div>
          <div class="flex min-w-0 grow flex-col gap-0.5">
            <p
              class="text-[13px]/[24px] font-semibold {selectable ? 'text-slate-800 dark:text-grey' : 'text-slate-400'}"
            >
              {credential.display_name}
            </p>
            <p class="text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">{credential.issuer_name}</p>
            <CredentialValidityLine {credential} />
            {#if requested.last_shared.includes(credential.id)}
              <div class="pt-1">
                <span
                  class="inline-flex items-center gap-1 rounded-full bg-silver px-2 py-0.5 text-[12px]/[20px] font-medium text-slate-500 dark:bg-navy dark:text-slate-300"
                >
                  <ClockCounterClockwiseIcon class="size-3" />
                  {$LL.SCAN.SHARE_CREDENTIALS.PICKER.LAST_SHARED({ name: clientName })}
                </span>
              </div>
            {/if}
          </div>
        </button>
        <button
          class="flex items-center gap-[6px] border-t border-grey px-[14px] py-[10px] text-primary dark:border-slate-600"
          on:click={() => dispatch('details', credential)}
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
        ? $LL.SCAN.SHARE_CREDENTIALS.PICKER.USE_MULTIPLE({ selected: selected.length, total: candidates.length })
        : $LL.SCAN.SHARE_CREDENTIALS.PICKER.USE_ONE()}
      disabled={selected.length === 0}
      on:click={() => dispatch('use', selected)}
    />
  </div>
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
