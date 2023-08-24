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

  let selected_credentials = $state.credentials?.filter(
    (c) => $state.current_user_prompt.options.indexOf(c.at(0)) > -1
  );

  console.log(selected_credentials);
</script>

<div class="content-height flex flex-col items-stretch bg-neutral-100">
  <TopNavigation title={'Connection Request'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-6">
    <PaddedIcon icon={PlugsConnected} />
    <p class="text-2xl font-semibold">BestDex</p>

    <!-- Warning -->
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4">
      <WarningCircle class="mr-2 h-6 w-6 text-amber-500" />
      <p class="text-sm">Only accept connections that you trust.</p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white">
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <p class="text-sm text-slate-800">URL</p>
        <p class="text-sm text-slate-500">bestdex.com</p>
      </div>
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <p class="text-sm text-slate-800">Connected previously</p>
        <X class="text-red-600" />
      </div>
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <div class="flex items-center">
          <p class="text-sm text-slate-800">Verified</p>
          <button class="-m-2 ml-1 rounded-full p-1" use:melt={$trigger}>
            <Question class="h-6 w-6 text-indigo-500" />
          </button>
          {#if $open}
            <div
              use:melt={$content}
              transition:fade={{ duration: 200 }}
              class="z-10 w-1/2 rounded-2xl bg-[#2F3036] p-[20px] text-[#D4D6DD] shadow-md"
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
        <Check class="text-green-600" />
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
