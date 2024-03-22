<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade, fly, slide } from 'svelte/transition';

  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';

  import '@lottiefiles/lottie-player';

  import { melt } from '@melt-ui/svelte';
  import { debug, info } from '@tauri-apps/plugin-log';

  import LL from '$src/i18n/i18n-svelte';
  import WelcomeMessage from '$src/lib/app/WelcomeMessage.svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import ButtonRounded from '$src/lib/components/atoms/ButtonRounded.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import IconMessage from '$src/lib/components/molecules/IconMessage.svelte';
  import Tabs from '$src/lib/components/molecules/navigation/Tabs.svelte';
  import Sort from '$src/lib/components/molecules/sort/Sort.svelte';
  import CredentialList from '$src/lib/credentials/CredentialList.svelte';
  import Favorites from '$src/lib/credentials/Favorites.svelte';
  import UserJourney from '$src/lib/journeys/UserJourney.svelte';
  import { onboarding_state, state } from '$src/stores';

  import Ghost from '~icons/ph/ghost-fill';
  import MagnifyingGlass from '~icons/ph/magnifying-glass';
  import PlusCircle from '~icons/ph/plus-circle';
  import RocketLaunch from '~icons/ph/rocket-launch-fill';

  import { calculateInitials } from './utils';

  let initials: string | undefined;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('routes/(app)/me/+page.svelte: state', $state);
    if ($state?.profile_settings.profile?.name) {
      initials = calculateInitials($state?.profile_settings.profile?.name);
    }
  }

  // security: clear onboarding state after successful creation
  // TODO: move somewhere else
  onboarding_state.set({});
</script>

<div class="flex min-h-full flex-col bg-white dark:bg-dark">
  <div class="sticky top-0 z-10 w-full bg-white px-[20px] py-4 dark:bg-dark">
    <!-- Top Bar -->
    <div class="flex items-center justify-between">
      <button
        class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary"
        on:click={() => goto('/me/settings')}
      >
        {#if $state.profile_settings.profile?.picture}
          <span class="text-[28px]/[28px]">
            {@html $state.profile_settings.profile?.picture}
          </span>
        {:else}
          <span class="text-[20px]/[20px] font-semibold text-white dark:text-dark">
            {initials}
          </span>
        {/if}
      </button>
      <button
        on:click={() => goto('/me/search')}
        class="-mr-3 flex h-11 w-11 items-center justify-center rounded-2xl text-black dark:text-white"
      >
        <MagnifyingGlass class="h-6 w-6" />
      </button>
    </div>
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
      <div class="relative">
        <div>
          <Tabs
            class="mr-[58px]"
            triggers={[$LL.ME.CREDENTIAL_TABS.ALL(), $LL.ME.CREDENTIAL_TABS.DATA(), $LL.ME.CREDENTIAL_TABS.BADGES()]}
          >
            <!-- All -->
            <div slot="0" class="h-full pt-5">
              <Favorites />
              <CredentialList />
            </div>

            <!-- Data -->
            <div slot="1" class="h-full pt-5">
              <Favorites credentialType="data" />
              <CredentialList credentialType="data" />
            </div>

            <!-- Badges -->
            <div slot="2" class="h-full pt-5">
              <Favorites credentialType="badges" />
              <CredentialList credentialType="badges" />
            </div>
          </Tabs>
        </div>

        <div class="absolute right-0 top-0 z-50">
          <Sort />
        </div>
      </div>

      <!-- container that animates and places the button -->
      <div in:fly={{ y: 12, delay: 0, opacity: 1, duration: 200 }} class="absolute bottom-4 right-4">
        <!-- <div in:fade={{ delay: 200, duration: 200 }} class="absolute bottom-4 right-4"> -->
        <!-- TODO: feature disabled: "Add self-signed credential" -->
        <!-- <ButtonRounded label="Add" icon={PlusCircle} /> -->
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
            Shall we get started?
          </p>
          <p class="custom w-[240px] text-slate-500 dark:text-slate-300">
            Start your first steps to add some credentials to your "Me".
          </p>
        </div>
      </div>

      <BottomDrawer
        titleText="Complete new goals"
        descriptionText="Start your mission here! Goals will lead you through important features and possibilities of UniMe app."
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
      <div class="flex grow flex-col items-center justify-center">
        <IconMessage icon={Ghost} title={$LL.ME.EMPTY_CREDENTIALS.TITLE()} />
        <p class="w-[280px] pt-[15px] text-center text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
          {$LL.ME.DEMO.TEXT_1()} <span class="font-semibold text-primary">https://demo.ngdil.com</span>
          {$LL.ME.DEMO.TEXT_2()}
        </p>
      </div>
      <!-- TODO: feature disabled: "Add self-signed credential" -->
      <div in:fly={{ y: 12, delay: 400, opacity: 0 }} class="absolute bottom-4 right-4">
        <!-- <ButtonRounded label="Add" icon={PlusCircle} /> -->
      </div>
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
