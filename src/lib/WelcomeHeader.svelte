<script>
  import { Avatar, ProgressBar } from '@impierce/ui-components';
  import { ChevronRight, Cog } from 'svelte-heros-v2';
  import { state } from '../stores';
  import { goto } from '$app/navigation';
  import Settings from '~icons/lucide/settings';
  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  import LL from '$i18n/i18n-svelte';
  // import DotsThreeVertical from '~icons/ph/dots-three-outline-vertical-fill';
</script>

<div class="bg-white p-6">
  <div class="flex items-center justify-between">
    <!-- <Avatar size="small" initials="FR" /> -->
    <div class="h-12 w-12 rounded-2xl bg-indigo-600 p-2">
      <div class="flex h-full w-full items-center justify-center font-medium text-neutral-100">
        FR
      </div>
    </div>
    <button
      on:click={() => goto('/me/settings')}
      class="-mx-4 rounded-full p-4 text-black hover:bg-white hover:text-black"
    >
      <DotsThreeVertical class="h-8 w-8" />
      <!-- <Cog class="h-6 w-6" /> -->
    </button>
  </div>
  <p class="pt-4 text-3xl font-semibold text-black">
    {$LL.WELCOME()}, <span class="text-indigo-500">{$state?.active_profile?.display_name}.</span>
  </p>

  {#if $state?.user_journey}
    <div class="flex items-center justify-between">
      <div class="grow">
        <div class="flex justify-between py-2 text-black">
          <p>{$state.user_journey.description_short}</p>
          <p>2 / 5</p>
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
</div>
