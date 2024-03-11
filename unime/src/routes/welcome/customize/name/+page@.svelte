<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { onboarding_state } from '$src/stores';

  $onboarding_state.name = $onboarding_state.name || 'Me';
</script>

<!-- <TopNavBar title="Appearance" on:back={() => history.back()} /> -->
<div class="content-height relative flex flex-col bg-silver dark:bg-navy">
  <!-- TODO: the only reason why we're breaking out of the layout is because we do not want to inherit the "Skip" button -->
  <TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.CUSTOMIZE.NAVBAR_TITLE()} />
  <div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
    <div class="px-2 pb-8 pt-4">
      <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
        {$LL.ONBOARDING.CUSTOMIZE.NAME.TITLE_1()}
        <span class="text-primary">{$LL.ONBOARDING.CUSTOMIZE.NAME.TITLE_2()}</span>
      </p>
      <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
        {$LL.ONBOARDING.CUSTOMIZE.NAME.SUBTITLE()}
      </p>
    </div>
    <!-- <p class="pb-2 text-[14px]/[22px] font-medium text-slate-600">Profile name</p> -->
    <input
      class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
      placeholder={$LL.ONBOARDING.CUSTOMIZE.NAME.INPUT_PLACEHOLDER()}
      bind:value={$onboarding_state.name}
    />
  </div>
  <div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
    <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password')} disabled={!!!$onboarding_state.name} />
  </div>
</div>

<div class="safe-area-top bg-silver dark:bg-navy" />

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
