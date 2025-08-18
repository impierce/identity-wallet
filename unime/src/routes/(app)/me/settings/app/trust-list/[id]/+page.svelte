<script lang="ts">
  import { page } from '$app/stores';

  import { melt } from '@melt-ui/svelte';
  import { redirect } from '@sveltejs/kit';

  import { ActionSheet, Button, DeprecatedSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { ArrowCounterClockwiseBoldIcon, CheckBoldIcon, TrashRegularIcon } from '$lib/icons';
  import { error, state } from '$lib/stores';

  $: trustList = $state.trust_lists.find((tl) => tl.id === $page.params.id);
  $: entries = trustList?.entries || {};
  $: domains = Object.keys(entries);

  let showNewEntry = false;
  let newEntryValue = '';
  $: updatedListName = trustList?.display_name;

  const trust_list_id = $page.params.id;

  if (!trust_list_id) {
    $error = `No trust list found with id ${trust_list_id}.`;
    redirect(303, '/me/settings/app/trust-list');
  }

  function init(el: HTMLInputElement) {
    el.focus();
  }
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
          dispatch({ type: '[Trust Lists] Delete', payload: { trust_list_id } });
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
      {#if trustList?.custom}
        <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Update list name</p>
        <div class="flex space-x-[10px]">
          <input
            type="text"
            class="h-12 grow rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-secondary disabled:text-slate-400 disabled:opacity-60 dark:border-slate-600 dark:bg-dark"
            value={trustList?.display_name}
            on:input={(e: Event) => (updatedListName = (e.target as HTMLInputElement).value)}
          />
          <div class="ml-2 w-[88px]">
            <Button
              label="Save"
              on:click={() => {
                if (updatedListName) {
                  dispatch({
                    type: '[Trust Lists] Edit',
                    payload: { trust_list_id, new_display_name: updatedListName },
                  });
                }
              }}
              disabled={updatedListName === trustList?.display_name}
            />
          </div>
        </div>
      {/if}
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Trusted issuers</p>
    </div>

    {#if trustList?.custom}
      <!-- Custom list -->
      {#each Object.entries(entries) as [domain, active], i (domain)}
        <div class="flex flex-row items-center space-x-2">
          <input
            type="text"
            class="h-12 grow rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-secondary disabled:text-slate-400 disabled:opacity-60 dark:border-slate-600 dark:bg-dark"
            value={domains[i]}
            on:input={(e: Event) => (domains[i] = (e.target as HTMLInputElement).value)}
            disabled={!active}
          />
          {#if domains[i] !== domain}
            <div class="flex flex-row">
              <button
                class="rounded-full p-2"
                on:click={() =>
                  dispatch({
                    type: '[Trust List] Edit entry',
                    payload: { trust_list_id, old_domain: domain, new_domain: domains[i] },
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
              on:click={() => dispatch({ type: '[Trust List] Delete entry', payload: { trust_list_id, domain } })}
            >
              <TrashRegularIcon class="h-5 w-5 text-rose-500 dark:text-rose-400" />
            </button>
          {/if}
          <div class="flex items-center">
            <!-- `active` prop in Switch is not reactive, so triggering a rerender manually using `#key` -->
            <DeprecatedSwitch
              {active}
              on:change={() => {
                // dispatch({ type: '[Trust List] Toggle entry', payload: { trust_list_id: $page.params.id, domain } });
              }}
            />
          </div>
        </div>
      {/each}
      {#if showNewEntry}
        <div class="flex flex-row items-center space-x-2">
          <input
            type="text"
            class="h-12 grow rounded-xl border border-slate-200 px-3 text-[13px]/[24px] text-secondary dark:border-slate-600 dark:bg-dark"
            placeholder="example.org"
            bind:value={newEntryValue}
            use:init
          />
        </div>
        <Button
          label="Save"
          on:click={() => {
            dispatch({
              type: '[Trust List] Add entry',
              payload: { trust_list_id, domain: newEntryValue },
            });
            newEntryValue = '';
            showNewEntry = false;
          }}
          disabled={!newEntryValue}
        />
      {:else}
        <Button label="Add trusted domain" on:click={() => (showNewEntry = true)} disabled={showNewEntry} />
      {/if}
    {:else}
      <!-- Imported list -->
      {#each Object.entries(entries) as [domain, active] (domain)}
        <div class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark">
          <p
            class={`grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white ${active ? '' : 'opacity-50'}`}
          >
            {URL.parse(domain)?.hostname}
          </p>
          <DeprecatedSwitch
            {active}
            on:change={() => {
              // dispatch({ type: '[Trust List] Toggle entry', payload: { trust_list_id: $page.params.id, domain } });
            }}
          />
        </div>
      {/each}
    {/if}
  </div>
</div>
