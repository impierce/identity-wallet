<script lang="ts">
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import Button from '$lib/components/Button.svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import GoalItem from '$lib/journey/goals/GoalItem.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import Trophy from '~icons/ph/trophy-fill';

  import type { Goal } from './types';

  let journeyDefinition = $state?.user_journey;

  let goals: Goal[] =
    journeyDefinition?.goals.map((goal) => ({
      ...goal,
      completed: false // TODO: should be determined by the backend
    })) ?? [];
</script>

<!-- Navbar -->
<TopNavigation on:back={() => history.back()} title={$state?.user_journey?.title}>
  <!-- TODO: replace BottomDrawer with AlertDialog -->
  <BottomDrawer
    titleText={$LL.GETTING_STARTED.SKIP_TITLE()}
    descriptionText={$LL.GETTING_STARTED.SKIP_TEXT()}
  >
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary">{$LL.SKIP()}</button
    >
    <!-- <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary"
    >
      <div class="h-6 w-6 bg-slate-200" />
    </button> -->
    <button
      slot="content"
      class="w-full rounded-lg bg-red-100 px-4 py-2 text-red-600"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Yes</button
    >
    <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      >No, let's continue</button
    >
  </BottomDrawer>
</TopNavigation>

<!-- Content -->
<div class="flex h-full flex-col bg-bg-secondary dark:bg-bg-dark-secondary">
  <div class="flex h-[54px] items-center bg-bg-primary px-[18px] py-[15px] dark:bg-bg-dark-primary">
    <span class="grow pr-[15px]">
      <ProgressBar value={40} />
    </span>
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-white">3/7</p>
  </div>

  <div
    class="hide-scrollbar flex h-full flex-col items-center justify-between overflow-y-scroll p-6"
    in:fly={{ x: 32, opacity: 1 }}
  >
    <div class="flex flex-col items-center">
      <!-- Header -->
      <PaddedIcon icon={Trophy} />
      <p class="pt-8 text-2xl font-semibold dark:text-white">{journeyDefinition?.title}</p>
      <p class="pt-4 text-center font-medium text-slate-500">
        {journeyDefinition?.description}
      </p>

      <!-- Goal items -->
      <div class="flex w-full flex-col space-y-4 py-12">
        {#each goals as goal}
          <BottomDrawer titleText={goal.label} descriptionText={goal.description}>
            <svelte:fragment slot="trigger" let:trigger>
              <GoalItem {trigger} label={goal.label} completed={goal.completed} />
            </svelte:fragment>

            <svelte:fragment slot="content">
              <!-- <button
                class="w-full rounded-lg bg-primary px-4 py-2 text-white"
                on:click={() => goto(`/goals/${goal.id}/faqs`)}>Start</button
              > -->
              <Button label="Start" on:click={() => goto(`/goals/${goal.id}/faqs`)} />
            </svelte:fragment>

            <button
              slot="close"
              let:close
              use:melt={close}
              class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
              >Close</button
            >
          </BottomDrawer>
        {/each}
      </div>
    </div>

    <!-- 'TODO: determine first incomplete item in list' -->
    <!-- <button
      class="w-full rounded-lg bg-primary px-4 py-2 text-white"
      on:click={() => goto('/goals/0/faqs')}>Continue</button
    > -->
  </div>
  <div class="sticky bottom-[var(--safe-area-inset-bottom)] left-0 p-6">
    <Button label={$LL.CONTINUE()} on:click={() => goto('/goals/0/faqs')} />
  </div>
</div>
