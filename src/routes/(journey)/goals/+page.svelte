<script lang="ts">
  import { Button, ProgressBar } from '@impierce/ui-components';
  import CheckCircle from '~icons/lucide/check-circle';
  import Circle from '~icons/lucide/circle';
  import exampleJourneyDefinition from '$lib/example/data/journey-definition.json';
  import type { Goal } from './types';

  // TODO: extract component
  import {
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger
  } from '@impierce/ui-components';
  let sheetOpen: boolean = false;

  let goals: Goal[] = exampleJourneyDefinition.goals.map((goal) => ({
    ...goal,
    completed: false
  }));

  // TODO: remove hardcoded completions (obviously!)
  goals.at(0)!!.completed = true;
  goals.at(1)!!.completed = true;
</script>

<div class="flex h-full flex-col bg-slate-100">
  <div class="px-4 pb-1 pt-4">
    <ProgressBar value={40} />
  </div>
  <div class="flex flex-col items-center justify-center p-8">
    <p class="pt-8 text-3xl font-semibold">{exampleJourneyDefinition.title}</p>
    <p class="pt-4 text-center font-medium text-slate-500">
      {exampleJourneyDefinition.description}
    </p>

    <div class="flex flex-col items-start space-y-4 py-12">
      {#each goals as goal}
        {#if goal.completed}
          <button
            class="flex w-full items-center rounded-lg bg-violet-200 px-4 py-4"
            on:click={() => (sheetOpen = !sheetOpen)}
          >
            <CheckCircle class="mr-4 h-6 w-6 text-violet-700 " />
            <p class="font-medium text-violet-700">{goal.label}</p>
          </button>
        {:else}
          <button
            class="flex w-full items-center rounded-lg bg-slate-200 px-4 py-4"
            on:click={() => (sheetOpen = !sheetOpen)}
          >
            <Circle class="mr-4 h-6 w-6 text-slate-500" />
            <p class="font-medium text-slate-500">{goal.label}</p>
          </button>
        {/if}
      {/each}
    </div>
    <Button on:click={() => (sheetOpen = !sheetOpen)}>Continue</Button>
  </div>
</div>

<Sheet open={sheetOpen}>
  <!-- <SheetTrigger>
    <Button variant="outline">Open sheet (current sheetOpen: {sheetOpen})</Button>
  </SheetTrigger> -->
  <SheetContent position="bottom" size="content">
    <SheetHeader>
      <SheetTitle>{goals.at(2)?.label}</SheetTitle>
      <SheetDescription>
        <p class="text-sm rounded bg-orange-100 px-4 py-2 font-medium text-orange-600">
          TODO: bind SheetTrigger to goal.id
        </p>
      </SheetDescription>
    </SheetHeader>
    <!-- <div class="grid gap-4 py-4">
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="name" class="text-right">Name</Label>
        <Input id="name" value="Pedro Duarte" class="col-span-3" />
      </div>
      <div class="grid grid-cols-4 items-center gap-4">
        <Label for="username" class="text-right">Username</Label>
        <Input id="username" value="@peduarte" class="col-span-3" />
      </div>
    </div> -->
    <div class="p-8 font-medium text-slate-500">
      {goals.at(2)?.description}
    </div>
    <SheetFooter>
      <SheetClose>
        <Button href="/goals/2/faqs" class="mb-8">Ok, let's go!</Button>
      </SheetClose>
    </SheetFooter>
  </SheetContent>
</Sheet>
