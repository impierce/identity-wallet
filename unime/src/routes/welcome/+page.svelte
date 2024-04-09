<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import LL from '$src/i18n/i18n-svelte';
  import LanguageSelect from '$src/lib/app/LanguageSelect.svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import MeLarge from '$src/lib/static/svg/logo/MeLarge.svelte';
  import UniMeTextDark from '$src/lib/static/svg/logo/UniMeTextDark.svelte';
  import UniMeTextLight from '$src/lib/static/svg/logo/UniMeTextLight.svelte';
  import { onboarding_state } from '$src/stores';

  // TODO: make reactive
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');
</script>

<div
  class="content-height relative flex flex-col bg-silver dark:bg-navy"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
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
  <div class="z-10 space-y-[10px] rounded-t-3xl bg-white p-6 dark:bg-dark">
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

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
