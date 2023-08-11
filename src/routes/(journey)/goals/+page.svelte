<script lang="ts">
  import { BottomDrawer, Button, TopNavigation } from '@impierce/ui-components';
  import { state } from '../../../stores';
  import type { Goal } from './types';
  import { melt } from '@melt-ui/svelte';
  import Trophy from '~icons/ph/trophy-fill';
  import GoalItem from '$lib/journey/goals/GoalItem.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { goto } from '$app/navigation';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import LL from '$i18n/i18n-svelte';
  import ProgressBar from '$lib/components/ProgressBar.svelte';

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
  <BottomDrawer titleText="Skip onboarding" descriptionText="Are you sure?">
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="w-full py-4 text-left text-[13px]/[24px] font-medium text-indigo-500"
      >{$LL.SKIP()}</button
    >
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
<div class="flex h-full flex-col bg-neutral-100">
  <div class="h-[54px] bg-white py-[15px] px-[18px] flex items-center">
    <span class="grow pr-[15px]">
      <ProgressBar value={40} />
    </span>
    <p class="text-slate-800 font-medium text-[13px]/[24px]">3/7</p>
  </div>

  <div class="flex h-full flex-col items-center justify-between p-6">
    <div class="flex flex-col items-center">
      <!-- Header -->
      <PaddedIcon icon={Trophy} />
      <p class="pt-8 text-2xl font-semibold">{journeyDefinition?.title}</p>
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
              <button
                class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
                on:click={() => goto(`/goals/${goal.id}/faqs`)}>Start</button
              >
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
    <button
      class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
      on:click={() => goto('/goals/0/faqs')}>Continue</button
    >
  </div>
</div>
