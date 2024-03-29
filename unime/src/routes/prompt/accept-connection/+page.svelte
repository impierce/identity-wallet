<script lang="ts">
  import { onDestroy } from 'svelte';

  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { createCheckbox, createPopover, melt } from '@melt-ui/svelte';

  import Image from '$lib/components/atoms/Image.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import Question from '~icons/ph/question';
  import RocketLaunch from '~icons/ph/rocket-launch';
  import WarningCircle from '~icons/ph/warning-circle-fill';
  import X from '~icons/ph/x-bold';

  const {
    elements: { trigger, content, arrow, close },
    states: { open },
  } = createPopover();

  // let selected_credentials = $state.credentials?.filter(
  //   (c) => $state.current_user_prompt.options.indexOf(c.id) > -1
  // );

  let client_name = $state.current_user_prompt.client_name;

  const previously_connected = $state.current_user_prompt.previously_connected;

  const hostname = new URL($state.current_user_prompt.redirect_uri).hostname;

  console.log({ '$state.current_user_prompt': $state.current_user_prompt });

  onDestroy(async () => {
    // TODO: is onDestroy also called when user accepts since the component itself is destroyed?
    dispatch({ type: '[User Flow] Cancel', payload: {} });
  });
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={$LL.SCAN.CONNECTION_REQUEST.NAVBAR_TITLE()} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    {#if $state.current_user_prompt.logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver">
        <Image id={'client_0'} isTempAsset={true} />
      </div>
    {:else}
      <PaddedIcon icon={PlugsConnected} />
    {/if}
    <div class="text-center">
      <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
        {client_name}
      </p>
      <p class="pt-[10px] text-sm font-medium text-slate-500">
        {hostname}
      </p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-2 rounded-2xl bg-white p-3 dark:bg-dark">
      <!-- Warning -->
      {#if !previously_connected}
        <div class="flex w-full items-center rounded-lg bg-silver px-4 py-4 dark:bg-navy">
          <span class="mr-4 h-6 w-6">
            <WarningCircle class="h-6 w-6 text-amber-500" />
          </span>
          <div class="flex flex-col">
            <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {$LL.SCAN.CONNECTION_REQUEST.TITLE()}
            </p>
            <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
              {$LL.SCAN.CONNECTION_REQUEST.DESCRIPTION()}
            </p>
          </div>
        </div>
      {/if}

      <div
        class="flex justify-between rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
      >
        <p class="mr-3 text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">URL</p>
        <p class="break-all text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
          {$state.current_user_prompt.redirect_uri}
        </p>
      </div>
      <div
        class="flex items-center justify-between rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
      >
        <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
          {$LL.SCAN.CONNECTION_REQUEST.CONNECTED_PREVIOUSLY()}
        </p>
        {#if previously_connected}
          <Check class="text-emerald-500" />
        {:else}
          <X class="text-rose-500" />
        {/if}
      </div>
      <!-- TODO: feature disabled: "Verify .well-known" -->
      <!-- <div
        class="flex justify-between rounded-xl border border-slate-200 bg-white px-4 py-4 dark:border-slate-600 dark:bg-dark"
      >
        <div class="flex items-center">
          <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Verified</p>
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
                  class="underline underline-offset-2"
                  >{$state.current_user_prompt.client_name}</span
                >
                to provide you with a secure login.
              </div>
            </div>
          {/if}
        </div>
        <Check class="text-emerald-500" />
      </div> -->
    </div>
  </div>

  <!-- Controls -->
  <!-- TODO: on iOS subtract the --safe-area-inset-bottom from the bottom-padding -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.CONNECTION_REQUEST.ACCEPT()}
      on:click={() =>
        dispatch({
          type: '[Authenticate] Connection accepted',
        })}
    />
    <Button
      label={$LL.REJECT()}
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
        goto('/me');
      }}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
