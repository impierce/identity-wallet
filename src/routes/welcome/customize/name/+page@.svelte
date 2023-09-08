<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { Input, SearchInput, TopNavigation } from '@impierce/ui-components';

  import Button from '$src/lib/components/Button.svelte';
  import { onboarding_state } from '$src/stores';

  $: {
    console.log($onboarding_state);
  }
</script>

<!-- <TopNavigation title="Appearance" on:back={() => history.back()} /> -->
<div class="flex h-screen flex-col bg-silver dark:bg-navy">
  <TopNavigation on:back={() => history.back()} title="Customization" />
  <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
    <div class="px-2 pb-8 pt-4">
      <p class="pb-4 text-3xl font-semibold text-slate-700">
        Let's go! Choose a <span class="text-primary">profile name</span>
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500">
        Your profile information will never leave your device.
      </p>
    </div>
    <!-- <p class="pb-2 text-[14px]/[22px] font-medium text-slate-600">Profile name</p> -->
    <input
      class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal"
      placeholder="Enter a profile name"
      bind:value={$onboarding_state.name}
    />

    <!-- <SearchInput
    delay={100}
    placeholder="First name"
    on:value={(e) => ($onboarding_state.name = e.detail)}
  /> -->
  </div>
  <div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
    <Button
      label="Continue"
      on:click={() => goto('/welcome/customize/theme')}
      disabled={!!!$onboarding_state.name}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
