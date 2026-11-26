<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { Connection } from '@bindings/connections/Connection';
  import type { IssuedCredential } from '@bindings/connections/IssuedCredential';

  import LogoBadge from '$lib/components/ecosystems/LogoBadge.svelte';
  import LoadingSpinner from '$lib/components/LoadingSpinner.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { CheckCircleFillIcon, MagnifyingGlassBoldIcon, XBoldIcon } from '$lib/icons';
  import { state } from '$lib/stores';
  import { hashIndex } from '$lib/utils';

  export let connection: Connection;

  const PAGE_SIZE = 20;
  const SEARCH_DELAY = 300;
  // Whole class strings: Tailwind only emits classes it finds literally in the source.
  const TINTS = ['bg-primary', 'bg-slate-700', 'bg-slate-400'];

  let searchTerm = '';
  let requestedTerm = '';
  let items: IssuedCredential[] = [];
  let total = 0;
  let loading = false;
  let requestId = 0;
  let pendingRequest: number | undefined;
  let loaded = false;
  let unavailable = false;
  let sentinel: HTMLElement;
  let observer: IntersectionObserver | undefined;
  let timer: ReturnType<typeof setTimeout>;

  function load(offset: number) {
    loading = true;
    pendingRequest = ++requestId;
    dispatch({
      type: '[Connection] Load issued credentials',
      payload: {
        request_id: pendingRequest,
        connection_id: connection.id,
        search_term: requestedTerm,
        offset,
        limit: PAGE_SIZE,
      },
    });
  }

  function reload() {
    items = [];
    total = 0;
    loaded = false;
    unavailable = false;
    load(0);
  }

  // The backend keeps only the last requested page; this list collects the pages scrolled through so far. Every state
  // update carries that page, so only the answer to the pending request is taken, once.
  $: receive($state.issued_credentials);

  function receive(page: typeof $state.issued_credentials) {
    if (!page || page.request_id !== pendingRequest || page.connection_id !== connection.id) return;
    pendingRequest = undefined;
    items = page.offset === 0 ? page.items : [...items, ...page.items];
    total = page.total;
    unavailable = page.unavailable;
    loaded = true;
    loading = false;
  }

  function onSearchInput() {
    clearTimeout(timer);
    timer = setTimeout(() => {
      if (searchTerm.trim() === requestedTerm) return;
      requestedTerm = searchTerm.trim();
      reload();
    }, SEARCH_DELAY);
  }

  function clearSearch() {
    searchTerm = '';
    onSearchInput();
  }

  $: hasMore = loaded && !unavailable && items.length < total;

  onMount(() => {
    reload();
    observer = new IntersectionObserver(
      (entries) => {
        if (entries.some((entry) => entry.isIntersecting) && hasMore && !loading) load(items.length);
      },
      { rootMargin: '200px' },
    );
    observer.observe(sentinel);
  });

  onDestroy(() => {
    clearTimeout(timer);
    observer?.disconnect();
  });

  // An observed sentinel that stays in view after a page arrives does not fire again, so check after every page.
  $: if (hasMore && !loading && sentinel && isInView(sentinel)) load(items.length);

  function isInView(element: HTMLElement) {
    const { top } = element.getBoundingClientRect();
    return top < window.innerHeight + 200;
  }

  function initials(name: string) {
    return (name.match(/[\p{L}\p{N}]+/u)?.[0] ?? '?').slice(0, 2).toUpperCase();
  }
</script>

<!--
@component
The credentials a connection issues, read from its issuer metadata page by page as the list is scrolled, with search.
-->
<div class="flex flex-col">
  <p class="px-1 pb-3 text-[13px]/[20px] font-normal text-text-alt">
    {$LL.CONNECTION.ISSUED_CREDENTIALS.DESCRIPTION({ name: connection.name })}
  </p>

  <div class="relative flex items-center">
    <MagnifyingGlassBoldIcon class="absolute left-4 h-4 w-4 text-slate-800 dark:text-grey" />
    <input
      type="text"
      class="h-11 w-full rounded-full bg-white pr-10 pl-12 text-sm font-medium text-slate-500 placeholder-slate-500 dark:bg-dark dark:text-slate-300"
      placeholder={$LL.CONNECTION.ISSUED_CREDENTIALS.SEARCH_PLACEHOLDER()}
      data-testid="issued-credentials-search"
      bind:value={searchTerm}
      on:input={onSearchInput}
    />
    {#if searchTerm}
      <button
        class="absolute right-2 inline-flex h-4 w-4 items-center justify-center rounded-full p-4"
        aria-label={$LL.CANCEL()}
        on:click={clearSearch}
      >
        <XBoldIcon class="absolute h-4 w-4 text-slate-800 dark:text-grey" />
      </button>
    {/if}
  </div>

  {#if loaded && !unavailable && total > 0}
    <p class="px-1 pt-4 pb-2 text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.CONNECTION.ISSUED_CREDENTIALS.COUNT({ count: total })}
    </p>
  {/if}

  <ul class="space-y-2 pt-2" data-testid="issued-credentials-list">
    {#each items as item (item.id)}
      <li class="issued-credential flex h-16 items-center rounded-xl bg-white p-2 dark:bg-dark">
        <LogoBadge
          logoUri={item.logo_uri}
          initials={initials(item.name)}
          tint={TINTS[hashIndex(item.name, TINTS.length)]}
          class="mr-4 size-12 rounded-lg"
          textClass="text-[16px]/[24px]"
        />
        <div class="flex min-w-0 grow flex-col">
          <p class="line-clamp-1 text-[13px]/[18px] font-medium text-slate-800 dark:text-grey">{item.name}</p>
          {#if item.description}
            <p class="line-clamp-1 text-[12px]/[20px] font-medium text-slate-400 dark:text-slate-300">
              {item.description}
            </p>
          {/if}
        </div>
        {#if item.owned}
          <span
            class="ml-2 flex shrink-0 items-center gap-1 rounded-full bg-primary/10 px-2 py-1 text-[11px]/[16px] font-semibold text-primary"
            data-testid="issued-credential-owned"
          >
            <CheckCircleFillIcon class="size-3.5" />
            {$LL.CONNECTION.ISSUED_CREDENTIALS.OWNED()}
          </span>
        {/if}
      </li>
    {/each}
  </ul>

  {#if loaded && unavailable}
    <div class="flex flex-col items-center pt-8 text-center">
      <p class="text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.CONNECTION.ISSUED_CREDENTIALS.UNAVAILABLE({ name: connection.name })}
      </p>
      <button class="pt-3 text-[13px]/[20px] font-semibold text-primary" on:click={reload}>
        {$LL.INTERACTION_RESULT.TRY_AGAIN()}
      </button>
    </div>
  {:else if loaded && total === 0}
    <p class="pt-8 text-center text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      {requestedTerm
        ? $LL.CONNECTION.ISSUED_CREDENTIALS.NO_RESULTS({ term: requestedTerm })
        : $LL.CONNECTION.ISSUED_CREDENTIALS.EMPTY({ name: connection.name })}
    </p>
  {/if}

  <div bind:this={sentinel} class="flex justify-center py-4">
    {#if loading}
      <LoadingSpinner class="size-6" />
    {/if}
  </div>
</div>

<style>
  /* Rows scrolled far out of view skip layout and paint, so a long list stays cheap to render. */
  .issued-credential {
    content-visibility: auto;
    contain-intrinsic-size: auto 64px;
  }
</style>
