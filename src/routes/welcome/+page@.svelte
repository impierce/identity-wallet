<script>
  import { onboarding_state, state } from '$src/stores';
  import { dispatch } from '$src/lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { LanguageSelect } from '@impierce/ui-components';
  import Button from '$src/lib/components/Button.svelte';
  import { goto } from '$app/navigation';
  import { fade, fly } from 'svelte/transition';
</script>

<div
  class="content-height relative flex flex-col bg-bg-secondary dark:bg-bg-dark-secondary"
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

  <div class="flex grow flex-col justify-center">
    <!-- <div class="flex grow flex-col justify-center" in:fade out:fly={{ x: -300, duration: 300 }}> -->
    <div class="px-4">
      <p class="pb-[30px] text-[36px]/[44px] font-bold text-black dark:text-white">
        Welcome to <br /><span class="text-primary">UniMe</span>
      </p>
      <p class="text-[15px]/[24px] font-medium text-slate-500 dark:text-white">
        UniMe connects your digital world, safely and securely.
        <br /><br />
        Create a brand new identity profile or recover an existing one to get started.
      </p>
    </div>
  </div>

  <!-- Actions -->
  <div class="space-y-[10px] rounded-t-3xl bg-white p-6">
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
    <Button label="Recover existing profile" disabled />
  </div>
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
