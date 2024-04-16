<script lang="ts">
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import ProgressBar from '$src/lib/components/atoms/ProgressBar.svelte';
  import ActionSheet from '$src/lib/components/molecules/dialogs/ActionSheet.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import GoalItem from '$src/lib/journeys/goals/GoalItem.svelte';
  import { icons } from '$src/lib/journeys/goals/icons';
  import { state } from '$src/stores';

  import Trophy from '~icons/ph/trophy-fill';

  import type { Goal } from './types';

  let journeyDefinition = $state?.user_journey;

  let goals: Goal[] =
    journeyDefinition?.goals.map((goal) => ({
      ...goal,
      completed: true, // TODO: should be determined by the backend
    })) ?? [];

  goals.at(2).completed = false;

  let completedPercentage = Math.round((goals.filter((goal) => goal.completed).length / goals.length) * 100);
</script>

<!-- Navbar -->
<TopNavBar on:back={() => history.back()} title={$state?.user_journey?.title}>
  <!-- TODO: replace ActionSheet with AlertDialog -->
  <ActionSheet titleText={$LL.GETTING_STARTED.SKIP_TITLE()} descriptionText={$LL.GETTING_STARTED.SKIP_TEXT()}>
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
    <!-- <button
      slot="content"
      class="w-full rounded-lg bg-red-100 px-4 py-2 text-red-600"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Yes</button
    > -->
    <!-- <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      >No, let's continue</button
    > -->
    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <Button label="Yes" on:click={() => dispatch({ type: '[User Journey] Cancel' })} />
    </div>
    <Button variant="secondary" slot="close" let:close trigger={close} label="No, let's continue" />
  </ActionSheet>
</TopNavBar>

<!-- Content -->
<div class="flex h-full flex-col bg-silver dark:bg-navy">
  <div class="flex h-[54px] items-center bg-silver px-[18px] py-[15px] dark:bg-navy">
    <span class="grow pr-[15px]">
      <ProgressBar value={completedPercentage} />
    </span>
    <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
      {goals.filter((goal) => goal.completed).length}/{goals.length}
    </p>
  </div>

  <div
    class="hide-scrollbar flex h-full flex-col items-center justify-between overflow-y-scroll p-6"
    in:fly={{ x: 32, opacity: 1 }}
  >
    <div class="flex flex-col items-center">
      <!-- Header -->
      <PaddedIcon icon={Trophy} />
      <p class="pt-8 text-[22px]/[30px] font-semibold text-slate-800 dark:text-white">
        {journeyDefinition?.title}
      </p>
      <p class="w-3/4 pt-4 text-center text-[13px]/[24px] font-normal text-slate-500">
        {journeyDefinition?.description}
      </p>

      <!-- Goal items -->
      <div class="flex w-full flex-col space-y-4 py-8">
        {#each goals as goal}
          <ActionSheet titleText={goal.label} descriptionText={goal.description}>
            <svelte:fragment slot="trigger" let:trigger>
              <GoalItem {trigger} label={goal.label} completed={goal.completed} icon={goal.icon} />
            </svelte:fragment>

            <svelte:fragment slot="icon">
              <div class="mb-[15px] flex h-[75px] w-[75px] items-center justify-center rounded-3xl bg-slate-100">
                <svelte:component this={icons[goal.icon] || icons['Trophy']} class="h-7 w-7 text-primary" />
              </div>
            </svelte:fragment>

            <svelte:fragment slot="content">
              <div class="w-full pb-[10px] pt-8">
                <!-- <button
                class="w-full rounded-lg bg-primary px-4 py-2 text-white"
                on:click={() => goto(`/goals/${goal.id}/faqs`)}>Start</button
              > -->
                <Button label="Start" on:click={() => goto(`/goals/${goal.id}/faqs`)} />
              </div>
            </svelte:fragment>

            <!-- <div class="mt-2"> -->
            <Button variant="secondary" slot="close" let:close trigger={close} label="Close" />
            <!-- </div> -->
            <!-- <button
              slot="close"
              let:close
              use:melt={close}
              class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
              >Close</button
            > -->
          </ActionSheet>
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
