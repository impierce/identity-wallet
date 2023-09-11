<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';
  import { createCheckbox, createPopover, melt } from '@melt-ui/svelte';

  import Button from '$lib/components/Button.svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import { dispatch } from '$lib/dispatcher';
  import CredentialListEntry from '$src/lib/components/CredentialListEntry.svelte';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import Question from '~icons/ph/question';
  import RocketLaunch from '~icons/ph/rocket-launch';
  import WarningCircle from '~icons/ph/warning-circle-fill';
  import X from '~icons/ph/x-bold';

  const {
    elements: { trigger, content, arrow, close },
    states: { open }
  } = createPopover();

  // let selected_credentials = $state.credentials?.filter(
  //   (c) => $state.current_user_prompt.options.indexOf(c.id) > -1
  // );

  // console.log(selected_credentials);

  console.log($state.current_user_prompt);
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavigation title={'Connection Request'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    <PaddedIcon icon={PlugsConnected} />
    <div class="text-center">
      <p class="text-2xl font-semibold">BestDex</p>
      <p class="pt-[10px] text-sm font-medium text-slate-500">www.bestdex.com</p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-2 rounded-2xl bg-white p-3">
      <!-- Warning -->
      <div class="flex w-full items-center rounded-lg bg-slate-50 px-4 py-4">
        <span class="mr-4 h-6 w-6">
          <WarningCircle class="h-6 w-6 text-amber-500" />
        </span>
        <div class="flex flex-col">
          <p class="text-[13px]/[24px] font-medium text-slate-900">New connection</p>
          <p class="text-[12px]/[20px] font-medium text-slate-500">
            Only accept new connections that you recognize and trust
          </p>
        </div>
      </div>

      <!-- <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <p class="text-sm text-slate-800">URL</p>
        <p class="text-sm text-slate-500">bestdex.com</p>
      </div> -->
      <div class="flex justify-between rounded-lg border border-slate-200 bg-white px-4 py-4">
        <p class="text-sm text-slate-800">Connected previously</p>
        <X class="text-rose-500" />
      </div>
      <div class="flex justify-between rounded-lg border border-slate-200 bg-white px-4 py-4">
        <div class="flex items-center">
          <p class="text-sm text-slate-800">Verified</p>
          <button class="-m-2 ml-1 rounded-full p-1" use:melt={$trigger}>
            <Question class="h-6 w-6 text-primary" />
          </button>
          {#if $open}
            <div
              use:melt={$content}
              transition:fade={{ duration: 200 }}
              class="z-10 w-1/2 rounded-2xl bg-dark p-[20px] text-white shadow-md"
            >
              <div use:melt={$arrow} />
              <div class="break-keep text-sm">
                Your UniMe app automatically tries to verify the identity of <span
                  class="underline underline-offset-2">BestDex</span
                >
                to provide you with a secure login.
              </div>
            </div>
          {/if}
        </div>
        <Check class="text-emerald-500" />
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 pb-0">
    <Button label="Accept connection" on:click={() => goto('/prompt/select-credentials/share')} />
    <Button
      label="Reject"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}
    />
  </div>

  <div class="safe-area-top" />
  <div class="safe-area-bottom" />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
