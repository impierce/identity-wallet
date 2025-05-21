<script lang="ts">
  import { beforeNavigate, goto, replaceState } from '$app/navigation';
  import { page } from '$app/stores';
  import { fly } from 'svelte/transition';

  import { ActionSheet } from '$lib/components';

  import '@lottiefiles/lottie-player';

  import LL from '$i18n/i18n-svelte';
  import { writable, type Writable } from 'svelte/store';

  import { Button, CredentialList, Favorites, IconMessage, PaddedIcon, Tabs } from '$lib/components';
  import { GhostFillIcon, MagnifyingGlassIcon, PlusCircleIcon, RocketLaunchFillIcon } from '$lib/icons';
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

<!-- Isolate stacking context to avoid z-index conflicts. -->
<div class="relative isolate flex flex-col bg-white dark:bg-dark">
  <div class="sticky top-0 z-10 w-full bg-white px-[20px] py-4 dark:bg-dark">
    <!-- Top Bar -->
    <div class="flex items-center justify-between">
      <button
        class="flex h-11 w-11 items-center justify-center rounded-2xl bg-primary"
        onclick={() => goto('/me/settings')}
      >
        {#if $state.profile_settings.profile?.picture}
          <span class="text-[28px]/[28px]">
            <!-- The profile picture is an emoticon selected from a list of emoticons we provide. -->
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html $state.profile_settings.profile?.picture}
          </span>
        {:else}
          <span class="text-[20px]/[20px] font-semibold text-white dark:text-dark">
            {initials}
          </span>
        {/if}
      </button>
      <button
        onclick={() => goto('/me/search')}
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
    in:fly={{ y: 24, duration: 200, opacity: 1 }}
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

        <div class="absolute right-0 top-0">
          <SortingSheet />
        </div>
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
            ></lottie-player>
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
          {$LL.ME.EMPTY_CREDENTIALS.SUBTITLE()}
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- "Add" button -->
<!-- <div in:fly={{ y: 12, delay: 0, opacity: 1, duration: 200 }} class="absolute bottom-5 right-4"> -->
<div in:fly={{ y: 12, opacity: 1, duration: 200 }} class="absolute bottom-[calc(64px_+_16px)] right-4">
  <button
    class="flex w-fit justify-center rounded-full bg-primary px-4 py-3 text-white dark:text-dark"
    onclick={() => goto('/me/add')}
  >
    <PlusCircleIcon class="mr-2 size-6" />
    <div class="text-[13px]/[24px] font-medium">{$LL.ADD_CREDENTIALS.BUTTON()}</div>
  </button>
</div>
