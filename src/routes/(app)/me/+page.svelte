<script lang="ts">
  import { state } from '../../../stores';
  import LL from '../../../i18n/i18n-svelte';
  import { Avatar } from '@impierce/ui-components';
  import { fade, fly, slide } from 'svelte/transition';
  import QrCodeButton from '$lib/QrCodeButton.svelte';
  import { debug, info } from '@tauri-apps/plugin-log';
  import CredentialList from '$lib/CredentialList.svelte';
  import { calculate_initials } from './utils';
  import WelcomeHeader from '$lib/WelcomeHeader.svelte';
  import Favorites from '$lib/Favorites.svelte';

  let initials: string | undefined;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('routes/(app)/me/+page.svelte: state', $state);
    if ($state?.active_profile?.display_name) {
      initials = calculate_initials($state?.active_profile?.display_name);
    }
  }
</script>

<div class="">
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
    <div in:fly={{ y: -24, opacity: 1 }} class="">
      <WelcomeHeader />
    </div>
  </div>

  <div in:fly={{ y: 24 }} class="px-8 pt-8">
    <Favorites />
    <!-- Divider -->
    <p class="my-6 h-[2px] bg-slate-100" />
    <CredentialList />
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
