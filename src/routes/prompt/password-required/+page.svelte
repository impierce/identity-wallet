<script lang="ts">
  import { onMount } from 'svelte';

  import Button from '$src/lib/components/Button.svelte';
  import UniMeLogoDark from '$src/lib/components/logo/UniMeLogoDark.svelte';
  import UniMeLogoLight from '$src/lib/components/logo/UniMeLogoLight.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { developer_mode } from '$src/stores';

  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';

  let showPassword = false;

  let password: string;

  // TODO: make reactive
  const isDarkModeEnabled = document.documentElement.classList.contains('dark');

  onMount(() => {
    if ($developer_mode) {
      console.log('Developer mode - Injecting password automatically ...');
      setTimeout(() => {
        dispatch({ type: '[Storage] Unlock', payload: { password: 'sup3rSecr3t' } });
      }, 500);
    }
  });
</script>

<div class="content-height flex items-center justify-center bg-silver dark:bg-navy">
  <!-- Placeholder -->
  <!-- <div class="aspect-square w-1/4 rounded-3xl border border-slate-200 bg-slate-100" /> -->
  <div class="flex flex-col items-center justify-center">
    {#if isDarkModeEnabled}
      <UniMeLogoDark />
    {:else}
      <UniMeLogoLight />
    {/if}
    {#if !$developer_mode}
      <div class="relative mb-4 mt-8 w-[240px]">
        <input
          type={showPassword ? 'text' : 'password'}
          class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
          placeholder="Enter your password"
          on:input={(e) => (password = e.target.value)}
        />
        <div class="absolute right-3 top-0 flex h-full items-center">
          <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
            {#if showPassword}
              <Eye class="text-slate-700 dark:text-grey" />
            {:else}
              <EyeClosed class="text-slate-700 dark:text-grey" />
            {/if}
          </button>
        </div>
      </div>
      <Button
        label="Unlock wallet"
        on:click={() => dispatch({ type: '[Storage] Unlock', payload: { password } })}
        disabled={!password}
      />
    {/if}
  </div>
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
