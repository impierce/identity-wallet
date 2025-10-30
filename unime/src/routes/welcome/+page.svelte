<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button } from '$lib/components';
  import MeLarge from '$lib/static/svg/logo/MeLarge.svelte';
  import UniMeText from '$lib/static/svg/logo/UniMeText.svelte';
  import { onboarding_state } from '$lib/stores';

  import LanguageSelect from './LanguageSelect.svelte';
</script>

<div class="relative flex h-full flex-col" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="grow">
    <div class="mt-[calc(100vh/8)] px-4 py-6">
      <div class="pb-9">
        <p class=" text-blue dark:text-silver pb-[10px] text-[36px]/[44px] font-bold">
          {$LL.ONBOARDING.WELCOME.GREETING()}
        </p>
        <UniMeText class="text-blue dark:text-silver" />
      </div>

      <p class="text-ex-grey-2 dark:text-grey text-[14px]/[22px] font-medium">
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
  <div class="dark:bg-dark z-10 rounded-t-3xl bg-white p-6">
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
