<script lang="ts">
  import { page } from '$app/stores';

  import { melt } from '@melt-ui/svelte';

  import { ActionSheet, Button, Switch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { ArrowCounterClockwiseBoldIcon, CheckBoldIcon, PlusBoldIcon, TrashRegularIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  $: trustList = $state.trust_lists.find((tl) => tl.id === $page.params.id);
  $: entries = trustList?.entries || {};
  $: domains = Object.keys(entries);

  let newEntryInputElement: HTMLInputElement;
  let showNewEntry = false;
  let newEntryValue = '';
</script>

<TopNavBar on:back={() => history.back()} title={trustList?.display_name ?? ''}>
  <ActionSheet
    titleText={'Delete trust list'}
    descriptionText={'Are you sure you want to delete this list of trusted issuers?'}
  >
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary">Delete</button
    >
    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <Button
        label="Yes, delete it."
        on:click={() => {
          dispatch({ type: '[Trust Lists] Delete', payload: { trust_list_id: $page.params.id } });
          history.back();
        }}
      />
    </div>
    <Button variant="secondary" slot="close" let:close trigger={close} label="No, keep the list." />
  </ActionSheet>
</TopNavBar>
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="space-y-[15px] px-4 py-5">
    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Trusted issuers</p>
      <!-- {#each Object.entries(entries) as [domain, active], i (domain)}
        <div class="flex flex-row items-center space-x-2">
          <div class="flex h-14 w-full items-center justify-between space-x-4 rounded-xl bg-white p-4 dark:bg-dark">
            <p class={`text-sm font-medium text-slate-500 ${active ? '' : 'opacity-30'}`}>{domain}</p>
            <Switch
              {active}
              on:change={() =>
                dispatch({ type: '[Trust List] Toggle Entry', payload: { trust_list_id: $page.params.id, domain } })}
            />
          </div>
        </div>
      {/each} -->
    </div>

    {#if trustList?.custom}
      <!-- Custom list -->
      {#each Object.entries(entries) as [domain, active], i (domain)}
        <div class="flex flex-row items-center space-x-2">
          <input
            type="text"
            class="h-12 grow rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-teal disabled:text-slate-400 disabled:opacity-60 dark:border-slate-600 dark:bg-dark"
            value={domains[i]}
            on:input={(e) => (domains[i] = e.target.value)}
            disabled={!active}
          />
          {#if domains[i] !== domain}
            <div class="flex flex-row">
              <button
                class="rounded-full p-2"
                on:click={() =>
                  dispatch({
                    type: '[Trust List] Edit Entry',
                    payload: { trust_list_id: $page.params.id, old_domain: domain, new_domain: domains[i] },
                  })}
              >
                <CheckBoldIcon class="h-5 w-5 text-primary" />
              </button>
              <button class="rounded-full p-2 active:bg-slate-100" on:click={() => (domains[i] = domain)}>
                <ArrowCounterClockwiseBoldIcon class="h-5 w-5 text-slate-400" />
              </button>
            </div>
          {:else}
            <button
              class="rounded-full p-2"
              on:click={() =>
                dispatch({ type: '[Trust List] Delete Entry', payload: { trust_list_id: $page.params.id, domain } })}
            >
              <TrashRegularIcon class="h-5 w-5 text-rose-500 dark:text-rose-400" />
            </button>
          {/if}
          <div class="flex items-center">
            <!-- `active` prop in Switch is not reactive, so triggering a rerender manually using `#key` -->
            {#key active}
              <Switch
                {active}
                on:change={() =>
                  dispatch({ type: '[Trust List] Toggle Entry', payload: { trust_list_id: $page.params.id, domain } })}
              />
            {/key}
          </div>
        </div>
      {/each}
      {#if showNewEntry}
        <div class="flex flex-row items-center space-x-2">
          <input
            type="text"
            class="h-12 grow rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-teal dark:border-slate-600 dark:bg-dark"
            placeholder="example.org"
            bind:this={newEntryInputElement}
            bind:value={newEntryValue}
          />
          <button
            class="rounded-full p-2 disabled:opacity-50 disabled:grayscale"
            on:click={() => {
              dispatch({
                type: '[Trust List] Add Entry',
                payload: { trust_list_id: $page.params.id, domain: newEntryValue },
              });
              newEntryValue = '';
              showNewEntry = false;
            }}
            disabled={!newEntryValue}
          >
            <PlusBoldIcon class="h-5 w-5 text-primary" />
          </button>
          <!-- Placeholder with the same dimensions of a Switch for better visual alignment -->
          <div class="w-11"></div>
        </div>
      {:else}
        <Button
          label="Add trusted domain"
          on:click={() => {
            showNewEntry = true;
            newEntryInputElement.focus();
          }}
          disabled={showNewEntry}
        />
      {/if}
    {:else}
      <!-- Imported list -->
      {#each Object.entries(entries) as [domain, active] (domain)}
        <div class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark">
          <p
            class={`grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white ${active ? '' : 'opacity-50'}`}
          >
            {domain}
          </p>
          {#key active}
            <Switch
              {active}
              on:change={() =>
                dispatch({ type: '[Trust List] Toggle Entry', payload: { trust_list_id: $page.params.id, domain } })}
            />
          {/key}
        </div>
      {/each}
    {/if}
  </div>
</div>
