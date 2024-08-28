<script lang="ts">
  import { goto } from '$app/navigation';

  import { TopNavBar } from '$lib/components';
  import { CaretRightBoldIcon, InfoRegularIcon, ListStarFillIcon, StarFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  $: trustLists = $state.trust_lists || {};
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
          {#if trustList.owned}
            <svelte:component this={StarFillIcon} class="h-5 w-5 text-primary" />
          {:else}
            <!-- <svelte:component this={ListStarFillIcon} class="h-5 w-5 text-slate-500" /> -->
          {/if}
          <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
            {trustList.name}
          </p>
          <!-- {#if hasCaretRight} -->
          <svelte:component this={CaretRightBoldIcon} class="h-4 w-4 text-slate-500" />
          <!-- {:else if textRight}
    <p class="text-[13px]/[24px] font-medium text-primary">{textRight}</p>
  {/if} -->
          <!-- Slot for any content which is not text or caret, such as a Switch -->
          <slot />
        </button>

        <!-- <SettingsEntry
          icon={ListStarFillIcon}
          title={trustList.name}
          on:click={() => goto(`/me/settings/app/trust-list/${trustList.name}`)}
        >
          {#if trustList.owned}
            Custom
          {/if}
        </SettingsEntry> -->
      {/each}
      <!-- TODO: refactor below -->
    </div>
  </div>
</div>
