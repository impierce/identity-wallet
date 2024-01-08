<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade, fly } from 'svelte/transition';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import MeLarge from '$src/lib/static/svg/logo/MeLarge.svelte';
  import UniMeTextDark from '$src/lib/static/svg/logo/UniMeTextDark.svelte';
  import UniMeTextLight from '$src/lib/static/svg/logo/UniMeTextLight.svelte';
  import { onboarding_state, state } from '$src/stores';

  // TODO: make reactive
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');
</script>

<div
  class="content-height relative flex flex-col bg-silver dark:bg-navy"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
  <div class="absolute top-12 w-full">
    <!-- <LanguageSelect
      selected={$state?.locale}
      on:value={(e) => dispatch({ type: '[Settings] Set locale', payload: { locale: e.detail } })}
    /> -->

    <!-- <div class="grid grid-flow-col space-x-4">
      <button
        class={active_language === 'en' ? active_language_class : ''}
        on:click={() => setLanguage('en')}><GB class="h-[27px] w-[36px] rounded-lg" /></button
      >
      <button
        class={active_language === 'nl' ? active_language_class : ''}
        on:click={() => setLanguage('nl')}><NL class="h-[27px] w-[36px] rounded-lg" /></button
      >
      <button
        class={active_language === 'de' ? active_language_class : ''}
        on:click={() => setLanguage('de')}><DE class="h-[27px] w-[36px] rounded-lg" /></button
      >
    </div> -->
  </div>

  <div class="grow">
    <!-- <div class="flex grow flex-col justify-center" in:fade out:fly={{ x: -300, duration: 300 }}> -->
    <div class="mt-[calc(100vh/6)] px-4 py-6">
      <div class="pb-[50px]">
        <p class=" pb-[10px] text-[36px]/[44px] font-bold text-blue dark:text-silver">Welcome to</p>
        {#if isDarkModeEnabled}
          <UniMeTextDark />
        {:else}
          <UniMeTextLight />
        {/if}
      </div>

      <p class="text-[14px]/[22px] font-medium text-ex-grey-2 dark:text-grey">
        UniMe connects your digital world, safely and securely.
        <br /><br />
        <!-- Create a brand new identity profile or recover an existing one to get started. -->
        Create a brand new identity profile to get started.
      </p>
    </div>
  </div>

  <div class="absolute bottom-16 left-0">
    <MeLarge class="h-auto w-screen" />
  </div>

  <!-- Actions -->
  <div class="z-10 space-y-[10px] rounded-t-3xl bg-white p-6 dark:bg-dark">
    <!-- <div
    class="space-y-[10px] rounded-t-3xl bg-white p-6"
    in:fly={{ y: 154 }}
    out:fly={{ y: 154, duration: 300, opacity: 1 }}
  > -->
    <Button
      label="Create new profile"
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
