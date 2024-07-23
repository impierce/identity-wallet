<script lang="ts">
  import { beforeNavigate, goto, replaceState } from '$app/navigation';
  import { page } from '$app/stores';
  import { fly } from 'svelte/transition';

  import { ActionSheet } from '$lib/components';

  import '@lottiefiles/lottie-player';

  import LL from '$i18n/i18n-svelte';
  import { writable, type Writable } from 'svelte/store';

  import { Button, CredentialList, Favorites, IconMessage, PaddedIcon, Tabs } from '$lib/components';
  import { GhostFillIcon, MagnifyingGlassIcon, RocketLaunchFillIcon } from '$lib/icons';
  import Ngdil from '$lib/static/svg/logo/demos/Ngdil.svelte';
  import Selv from '$lib/static/svg/logo/demos/Selv.svelte';
  import { onboarding_state, state } from '$lib/stores';
  import { calculateInitials } from '$lib/utils';

  import SortingSheet from './SortingSheet.svelte';
  import UserJourney from './UserJourney.svelte';
  import WelcomeMessage from './WelcomeMessage.svelte';

  let initials: string | undefined;

  let triggers = [$LL.ME.CREDENTIAL_TABS.ALL(), $LL.ME.CREDENTIAL_TABS.DATA(), $LL.ME.CREDENTIAL_TABS.BADGES()];
  let activeTab: Writable<string> = writable($page.state.tab || triggers[0]);

  beforeNavigate(async () => {
    replaceState('', { tab: $activeTab });
  });

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
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
        <MagnifyingGlassIcon class="h-6 w-6" />
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
          <Tabs class="mr-[50px]" value={activeTab} {triggers}>
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
          <SortingSheet />
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
            <PaddedIcon icon={RocketLaunchFillIcon} />
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
          <p class="w-[240px] text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
            Start your first steps to add some credentials to your "Me".
          </p>
        </div>
      </div>

      <ActionSheet
        titleText="Complete new goals"
        descriptionText="Start your mission here! Goals will lead you through important features and possibilities of UniMe app."
      >
        <!-- TODO: bug: properly $close the drawer with melt-ui (otherwise two clicks necessary) -->
        <Button slot="trigger" let:trigger {trigger} label="Let's go" />
        <div slot="content" class="flex w-full flex-col pt-[20px]">
          <!-- TODO: add multiple steps inline in drawer -->
          <Button label={$LL.CONTINUE()} on:click={() => goto('/goals')} />
        </div>
      </ActionSheet>
    {:else}
      <!-- Skipped onboarding journey -->
      <div class="flex grow flex-col items-center justify-center">
        <IconMessage icon={GhostFillIcon} title={$LL.ME.EMPTY_CREDENTIALS.TITLE()} />
        <div class="w-[280px] pt-[15px] text-center text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
          {$LL.ME.DEMO()}
          <div class="flex flex-col gap-3 pt-[15px]">
            <!-- Selv -->
            <div class="flex h-14 items-center justify-between rounded-xl bg-white p-4 dark:bg-dark">
              <Selv class="h-6 w-14 text-slate-500 dark:text-slate-300" />
              <span class="text-[13px]/[24px] font-semibold text-primary">https://selv.iota.org</span>
            </div>
            <!-- NGDIL -->
            <div class="flex h-14 items-center justify-between rounded-xl bg-white p-4 dark:bg-dark">
              <Ngdil class="h-6 w-14 text-slate-500 dark:text-slate-300" />
              <span class="text-[13px]/[24px] font-semibold text-primary">https://demo.ngdil.com</span>
            </div>
          </div>
        </div>
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
