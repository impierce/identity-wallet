<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button } from '$lib/components';
  import MeLarge from '$lib/static/svg/logo/MeLarge.svelte';
  import UniMeTextDark from '$lib/static/svg/logo/UniMeTextDark.svelte';
  import UniMeTextLight from '$lib/static/svg/logo/UniMeTextLight.svelte';
  import { onboarding_state } from '$lib/stores';

  import LanguageSelect from './LanguageSelect.svelte';

  // TODO: make reactive
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');
</script>

<div class="relative flex h-full flex-col" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="grow">
    <div class="mt-[calc(100vh/8)] px-4 py-6">
      <div class="pb-9">
        <p class=" pb-[10px] text-[36px]/[44px] font-bold text-blue dark:text-silver">
          {$LL.ONBOARDING.WELCOME.GREETING()}
        </p>
        {#if isDarkModeEnabled}
          <UniMeTextDark />
        {:else}
          <UniMeTextLight />
        {/if}
      </div>

      <p class="text-[14px]/[22px] font-medium text-ex-grey-2 dark:text-grey">
        {$LL.ONBOARDING.WELCOME.WHAT_IS_UNIME_1()}
        <br /><br />
        {$LL.ONBOARDING.WELCOME.WHAT_IS_UNIME_2()}
      </p>
    </div>

    <div class="flex justify-center">
      <LanguageSelect />
    </div>
  </div>

  <div class="absolute bottom-16 left-0">
    <MeLarge class="h-auto w-screen" />
  </div>

  <!-- Actions -->
  <div class="z-10 rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.ONBOARDING.WELCOME.CREATE_NEW_PROFILE()}
      on:click={() => {
        onboarding_state.set({});
        goto('/welcome/pledge');
      }}
    />
    <!-- TODO: feature disabled: "recover existing profile" -->
    <!-- <Button label="Recover existing profile" variant="secondary" /> -->
  </div>
</div>
