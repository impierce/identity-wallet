<script lang="ts">
  import { Button, Switch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    ArrowCounterClockwiseBoldIcon,
    CheckBoldIcon,
    InfoRegularIcon,
    PlusBoldIcon,
    TrashRegularIcon,
  } from '$lib/icons';
  import { state } from '$lib/stores';

  $: entries = $state.trust_list || {};
  $: domains = Object.keys(entries);

  let newEntryInputElement: HTMLInputElement;
  let showNewEntry = false;
  let newEntryValue = '';
</script>

<TopNavBar on:back={() => history.back()} title={'Trust lists'} />
<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <div class="space-y-[15px] px-4 py-5">
    <!-- Developer info -->
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
      <span class="mr-4 h-6 w-6">
        <InfoRegularIcon class="h-6 w-6 text-primary" />
      </span>
      <div class="flex flex-col">
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Developer info</p>
        <ul class="ml-3 list-disc text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
          <li>Verifiable Presentations are trusted based on the domains from the list below.</li>
          <li>You can add, remove, update or disable entries.</li>
          <li>All edits can be reset to the default trust list.</li>
        </ul>
      </div>
    </div>

    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Trusted domains</p>
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
                    type: '[Trust List] Edit',
                    payload: { old_domain: domain, new_domain: domains[i] },
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
              on:click={() => dispatch({ type: '[Trust List] Delete', payload: { domain } })}
            >
              <TrashRegularIcon class="h-5 w-5 text-rose-500 dark:text-rose-400" />
            </button>
          {/if}
          <div class="flex items-center">
            <!-- `active` prop in Switch is not reactive, so triggering a rerender manually using `#key` -->
            {#key active}
              <Switch {active} on:change={() => dispatch({ type: '[Trust List] Toggle', payload: { domain } })} />
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
              dispatch({ type: '[Trust List] Add', payload: { domain: newEntryValue } });
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
      <Button label="Reset to default list" variant="secondary" />
    </div>
  </div>
</div>
