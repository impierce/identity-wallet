<script lang="ts">
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import { Avatar, BottomNavigation, CredentialListEntry } from '@impierce/ui-components';
  import { Plus, XMark } from 'svelte-heros-v2';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fade, fly, slide } from 'svelte/transition';

  let initials: string | undefined;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('state', $state);
    let names = $state?.active_profile?.display_name.split(' ');
    if (names?.length === 1) {
      initials = names?.at(0)?.slice(0, 2).toUpperCase();
    } else {
      let first = names?.at(0)?.charAt(0);
      let last = names?.at(1)?.charAt(0);
      // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
      initials = first + '' + last;
    }
    console.log('initials', initials);
  }
</script>

<div class="min-h-screen">
  <!-- Background -->
  <!-- "absolute -z-10 w-full opacity-60" -->
  <img
    src="blob-scene-haikei-slate.png"
    alt="background-blob-scene"
    class="absolute w-full opacity-50"
  />
  <!-- <div class="absolute -z-10 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500"></div> -->
  <!-- Content overlay -->
  <div
    class="absolute bottom-0 h-5/6 w-full rounded-t-3xl bg-slate-100"
    in:fly={{ y: 24, opacity: 1 }}
  >
    <div class="relative bottom-12 -mb-6 flex justify-center">
      <Avatar {initials} size="large" />
    </div>
    <div class="flex justify-center text-2xl font-semibold">
      {$state?.active_profile?.display_name}
    </div>

    <div class="space-y-8 p-8">
      <!-- <h1 class="font-serif text-2xl font-semibold">
        {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
      </h1> -->
      <div class="rounded-lg bg-slate-200 p-6">
        <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p>
        <p class="text-slate-400">{$LL.CREATE_IDENTITY_SUCCESS_BODY()}</p>
      </div>
      <button class="flex w-full justify-center rounded-lg bg-slate-200 p-6">
        <Plus class="text-violet-700" strokeWidth="2" />
      </button>
    </div>
  </div>
  <!-- Navigation -->
  <div class="sticky top-[100vh]">
    <BottomNavigation
      active="profile"
      on:settings={() => goto('/settings')}
      on:history={() => goto('/history')}
    />
  </div>
</div>
