<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';
  import { fade } from 'svelte/transition';

  import { Button, TopNavBar } from '$lib/components';
  import { onboarding_state } from '$lib/stores';

  const defaultName = 'Me';
  let input: HTMLInputElement;

  $onboarding_state.name = $onboarding_state.name || defaultName;

  onMount(() => {
    if ($onboarding_state.name === defaultName) {
      input.focus();
    }
  });
</script>

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
  <input
    class="w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] font-normal text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300 dark:caret-slate-300"
    placeholder={$LL.ONBOARDING.CUSTOMIZE.NAME.INPUT_PLACEHOLDER()}
    bind:value={$onboarding_state.name}
    bind:this={input}
  />
</div>

<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/password')} disabled={!$onboarding_state.name} />
</div>
