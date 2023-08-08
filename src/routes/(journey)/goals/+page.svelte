<script lang="ts">
  import { BottomDrawer, Button, ProgressBar } from '@impierce/ui-components';
  import { state } from '../../../stores';
  import type { Goal } from './types';
  import { melt } from '@melt-ui/svelte';
  import Trophy from '~icons/ph/trophy-fill';

  import GoalItem from '$lib/journey/goals/GoalItem.svelte';

  let journeyDefinition = $state?.user_journey;

  let goals: Goal[] = journeyDefinition.goals.map((goal) => ({
    ...goal,
    completed: false
  }));

  // TODO: remove hardcoded completions (obviously!)
  goals.at(0)!!.completed = true;
  goals.at(1)!!.completed = true;
</script>

<div class="flex h-full flex-col bg-neutral-100">
  <div class="px-4 pb-1 pt-4">
    <!-- TODO: ProgressBar -->
  </div>
  <div class="flex flex-col items-center justify-center p-6">
    <div class="rounded-2xl bg-indigo-500 p-4"><Trophy class="h-8 w-8 text-white" /></div>

    <p class="pt-8 text-2xl font-semibold">{journeyDefinition.title}</p>
    <p class="pt-4 text-center font-medium text-slate-500">
      {journeyDefinition.description}
    </p>

    <div class="flex w-full flex-col space-y-4 py-12">
      {#each goals as goal}
        <BottomDrawer titleText={goal.label} descriptionText={goal.description}>
          <svelte:fragment slot="trigger" let:trigger>
            <GoalItem {trigger} {...goal} />
          </svelte:fragment>

          <svelte:fragment slot="content">
            <button class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white">Start</button>
          </svelte:fragment>

          <button
            slot="close"
            let:close
            class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
            use:melt={close}>Close</button
          >
        </BottomDrawer>
      {/each}
    </div>
    <button
      class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
      on:click={() => console.log('TODO: determine first incomplete item in list')}>Continue</button
    >
  </div>
</div>
