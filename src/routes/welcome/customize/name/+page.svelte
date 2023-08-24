<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$src/lib/components/Button.svelte';
  import { onboarding_state } from '$src/stores';
  import { Input, SearchInput, TopNavigation } from '@impierce/ui-components';
  import { fade } from 'svelte/transition';

  $: {
    console.log($onboarding_state);
  }
</script>

<!-- <TopNavigation title="Appearance" on:back={() => history.back()} /> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      Let's go! Choose a <span class="text-indigo-500">profile name</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">
      Your profile information will never leave your device.
    </p>
  </div>
  <div class="m-8">
    <SearchInput
      delay={100}
      placeholder="Enter a profile name"
      on:value={(e) => ($onboarding_state.profile_name = e.detail)}
    />
  </div>
</div>
<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label="Continue"
    on:click={() => goto('/welcome/customize/theme')}
    disabled={!!!$onboarding_state.profile_name}
  />
</div>
