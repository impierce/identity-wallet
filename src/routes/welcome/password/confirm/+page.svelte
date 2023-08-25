<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';

  import CheckCircle from '~icons/ph/check-circle-fill';
  import Circle from '~icons/ph/circle';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';

  import { checkPasswordPolicy, passwordPolicy } from '../utils';

  let passwordsEqual = true;
  let showPassword = false;
</script>

<TopNavigation on:back={() => history.back()} title="Confirm Password">
  <BottomDrawer titleText={$LL.SETUP.SKIP_TITLE()} descriptionText={$LL.SETUP.SKIP_TEXT()}>
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary">{$LL.SKIP()}</button
    >
    <!-- <button
        slot="trigger"
        let:trigger
        use:melt={trigger}
        class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary"
      >
        <div class="h-6 w-6 bg-slate-200" />
      </button> -->
    <button
      slot="content"
      class="w-full rounded-lg bg-red-100 px-4 py-2 text-red-600"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Yes</button
    >
    <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      >No, let's continue</button
    >
  </BottomDrawer>
</TopNavigation>

<!-- Content -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="pb-8 pt-4">
    <p class="pb-8 text-3xl font-semibold text-slate-800">
      Please confirm your new <span class="text-primary">password</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">
      You need to create a strong password to secure your backup.
    </p>
    <!-- <div class="mt-[70px] flex w-full items-center justify-center" /> -->
  </div>
  <div class="relative">
    <input
      type={showPassword ? 'text' : 'password'}
      class="h-12 w-full rounded-xl border border-[#C5C6CC] px-4 py-3 text-[13px]/[24px] text-[#8F9098]"
      placeholder="Retype your password"
      on:input={(e) => {
        // TODO: check if passwords match
      }}
    />
    <div class="absolute right-3 top-0 flex h-full items-center">
      <button class="rounded-full p-2" on:click={() => (showPassword = !showPassword)}>
        {#if showPassword}
          <Eye />
        {:else}
          <EyeClosed />
        {/if}
      </button>
    </div>
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label="Continue"
    on:click={() => goto('/welcome/password/completed')}
    disabled={!passwordsEqual}
  />
</div>
