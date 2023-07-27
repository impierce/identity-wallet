<script lang="ts">
  import { state } from '../../../stores';
  import LL from '../../../i18n/i18n-svelte';
  import {
    Avatar,
    BottomNavigation,
    Button,
    CredentialListEntry,
    Input,
    Label
  } from '@impierce/ui-components';
  import {
    AlertDialog,
    AlertDialogTrigger,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogCancel,
    AlertDialogAction,
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger
  } from '@impierce/ui-components';
  import {
    Plus,
    XMark,
    AtSymbol,
    Phone,
    Home,
    Cake,
    User,
    AcademicCap,
    QuestionMarkCircle
  } from 'svelte-heros-v2';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fade, fly, slide } from 'svelte/transition';
  import QrCodeButton from '$lib/QrCodeButton.svelte';
  import CredentialDetails from '$lib/CredentialDetails.svelte';
  import { debug, info } from '@tauri-apps/plugin-log';
  import { root } from 'postcss';

  let initials: string | undefined;

  // let credentials: any[] = [];

  const calculate_initials = (display_name: string) => {
    let names = display_name.split(' ');
    if (names?.length === 1) {
      initials = names?.at(0)?.slice(0, 2).toUpperCase();
    } else {
      let first = names?.at(0)?.charAt(0) ?? '?';
      let last = names?.at(1)?.charAt(0) ?? '?';
      // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
      initials = first + '' + last;
    }
    info(`calculate_initials: "${initials}"`);
  };

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('routes/profile/+page.svelte: state', $state);
    if ($state?.active_profile?.display_name) {
      calculate_initials($state?.active_profile?.display_name);
    }
    // credentials = $state?.credentials ?? [];
  }
</script>

<div class="flex flex-col">
  <!-- Background -->
  <!-- <div class="absolute h-[4px] top-0 w-full z-10 bg-gradient-to-r from-blue-500 via-violet-500 to-pink-500" /> -->

  <!-- Banner image (switches when dark mode) -->
  <!-- <picture>
    <source srcset="dark.png" media="(prefers-color-scheme: dark)">
    <img src="light.png" alt="">
  </picture> -->
  <!-- End: Banner image -->

  <div class="-z-5 fixed top-0 w-full">
    <!-- TODO: refactor bg images: use native <picture> element -->
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
  </div>
  <!-- Content sheet -->
  <div
    class="fixed bottom-0 h-5/6 w-full rounded-t-3xl bg-slate-100 dark:bg-slate-700"
    in:fly={{ y: 24, opacity: 1 }}
  >
    <div
      class="relative bottom-12 -mb-6 flex select-none justify-center"
      in:fly={{ y: 12, opacity: 1 }}
    >
      <Avatar {initials} size="large" />
    </div>
    <div class="flex select-none justify-center text-2xl font-semibold dark:text-neutral-300">
      {$state?.active_profile?.display_name}
    </div>

    <div class="flex flex-col space-y-8 px-8">
      <!-- <h1 class="font-serif text-2xl font-semibold">
        {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
      </h1> -->
      <!-- <button class="flex w-full justify-center rounded-lg bg-slate-200 p-6">
        <Plus class="text-violet-700" strokeWidth="2" />
      </button> -->

      <div class="fixed bottom-28 right-12 z-10">
        <QrCodeButton />
      </div>

      
    </div>
  </div>
  <!-- Navigation -->
  <!-- <div class="safe-bottom fixed w-full">
    <BottomNavbar active="home" />
  </div> -->

  <!-- fill top safe zone with matching color of "slate-blob-scene"  -->
  <!-- <div class="bg-slate-300 h-[env(safe-area-inset-top)] fixed top-0 w-full z-10"></div> -->

  <!-- fill bottom safe zone with bg-color -->
  <!-- <div
    class="fixed bottom-0 z-10 h-[env(safe-area-inset-bottom)] w-full bg-white dark:bg-slate-800"
  /> -->
</div>

<!-- <style>
  .safe-bottom {
    bottom: env(safe-area-inset-bottom);
  }
</style> -->
