<script>
  import { Avatar, ProgressBar } from '@impierce/ui-components';
  import { ChevronRight } from 'svelte-heros-v2';
  import { state } from '../stores';
</script>

<div class="bg-gradient-to-br from-blue-600 to-violet-600 p-8 sticky top-0 h-[220px]">
  <div class="">
    <Avatar
      size="small"
      initials="FR"
    />
  </div>
  <p class="py-4 text-xl font-semibold text-white">Welcome back, Ferris!</p>
  <div class="flex items-center justify-between">
    <div class="grow">
      <div class="flex justify-between py-2 text-white">
        <p>Complete your first steps</p>
        <p>2/5</p>
      </div>
      <!-- <ProgressBar value={40} /> -->
      <!-- TODO: begin: custom progress bar, replace with ui-components/ProgressBar -->
      <div class="relative pt-1">
        <div class="mb-4 flex h-2 overflow-hidden rounded-full bg-violet-100 text-xs">
          <div
            style="width:40%"
            class="flex flex-col justify-center whitespace-nowrap rounded-full bg-gradient-to-r from-pink-500 to-pink-500 text-center text-white shadow-none"
          />
        </div>
      </div>
      <!-- end: custom progress bar -->
    </div>
    <button class="rounded-full py-4 pl-8">
      <ChevronRight class="text-white" strokeWidth="2" />
    </button>
  </div>
</div>
