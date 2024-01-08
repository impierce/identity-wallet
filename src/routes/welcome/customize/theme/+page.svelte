<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import Button from '$src/lib/components/atoms/Button.svelte';
  import ThemeSelect from '$src/lib/customize/ThemeSelect.svelte';
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
      Choose your app <span class="text-primary">appearance</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Are you more of a night owl?</p>
  </div>
  <ThemeSelect on:change={(e) => (value = e.detail.value)} />
</div>
<div class="rounded-t-3xl bg-white p-6 dark:bg-dark" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button label="Continue" on:click={() => goto('/welcome/customize/avatar')} />
</div>
