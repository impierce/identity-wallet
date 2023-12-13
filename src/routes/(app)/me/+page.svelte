<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { fade, fly, slide } from 'svelte/transition';

  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';

  import '@lottiefiles/lottie-player';

  import { melt } from '@melt-ui/svelte';
  import { debug, info } from '@tauri-apps/plugin-log';

  import Button from '$lib/components/Button.svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import CredentialList from '$lib/CredentialList.svelte';
  import AddButton from '$lib/credentials/AddButton.svelte';
  import NoCredentials from '$lib/credentials/NoCredentials.svelte';
  import Favorites from '$lib/Favorites.svelte';
  import TopBar from '$lib/home-header/TopBar.svelte';
  import UserJourney from '$lib/home-header/UserJourney.svelte';
  import WelcomeMessage from '$lib/home-header/WelcomeMessage.svelte';
  import QrCodeButton from '$lib/QrCodeButton.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import CredentialTabs from '$src/lib/components/molecules/tabs/CredentialTabs.svelte';
  import { onboarding_state, state } from '$src/stores';

  import RocketLaunch from '~icons/ph/rocket-launch-fill';

  import { calculate_initials } from './utils';

  let initials: string | undefined;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('routes/(app)/me/+page.svelte: state', $state);
    if ($state?.active_profile?.name) {
      initials = calculate_initials($state?.active_profile?.name);
    }
  }

  // security: clear onboarding state after successful creation
  // TODO: move somewhere else
  onboarding_state.set({});
</script>

<div class="flex min-h-full flex-col bg-white dark:bg-dark">
  <div class="sticky top-0 z-10 w-full bg-white px-[20px] py-4 dark:bg-dark">
    <TopBar />
  </div>

  <div class="p-5 pt-0">
    <WelcomeMessage />
    {#if $state?.user_journey}
      <div class="pt-4">
        <UserJourney />
      </div>
    {/if}
  </div>

  <!-- should have min height: full screen - smallest possible welcome header - bottom nav - safe areas (top, bottom) -->
  <div
    in:fly={{ y: 24, duration: 200 }}
    class="flex grow flex-col items-stretch justify-start rounded-t-[20px] bg-silver p-[18px] dark:bg-navy"
  >
    {#if $state?.credentials && $state?.credentials.length > 0}
      <!-- <div class="flex pb-5"> -->
      <!-- <div class="grow"> -->
      <CredentialTabs>
        <!-- All -->
        <div slot="all" class="pt-5">
          <Favorites />
          <CredentialList />
        </div>

        <!-- Data -->
        <div slot="data" class="pt-5">
          <Favorites credentialType="data" />
          <CredentialList credentialType="data" />
        </div>

        <!-- Badges -->
        <div slot="badges" class="pt-5">
          <Favorites credentialType="badges" />
          <CredentialList credentialType="badges" />
        </div>
      </CredentialTabs>
      <!-- </div> -->
      <!-- </div> -->
      <!-- <Favorites /> -->
      <!-- <CredentialList /> -->
      <!-- container that animates and places the button -->
      <div in:fly={{ y: 12, delay: 0, opacity: 1, duration: 200 }} class="absolute bottom-4 right-4">
        <!-- <div in:fade={{ delay: 200, duration: 200 }} class="absolute bottom-4 right-4"> -->
        <!-- TODO: feature disabled: "Add self-signed credential" -->
        <!-- <AddButton /> -->
      </div>
    {:else if $state?.user_journey}
      <!-- With active onboarding journey -->
      <div class="flex h-max grow flex-col items-center justify-center text-center">
        <div class="relative">
          <!-- TODO: extract icon component? -->
          <div class="relative z-10">
            <!-- z-index only applies to elements with explicit position, therefore also "relative" -->
            <PaddedIcon icon={RocketLaunch} />
          </div>

          <!-- Confetti -->
          <div class="absolute left-1/2 top-1/2 z-0 -translate-x-1/2 -translate-y-1/2">
            <lottie-player
              src="/lottiefiles/bubble-burst-confetti-ajgRKUnNJ7.json"
              autoplay
              loop
              speed={0.25}
              mode="normal"
              style="width: 320px"
            />
          </div>
        </div>

        <div class="pt-[15px]">
          <p class="pb-[15px] text-[22px]/[30px] font-semibold tracking-tight text-slate-800 dark:text-grey">
            {$LL.GETTING_STARTED.TITLE()}
          </p>
          <p class="custom w-[240px] text-slate-500 dark:text-slate-300">
            {$LL.GETTING_STARTED.SUBTITLE()}
          </p>
        </div>
      </div>

      <BottomDrawer
        titleText={$LL.GETTING_STARTED.DIALOG_0_TITLE()}
        descriptionText={$LL.GETTING_STARTED.DIALOG_0_TEXT()}
      >
        <!-- TODO: bug: properly $close the drawer with melt-ui (otherwise two clicks necessary) -->
        <Button slot="trigger" let:trigger {trigger} label="Let's go" />
        <!-- <button
          slot="trigger"
          let:trigger
          class="w-full rounded-lg bg-primary px-4 py-2 text-white"
          use:melt={trigger}>Start</button
        > -->
        <div slot="content" class="flex w-full flex-col pt-[20px]">
          <!-- TODO: add multiple steps inline in drawer -->
          <Button label={$LL.CONTINUE()} on:click={() => goto('/goals')} />
          <!-- <button
            class="w-full rounded-lg bg-primary px-4 py-2 text-white"
            on:click={() => goto('/goals')}>{$LL.CONTINUE()}</button
          > -->
        </div>
      </BottomDrawer>
    {:else}
      <!-- Skipped onboarding journey -->
      <NoCredentials />
      <!-- TODO: feature disabled: "Add self-signed credential" -->
      <!-- <div in:fly={{ y: 12, delay: 400, opacity: 0 }} class="absolute bottom-4 right-4">
        <AddButton />
      </div> -->
    {/if}
  </div>
</div>

<!-- Overwrite colors from layout -->
<div class="safe-area-top z-10 bg-white dark:bg-dark" />

<style>
  .custom {
    font-size: 13px;
    font-style: normal;
    font-weight: 400;
    line-height: 24px;
  }
</style>
