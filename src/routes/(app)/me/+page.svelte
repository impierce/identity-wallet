<script lang="ts">
  import { state } from '../../../stores';
  import LL from '$i18n/i18n-svelte';
  import { Avatar, BottomDrawer } from '@impierce/ui-components';
  import { fade, fly, slide } from 'svelte/transition';
  import QrCodeButton from '$lib/QrCodeButton.svelte';
  import { debug, info } from '@tauri-apps/plugin-log';
  import CredentialList from '$lib/CredentialList.svelte';
  import { calculate_initials } from './utils';
  import WelcomeHeader from '$lib/WelcomeHeader.svelte';
  import Favorites from '$lib/Favorites.svelte';
  import Ghost from '~icons/ph/ghost-fill';
  import RocketLaunch from '~icons/ph/rocket-launch-fill';
  import AddButton from '$lib/credentials/AddButton.svelte';
  import { melt } from '@melt-ui/svelte';
  import '@lottiefiles/lottie-player';
  import { goto } from '$app/navigation';

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
  <!-- Background -->
  <!-- <div class="absolute h-[4px] top-0 w-full z-10 bg-gradient-to-r from-blue-500 via-violet-500 to-pink-500" /> -->

  <!-- Banner image (switches when dark mode) -->
  <!-- <picture>
    <source srcset="dark.png" media="(prefers-color-scheme: dark)">
    <img src="light.png" alt="">
  </picture> -->
  <!-- End: Banner image -->

  <!-- TODO: Shrinking header on scroll: https://css-tricks.com/how-to-create-a-shrinking-header-on-scroll-without-javascript/ -->
  <div class="sticky top-0 z-10">
    <div in:fly={{ y: -24, opacity: 1 }}>
      <WelcomeHeader />
    </div>
  </div>

  <div
    in:fly={{ y: 24 }}
    class="flex grow flex-col items-stretch justify-start rounded-t-3xl bg-neutral-100 p-6"
  >
    {#if $state?.credentials && $state?.credentials.length > 0}
      <Favorites />
      <CredentialList />
      <!-- container that animates and places the button -->
      <div in:fly={{ y: 12, delay: 400, opacity: 0.5 }} class="absolute bottom-4 right-4">
        <AddButton />
      </div>
    {:else if $state?.user_journey}
      <!-- With active onboarding journey -->
      <div class="flex h-max grow flex-col items-center justify-center text-center">
        <div class="relative">
          <!-- TODO: extract icon component? -->
          <div class="relative z-10">
            <!-- z-index only applies to elements with explicit position, therefore also "relative" -->
            <div class="rounded-2xl bg-indigo-500 p-4">
              <RocketLaunch class="h-8 w-8 text-white" />
            </div>
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

        <div class="select-none p-6">
          <p class="pb-4 text-lg font-semibold text-slate-500">{$LL.GETTING_STARTED_TITLE()}</p>
          <p class="text-slate-400">{$LL.GETTING_STARTED_SUBTITLE()}</p>
        </div>
      </div>

      <BottomDrawer
        titleText="Complete new goals"
        descriptionText="Start your mission here! Goals will lead you through important features and possibilities of the UniMe app."
      >
        <button
          slot="trigger"
          let:trigger
          class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
          use:melt={trigger}>Start</button
        >
        <div slot="content" class="flex flex-col">
          <!-- TODO: add multiple steps inline in drawer -->
          <button
            class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
            on:click={() => goto('/goals')}>Continue</button
          >
        </div>
      </BottomDrawer>
    {:else}
      <!-- Skipped onboarding journey -->
      <div class="flex h-max flex-col items-center justify-center text-center">
        <div class="rounded-2xl bg-indigo-500 p-4"><Ghost class="h-8 w-8 text-white" /></div>
        <div class="select-none p-6">
          <p class="pb-4 text-lg font-semibold text-slate-500">
            {$LL.EMPTY_CREDENTIALS_LIST_TITLE()}
          </p>
          <p class="text-slate-400">{$LL.EMPTY_CREDENTIALS_LIST_SUBTITLE()}</p>
        </div>
      </div>

      <AddButton />
    {/if}
  </div>

  <!-- <div class="-z-5 fixed top-0 w-full">
    <img
      src="blob-scene-haikei-slate-dark.png"
      alt="background-blob-scene"
      class="absolute hidden w-full opacity-100 dark:block"
    />
    <img
      src="blob-scene-haikei-slate-light.png"
      alt="background-blob-scene"
      class="absolute w-full opacity-60 dark:hidden"
    />
  </div> -->

  <!-- Content sheet -->
  <!-- <div
    class="fixed bottom-0 h-1/3 w-full rounded-t-3xl bg-slate-100 dark:bg-slate-700"
    in:fly={{ y: 24, opacity: 1 }}
  > -->
  <!-- <div
      class="relative bottom-12 -mb-6 flex select-none justify-center"
      in:fly={{ y: 12, opacity: 1 }}
    >
      <Avatar {initials} size="large" />
    </div> -->
  <!-- <div class="flex select-none justify-center text-2xl font-semibold dark:text-neutral-300">
      {$state?.active_profile?.display_name}
    </div> -->

  <!-- <div class="flex flex-col space-y-8 px-8"> -->
  <!-- <h1 class="font-serif text-2xl font-semibold">
        {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
      </h1> -->
  <!-- <button class="flex w-full justify-center rounded-lg bg-slate-200 p-6">
        <Plus class="text-violet-700" strokeWidth="2" />
      </button> -->
  <!-- <CredentialList /> -->
  <!-- </div> -->
  <!-- </div> -->
  <!-- Navigation -->
  <!-- <div class="safe-bottom fixed w-full">
    <BottomNavbar active="home" />
  </div> -->
</div>
