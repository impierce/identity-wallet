<script lang="ts">
  import { state } from '../../../stores';
  import LL from '$i18n/i18n-svelte';
  import { Avatar, BottomDrawer } from '@impierce/ui-components';
  import { fade, fly, slide } from 'svelte/transition';
  import QrCodeButton from '$lib/QrCodeButton.svelte';
  import { debug, info } from '@tauri-apps/plugin-log';
  import CredentialList from '$lib/CredentialList.svelte';
  import { calculate_initials } from './utils';
  import TopBar from '$lib/home-header/TopBar.svelte';
  import Favorites from '$lib/Favorites.svelte';
  import RocketLaunch from '~icons/ph/rocket-launch-fill';
  import AddButton from '$lib/credentials/AddButton.svelte';
  import { melt } from '@melt-ui/svelte';
  import '@lottiefiles/lottie-player';
  import { goto } from '$app/navigation';
  import WelcomeMessage from '$lib/home-header/WelcomeMessage.svelte';
  import UserJourney from '$lib/home-header/UserJourney.svelte';
  import { onMount } from 'svelte';
  import NoCredentials from '$lib/credentials/NoCredentials.svelte';
  import Button from '$lib/components/Button.svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';

  let initials: string | undefined;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('routes/(app)/me/+page.svelte: state', $state);
    if ($state?.active_profile?.display_name) {
      initials = calculate_initials($state?.active_profile?.display_name);
    }
  }
</script>

<div class="flex min-h-full flex-col">
  <div class="sticky top-0 z-10 h-[56px] w-full">
    <TopBar />
  </div>

  <div class="p-[18px] pt-0">
    <WelcomeMessage />
    {#if $state?.user_journey}
      <UserJourney />
    {/if}
  </div>

  <!-- should have min height: full screen - smallest possible welcome header - bottom nav - safe areas (top, bottom) -->
  <div
    in:fly={{ y: 24 }}
    class="flex grow flex-col items-stretch justify-start rounded-t-[20px] bg-neutral-100 p-[18px]"
  >
    {#if $state?.credentials && $state?.credentials.length > 0}
      <Favorites />
      <CredentialList />
      <!-- container that animates and places the button -->
      <div in:fly={{ y: 12, delay: 400, opacity: 0 }} class="absolute bottom-4 right-4">
        <AddButton />
      </div>
    {:else if $state?.user_journey}
      <!-- With active onboarding journey -->
      <div class="flex h-max grow flex-col items-center justify-center text-center">
        <div class="relative">
          <!-- TODO: extract icon component? -->
          <div class="relative z-10">
            <!-- z-index only applies to elements with explicit position, therefore also "relative" -->
            <PaddedIcon icon={RocketLaunch}></PaddedIcon>
          </div>

          <!-- Confetti -->
          <div class="absolute left-1/2 top-1/2 z-0 -translate-x-1/2 -translate-y-1/2">
            <lottie-player
              src="lottiefiles/bubble-burst-confetti-idQiUsReAi.json"
              autoplay
              loop
              speed={0.25}
              mode="normal"
              style="width: 320px"
            />
          </div>
        </div>

        <div class="select-none pt-[15px]">
          <p class="pb-[15px] text-2xl font-semibold text-black">{$LL.GETTING_STARTED_TITLE()}</p>
          <p class="custom text-slate-500 w-[240px]">{$LL.GETTING_STARTED_SUBTITLE()}</p>
        </div>
      </div>

      <BottomDrawer
        titleText={$LL.GETTING_STARTED_DIALOG_0_TITLE()}
        descriptionText={$LL.GETTING_STARTED_DIALOG_0_TEXT()}
      >
        <Button slot="trigger" let:trigger {trigger} label="Let's go" />
        <!-- <button
          slot="trigger"
          let:trigger
          class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
          use:melt={trigger}>Start</button
        > -->
        <div slot="content" class="flex flex-col pt-[20px]">
          <!-- TODO: add multiple steps inline in drawer -->
          <Button label={$LL.CONTINUE()} on:click={() => goto('/goals')} />
          <!-- <button
            class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
            on:click={() => goto('/goals')}>{$LL.CONTINUE()}</button
          > -->
        </div>
      </BottomDrawer>
    {:else}
      <!-- Skipped onboarding journey -->
      <NoCredentials />
      <div in:fly={{ y: 12, delay: 400, opacity: 0 }} class="absolute bottom-4 right-4">
        <AddButton />
      </div>
    {/if}
  </div>
</div>

<style>
  .custom {
    font-size: 13px;
    font-style: normal;
    font-weight: 400;
    line-height: 24px;
  }
</style>
