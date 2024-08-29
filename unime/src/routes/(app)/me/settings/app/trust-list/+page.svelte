<script lang="ts">
  import { goto } from '$app/navigation';

  import type { TrustList } from '@bindings/trust_list/TrustList';

  import { Button, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CaretRightBoldIcon, InfoRegularIcon, StarFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  $: trustLists = $state.trust_lists || {};

  const activeDomainsString = (trustList: TrustList): string => {
    const totalEntries = Object.keys(trustList.trust_list).length;
    const enabledEntries = Object.values(trustList.trust_list).filter((active) => active === true).length;
    const disabledEntries = Object.values(trustList.trust_list).filter((active) => active === false).length;
    if (totalEntries === 0) {
      return 'Empty list';
    } else if (enabledEntries === totalEntries) {
      if (totalEntries === 1) {
        return '1 active domain';
      } else {
        return `${totalEntries} domains, all active`;
      }
    } else if (disabledEntries === totalEntries) {
      return 'Inactive';
    } else {
      return `${enabledEntries} of ${totalEntries} active`;
    }
  };
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
          <li>Verifiable Presentations are trusted based on domains found in trust lists.</li>
          <li>You can add, remove and update custom entries.</li>
          <li>All entries can be manually disabled.</li>
          <!-- <li>All edits can be reset to the default trust list.</li> -->
        </ul>
      </div>
    </div>

    <div class="flex flex-col space-y-[10px]">
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Trust lists</p>
      {#each trustLists as trustList}
        <button
          class="flex h-14 w-full items-center space-x-4 rounded-xl bg-white p-4 dark:bg-dark"
          on:click={() => goto(`/me/settings/app/trust-list/${trustList.name}`)}
        >
          <div class="flex grow flex-row items-center space-x-2">
            <p class="text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
              {trustList.name}
            </p>
            {#if trustList.owned}
              <svelte:component this={StarFillIcon} class="h-5 w-5 text-primary" />
            {/if}
          </div>
          <p class="text-[13px]/[24px] font-medium text-slate-400 dark:text-slate-300">
            {activeDomainsString(trustList)}
          </p>
          <svelte:component this={CaretRightBoldIcon} class="h-4 w-4 text-slate-500" />
        </button>
      {/each}
    </div>
    <Button
      label="Add new custom trust list"
      on:click={() => {
        dispatch({
          type: '[Trust List] Add',
          payload: { trust_list_id: `custom_list_${Math.floor(Math.random() * 100_000)}` },
        });
      }}
    />
  </div>
</div>
