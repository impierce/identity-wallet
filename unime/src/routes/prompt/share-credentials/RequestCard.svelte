<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

  import { CredentialValidityLine, ListItemImage } from '$lib/components';
  import { CaretDownBoldIcon, CaretUpBoldIcon, EyeRegularIcon, EyeSlashRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { formatDateWithWrittenMonth } from '$lib/utils';

  import ClaimRow from './ClaimRow.svelte';
  import { claimRows, claimText, satisfiesRequest } from './disclosure';
  import { distinguishingClaim, isBadge, typeLabel } from './utils';

  export let requested: RequestedCredential;
  // All options of the request, in the order they are shown in the picker.
  export let candidates: DisplayCredential[];
  export let selected: DisplayCredential[];
  // The requested claims (by index) that are shared.
  export let disclosed: number[];
  export let expanded: boolean;

  const dispatch = createEventDispatcher<{ change: void; disclose: number[] }>();

  $: first = selected[0];
  $: option = first && requested.options.find((option) => option.id === first.id);
  $: detail = first && selected.length === 1 ? distinguishingClaim(first, candidates) : undefined;
  $: expired = option?.state === 'expired';

  $: locale = $state.profile_settings.locale;
  $: rows = first && option ? claimRows(requested, option, first) : [];
  $: isShared = (row: (typeof rows)[number]) =>
    row.required || (row.index !== undefined && disclosed.includes(row.index));
  $: sharedRows = rows.filter(isShared);
  $: optionalOff = rows.length - sharedRows.length;
  $: selective = option?.selective_disclosure ?? false;
  $: satisfied = !option || satisfiesRequest(requested, option, disclosed);

  $: summary =
    sharedRows.length === 1
      ? `${sharedRows[0].label}: ${claimText(sharedRows[0].value, locale)}`
      : sharedRows.map((row) => row.label).join(', ');

  $: counts = !selective
    ? $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.WHOLE_CREDENTIAL()
    : rows.length === 0
      ? $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.NONE_REQUESTED()
      : [
          $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.SHARED_SUMMARY({ shared: sharedRows.length, total: rows.length }),
          ...(optionalOff > 0 ? [$LL.SCAN.SHARE_CREDENTIALS.CLAIMS.OPTIONAL_OFF({ count: optionalOff })] : []),
        ].join(' · ');

  $: footer = !selective
    ? $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.WHOLE_CREDENTIAL()
    : rows.length === 0
      ? $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.NONE_REQUESTED()
      : $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.SHARING({ shared: sharedRows.length, total: rows.length });

  function toggle(index: number | undefined, on: boolean) {
    if (index === undefined) return;
    dispatch(
      'disclose',
      on ? [...disclosed, index].sort((a, b) => a - b) : disclosed.filter((disclosedIndex) => disclosedIndex !== index),
    );
  }
</script>

<!--
@component
A requested credential type on the share screen: the chosen credential with the fields that will be shared, which
collapse into a summary, and a "Change" button when there are more options.
-->
<div class="flex flex-col overflow-hidden rounded-xl border border-grey bg-white dark:border-slate-600 dark:bg-dark">
  <button
    class="flex items-center gap-3 p-3 text-left"
    disabled={!first}
    aria-expanded={expanded}
    aria-label={expanded ? $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.COLLAPSE() : $LL.SCAN.SHARE_CREDENTIALS.CLAIMS.EXPAND()}
    on:click={() => (expanded = !expanded)}
  >
    <ListItemImage
      id={first?.id ?? requested.query_id}
      type={first && isBadge(first) ? 'badge' : 'data'}
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
        <p class="text-[12px]/[20px] font-medium break-words text-slate-400">
          {first.issuer_name}{first.metadata.date_issued && selected.length === 1
            ? ` · ${$LL.SCAN.SHARE_CREDENTIALS.ISSUED({
                date: formatDateWithWrittenMonth(first.metadata.date_issued, locale),
              })}`
            : ''}
        </p>
        {#if expired}
          <CredentialValidityLine credential={first} />
        {/if}
      {/if}
    </div>
    {#if first}
      <span class="grid size-6 shrink-0 place-items-center text-slate-500 dark:text-slate-300">
        <svelte:component this={expanded ? CaretUpBoldIcon : CaretDownBoldIcon} class="size-[14px]" />
      </span>
    {/if}
  </button>
  {#if first}
    {#if expanded}
      <div class="border-t border-grey dark:border-slate-600">
        {#each rows as row (row.index ?? row.label)}
          <ClaimRow
            label={row.label}
            value={claimText(row.value, locale)}
            required={row.required}
            shared={isShared(row)}
            on:toggle={({ detail }) => toggle(row.index, detail)}
          />
        {/each}
      </div>
      <div class="flex items-center gap-1.5 bg-silver px-[14px] py-2 dark:bg-navy">
        <svelte:component
          this={selective && optionalOff > 0 ? EyeSlashRegularIcon : EyeRegularIcon}
          class="size-3 shrink-0 text-slate-500 dark:text-slate-300"
        />
        <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{footer}</p>
      </div>
    {:else}
      <div class="flex flex-col gap-0.5 border-t border-grey px-[14px] py-[10px] dark:border-slate-600">
        {#if summary}
          <p class="line-clamp-2 text-[13px]/[24px] font-medium break-words text-slate-700 dark:text-grey">
            {summary}
          </p>
        {/if}
        <p class="text-[12px]/[20px] font-medium text-slate-400">{counts}</p>
      </div>
    {/if}
    {#if !satisfied}
      <p
        class="border-t border-grey px-[14px] py-2 text-[12px]/[20px] font-medium text-rose-700 dark:border-slate-600 dark:text-rose-300"
      >
        {$LL.SCAN.SHARE_CREDENTIALS.CLAIMS.NOT_ENOUGH()}
      </p>
    {/if}
  {/if}
  {#if candidates.length > 1 || selected.length === 0}
    <div class="flex items-center gap-2 border-t border-grey py-2 pr-[10px] pl-[14px] dark:border-slate-600">
      <p class="grow text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.SCAN.SHARE_CREDENTIALS.OPTIONS_AVAILABLE({ count: candidates.length })}
      </p>
      <button
        class="h-9 rounded-xl border border-slate-200 bg-white px-4 text-[13px]/[24px] font-medium text-slate-800 dark:border-slate-600 dark:bg-dark dark:text-grey"
        on:click={() => dispatch('change')}
      >
        {$LL.SCAN.SHARE_CREDENTIALS.CHANGE()}
      </button>
    </div>
  {/if}
</div>
