<script lang="ts">
  import { goto } from '$app/navigation';

  import ProgressBar from '$lib/components/ProgressBar.svelte';
  import { state } from '$src/stores';

  import CaretRight from '~icons/ph/caret-right-bold';

  // TODO: if user_journey.goals, then calculate completed percentage
</script>

{#if $state?.user_journey}
  <div class="flex items-center justify-between pt-[6px]">
    <div class="grow">
      <div class="flex justify-between text-sm font-medium text-slate-800 dark:text-white">
        <p>{$state.user_journey.description_short}</p>
        <!-- <p>? / {$state.user_journey.goals.length}</p> -->
      </div>
      <!-- <div class="py-[7px]"> -->
      <div class="flex items-center">
        <div class="grow">
          <ProgressBar value={(2 / 3) * 100} />
        </div>
        <button class="ml-[15px]" on:click={() => goto('/goals')}>
          <CaretRight class="h-6 w-6 text-primary" />
        </button>
      </div>
      <!-- <ProgressBar value={40} /> -->
      <!-- TODO: begin: custom progress bar, replace with ui-components/ProgressBar -->
      <!-- <div class="relative pt-1">
        <div class="flex h-2 overflow-hidden rounded-full bg-violet-100 text-xs">
          <div
            style="width:40%"
            class="flex flex-col justify-center whitespace-nowrap rounded-full bg-primary text-center text-white shadow-none"
          />
        </div>
      </div> -->
      <!-- end: custom progress bar -->
    </div>

    <!-- <button class="-mr-4 rounded-full p-5" on:click={() => goto('/goals')}>
      <ChevronRight class="h-6 w-6 text-primary" strokeWidth="2" />
    </button> -->
  </div>
{/if}
