<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import LL from '$src/i18n/i18n-svelte';
  import ThemeSelect from '$src/lib/app/settings/ThemeSelect.svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import { determineTheme } from '$src/routes/utils';
  import { onboarding_state } from '$src/stores';

  let value: 'system' | 'light' | 'dark' = 'system';

  $: {
    $onboarding_state.theme = value;
    determineTheme(window.matchMedia('(prefers-color-scheme: dark)').matches, value);
  }
</script>

<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.ONBOARDING.CUSTOMIZE.THEME.TITLE_1()}
      <span class="text-primary">{$LL.ONBOARDING.CUSTOMIZE.THEME.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ONBOARDING.CUSTOMIZE.THEME.SUBTITLE()}
    </p>
  </div>
  <ThemeSelect on:change={(e) => (value = e.detail.value)} />
</div>
<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label={$LL.CONTINUE()} on:click={() => goto('/welcome/customize/avatar')} />
</div>
