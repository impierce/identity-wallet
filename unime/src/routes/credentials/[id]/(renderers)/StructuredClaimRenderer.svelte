<script lang="ts">
  import LL from '$i18n/i18n-svelte';
  import { SvelteMap } from 'svelte/reactivity';
  import { writable } from 'svelte/store';
  import { slide } from 'svelte/transition';

  import ActionSheet from '$lib/components/ActionSheet.svelte';
  import { CaretDownBoldIcon, CaretRightBoldIcon } from '$lib/icons';

  import {
    buildClaimTree,
    type ClaimCount,
    type ClaimGroup,
    type ClaimNode,
    type ClaimTable,
    type ClaimTitle,
  } from '../structuredClaim';
  import ClaimValue from './ClaimValue.svelte';

  let { key, value }: { key: string; value: unknown } = $props();

  const tree = $derived(buildClaimTree(key, value));

  // Rows the holder folded or unfolded. The others keep their default: only the top level opens expanded.
  const toggled = new SvelteMap<string, boolean>();

  function isOpen(node: ClaimGroup | ClaimTable): boolean {
    return toggled.get(node.key) ?? node.depth === 0;
  }

  function toggle(node: ClaimGroup | ClaimTable) {
    toggled.set(node.key, !isOpen(node));
  }

  function titleText(title: ClaimTitle): string {
    return title.kind === 'key' ? title.text : $LL.CREDENTIAL.RENDERER.STRUCTURED.ITEM({ position: title.position });
  }

  function countText(count: ClaimCount): string {
    return count.unit === 'fields'
      ? $LL.CREDENTIAL.RENDERER.STRUCTURED.FIELDS({ count: count.count })
      : $LL.CREDENTIAL.RENDERER.STRUCTURED.ENTRIES({ count: count.count });
  }

  /** At most four columns fit the phone; wider Tables scroll sideways. */
  const VISIBLE_COLUMNS = 4;

  // The Table row shown in the sheet.
  const sheetOpen = writable(false);
  let sheet = $state<{ title: string; entry: string; pairs: [string, string | undefined][] }>();

  function openRow(table: ClaimTable, rowIndex: number) {
    sheet = {
      title: titleText(table.title),
      entry: $LL.CREDENTIAL.RENDERER.STRUCTURED.ENTRY({ position: rowIndex + 1 }),
      pairs: table.columns.map((column, index) => [column, table.rows[rowIndex].cells[index]]),
    };
    sheetOpen.set(true);
  }
</script>

<!--
@component
Renders a claim whose value is an Object or an array as one card of rows, following the UniMe preview of UniTrust.
Objects, Lists of Objects and Tables are rows that fold their content away when tapped; only the claim itself opens
expanded. Lists of single values show as chips, and Tables as a compact grid whose rows open in a sheet.
-->

{#snippet label(text: string)}
  <div class="text-[12px]/[18px] text-text-alt">{text}</div>
{/snippet}

{#snippet foldRow(node: ClaimGroup | ClaimTable, open: boolean)}
  {@const top = node.depth === 0}
  <button
    type="button"
    class="flex w-full items-center gap-2 text-left {top ? 'px-4 py-3' : 'px-3 py-2.5'}"
    aria-expanded={open}
    onclick={() => toggle(node)}
  >
    {#if top}
      <h2 class="font-medium text-text-alt">{titleText(node.title)}</h2>
    {:else}
      <span class="text-[12px]/[18px] text-text-alt">{titleText(node.title)}</span>
    {/if}
    {#if !open || node.kind === 'table'}
      <span class="text-[11px]/[18px] text-text-alt">{countText(node.count)}</span>
    {/if}
    {#if open}
      <CaretDownBoldIcon class="ml-auto size-3.5 shrink-0 text-primary" />
    {:else}
      <CaretRightBoldIcon class="ml-auto size-3.5 shrink-0 text-primary" />
    {/if}
  </button>
{/snippet}

{#snippet table(node: ClaimTable)}
  {@const template = `repeat(${node.columns.length}, minmax(min-content, 1fr))`}
  {@const width = `${(Math.max(node.columns.length, VISIBLE_COLUMNS) / VISIBLE_COLUMNS) * 100}%`}
  <div class="relative {node.depth === 0 ? 'mx-4 mb-4' : 'mx-3 mb-3'}">
    <div class="table-scroll overflow-x-auto rounded-lg border border-slate-200 dark:border-slate-600">
      <!-- Columns take the width of their longest word and share the rest; rows line up on a subgrid. -->
      <div
        style:min-width={width}
        style:grid-template-columns={template}
        class="grid divide-y divide-slate-200 text-[12px]/[16px] dark:divide-slate-600"
      >
        <div class="col-span-full grid grid-cols-subgrid gap-x-2 px-2 py-2 text-[11px]/[16px] text-text-alt">
          {#each node.columns as column, index (index)}
            <div class="wrap-break-word">{column}</div>
          {/each}
        </div>
        {#each node.rows as row, rowIndex (row.key)}
          <button
            type="button"
            class="col-span-full grid grid-cols-subgrid gap-x-2 px-2 py-2 text-left active:bg-black/5 dark:active:bg-white/5"
            aria-label={`${titleText(node.title)}, ${$LL.CREDENTIAL.RENDERER.STRUCTURED.ENTRY({ position: rowIndex + 1 })}`}
            onclick={() => openRow(node, rowIndex)}
          >
            {#each row.cells as cell, index (index)}
              <div class="wrap-break-word">{cell ?? '—'}</div>
            {/each}
          </button>
        {/each}
      </div>
    </div>
    {#if node.columns.length > VISIBLE_COLUMNS}
      <!-- More columns to the right. -->
      <div
        class="pointer-events-none absolute inset-y-px right-px w-8 rounded-r-lg bg-linear-to-l from-background to-transparent"
      ></div>
    {/if}
  </div>
{/snippet}

{#snippet chips(values: string[])}
  <div class="flex flex-wrap gap-1.5">
    {#each values as chip, index (index)}
      <span class="rounded-full bg-slate-200 px-2.5 py-0.5 text-[12px]/[20px] dark:bg-slate-600">{chip}</span>
    {/each}
  </div>
{/snippet}

{#snippet row(node: ClaimNode)}
  {#if node.kind === 'value'}
    <div class="px-3 py-2.5">
      {@render label(titleText(node.title))}
      <div class="wrap-break-word"><ClaimValue key={titleText(node.title)} value={node.value} /></div>
    </div>
  {:else if node.kind === 'chips'}
    <div class="flex flex-col gap-1.5 px-3 py-2.5">
      {@render label(titleText(node.title))}
      {@render chips(node.values)}
    </div>
  {:else}
    {@render fold(node)}
  {/if}
{/snippet}

{#snippet fold(node: ClaimGroup | ClaimTable)}
  {@const open = isOpen(node)}
  <div>
    {@render foldRow(node, open)}
    {#if open}
      <div transition:slide={{ duration: 200 }}>
        {#if node.kind === 'table'}
          {@render table(node)}
        {:else}
          <!-- Children sit behind a guide line, so the depth stays readable. -->
          <div
            class="{node.depth === 0
              ? 'ml-4'
              : 'ml-3'} divide-y divide-slate-200 border-t border-l-2 border-t-slate-200 border-l-primary dark:divide-slate-600 dark:border-t-slate-600"
          >
            {#each node.children as child (child.key)}
              {@render row(child)}
            {/each}
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/snippet}

{#if tree}
  <div class="overflow-hidden rounded-xl bg-background text-[13px]/[24px]">
    {#if tree.kind === 'chips'}
      <div class="flex flex-col gap-1.5 px-4 py-3">
        <h2 class="font-medium text-text-alt">{titleText(tree.title)}</h2>
        {@render chips(tree.values)}
      </div>
    {:else if tree.kind === 'value'}
      <div class="px-4 py-3">
        <h2 class="font-medium text-text-alt">{titleText(tree.title)}</h2>
        <p class="wrap-break-word"><ClaimValue key={titleText(tree.title)} value={tree.value} /></p>
      </div>
    {:else}
      {@render fold(tree)}
    {/if}
  </div>
{/if}

<!-- Mounted once a Table row is opened, so a claim without a Table carries no dialog. -->
{#if sheet}
  <ActionSheet open={sheetOpen} titleText={sheet.title} descriptionText={sheet.entry}>
    <div slot="content" class="mt-4 w-full">
      <div
        class="divide-y divide-slate-200 rounded-xl bg-background text-[13px]/[20px] text-slate-800 dark:divide-slate-600 dark:text-grey"
      >
        {#each sheet.pairs as [pairLabel, pairValue], index (index)}
          <div class="px-3 py-2.5">
            {@render label(pairLabel)}
            <div class="wrap-break-word">
              {#if pairValue}<ClaimValue key={pairLabel} value={pairValue} />{:else}—{/if}
            </div>
          </div>
        {/each}
      </div>
    </div>
  </ActionSheet>
{/if}

<style>
  /* Hide the scrollbar of a wide Table. */
  .table-scroll {
    scrollbar-width: none;
  }
  .table-scroll::-webkit-scrollbar {
    display: none;
  }
</style>
