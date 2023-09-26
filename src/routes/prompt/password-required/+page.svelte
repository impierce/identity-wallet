<script lang="ts">
  import { onMount } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import Button from '$src/lib/components/Button.svelte';
  import UniMeLogoDark from '$src/lib/components/logo/UniMeLogoDark.svelte';
  import UniMeLogoLight from '$src/lib/components/logo/UniMeLogoLight.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
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
      <!-- Forgot password? Reset app -->
      <div class="mt-8">
        <BottomDrawer
          titleText="Reset app"
          descriptionText="Are you sure you want to reset the entire app and remove all data?"
        >
          <button
            slot="trigger"
            let:trigger
            use:melt={trigger}
            class="rounded-xl px-4 py-2 text-[13px]/[24px] font-medium text-slate-400 opacity-50 active:bg-grey dark:active:bg-dark"
            >Forgot password?</button
          >

          <!-- TODO: bug: after resetting (closing the drawer, main UI is not clickable anymore) -->
          <div slot="content" class="w-full pb-[10px] pt-[20px]">
            <button
              class="h-[48px] w-full rounded-xl bg-rose-100 px-4 py-2 text-[14px]/[24px] font-medium text-rose-500"
              on:click={() => dispatch({ type: '[App] Reset' })}>Yes, delete everything</button
            >
          </div>

          <Button variant="secondary" slot="close" let:close trigger={close} label="No, keep my profile" />
        </BottomDrawer>
      </div>
    {/if}
  </div>
</div>

<div class="safe-area-top bg-silver dark:bg-navy" />
<div class="safe-area-bottom bg-silver dark:bg-navy" />

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }

  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the emoji drawer is open */
    position: unset !important;
  }
</style>
