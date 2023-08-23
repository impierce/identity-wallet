<script lang="ts">
  import { dispatch } from '$src/lib/dispatcher';
  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';
  import LL from '$src/i18n/i18n-svelte';
  import { fade } from 'svelte/transition';
  import Button from '$src/lib/components/Button.svelte';
  import Eye from '~icons/ph/eye';
  import EyeClosed from '~icons/ph/eye-closed';
  import { goto } from '$app/navigation';
  import { passwordPolicy, checkPasswordPolicy } from '../utils';
  import Circle from '~icons/ph/circle';
  import CheckCircle from '~icons/ph/check-circle-fill';
  import Shield from '~icons/ph/shield-fill';
  import '@lottiefiles/lottie-player';

  let passwordsEqual = true;
  let showPassword = false;
</script>

<TopNavigation on:back={() => history.back()} title="Confirm Password">
  <BottomDrawer titleText={$LL.SETUP.SKIP_TITLE()} descriptionText={$LL.SETUP.SKIP_TEXT()}>
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-indigo-500"
      >{$LL.SKIP()}</button
    >
    <!-- <button
          slot="trigger"
          let:trigger
          use:melt={trigger}
          class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-indigo-500"
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
      Your UniMe profile is now <span class="text-indigo-500">protected</span>
    </p>
    <!-- <p class="text-[15px]/[24px] font-medium text-slate-500">
        You need to create a strong password to secure your backup.
      </p> -->
  </div>
  <div class="flex flex-col items-center justify-center space-y-6 rounded-3xl bg-white p-5">
    <p class="text-2xl font-semibold text-indigo-500">Safe & Secure.</p>
    <div class="relative">
      <div class="relative z-10">
        <div class="text-[100px]/[100px]"><Shield class="text-indigo-500" /></div>
        <span class="absolute left-[calc(50%_-_22px)] top-[calc(50%_-_22px)] text-[44px]/[44px]">
          {@html '&#129312;'}
        </span>
      </div>
      <div class="absolute left-1/2 top-1/2 z-0 -translate-x-1/2 -translate-y-1/2">
        <lottie-player
          src="/lottiefiles/bubble-burst-confetti-idQiUsReAi.json"
          autoplay
          loop
          speed={0.25}
          mode="normal"
          style="width: 320px"
        />
      </div>
    </div>
    <p class="text-2xl font-semibold text-indigo-500">Nice Job.</p>
    <!-- Hint: backup -->
    <!-- <div class="bg-slate-100 p-4 rounded-2xl w-full">
      <p class="text-sm text-slate-800">Let's create a quick backup.</p>
    </div> -->
  </div>
</div>

<div class="rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <Button
    label="Done"
    on:click={() =>
      dispatch({
        type: '[DID] Create new',
        payload: { display_name: 'Tony', password: 'sup3rSecr3t' }
      })}
  />
</div>
