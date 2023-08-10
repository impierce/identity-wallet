<script lang="ts">
  import { state } from '../../stores';
  import { ChevronRight, Cog } from 'svelte-heros-v2';
  import { goto } from '$app/navigation';

  // TODO: if user_journey.goals, then calculate completed percentage
</script>

{#if $state?.user_journey}
  <div class="flex items-center justify-between">
    <div class="grow">
      <div class="flex justify-between py-2 text-black">
        <p>{$state.user_journey.description_short}</p>
        <p>? / {$state.user_journey.goals.length}</p>
      </div>
      <!-- <ProgressBar value={40} /> -->
      <!-- TODO: begin: custom progress bar, replace with ui-components/ProgressBar -->
      <div class="relative pt-1">
        <div class="flex h-2 overflow-hidden rounded-full bg-violet-100 text-xs">
          <div
            style="width:40%"
            class="flex flex-col justify-center whitespace-nowrap rounded-full bg-indigo-500 text-center text-white shadow-none"
          />
        </div>
      </div>
      <!-- end: custom progress bar -->
    </div>
    <button class="-mr-4 rounded-full p-5" on:click={() => goto('/goals')}>
      <ChevronRight class="h-6 w-6 text-indigo-500" strokeWidth="2" />
    </button>
  </div>
{/if}
