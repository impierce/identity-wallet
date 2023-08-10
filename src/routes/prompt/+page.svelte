<script lang="ts">
  import { state } from '../../stores';
  import { TopNavigation } from '@impierce/ui-components';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import WarningCircle from '~icons/ph/warning-circle-fill';
  import X from '~icons/ph/x-bold';
  import Check from '~icons/ph/check-bold';
  import Question from '~icons/ph/question';
  import { createPopover, melt } from '@melt-ui/svelte';
  import { fade } from 'svelte/transition';
  import { dispatch } from '$lib/dispatcher';
  import { goto } from '$app/navigation';

  const {
    elements: { trigger, content, arrow, close },
    states: { open }
  } = createPopover();
</script>

<div class="content-height flex flex-col items-stretch bg-neutral-100">
  <TopNavigation title={$state?.current_user_flow?.type ?? 'no title'} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-6">
    <div class="rounded-2xl bg-indigo-500 p-4">
      <PlugsConnected class="h-8 w-8 text-white" />
    </div>
    <p class="text-2xl font-medium">BestDex</p>

    <!-- Warning -->
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4">
      <WarningCircle class="mr-2 h-6 w-6 text-amber-500" />
      <p class="text-sm">Only accept connections that you trust.</p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white">
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <p>URL</p>
        <p class="text-neutral-600">bestdex.com</p>
      </div>
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <p>Connected previously</p>
        <X class="text-red-600" />
      </div>
      <div class="flex justify-between rounded-lg bg-white px-4 py-4">
        <div class="flex items-center">
          <p>Verified</p>
          <button class="-m-2 ml-1 rounded-full p-1" use:melt={$trigger}>
            <Question class="h-6 w-6 text-indigo-500" />
          </button>
          {#if $open}
            <div
              use:melt={$content}
              transition:fade={{ duration: 200 }}
              class="z-10 w-1/2 rounded-lg bg-indigo-500 p-2 text-white shadow-md"
            >
              <div use:melt={$arrow} />
              <div class="break-keep text-sm">
                Your UniMe app automatically tries to verify the identity of <span
                  class="underline underline-offset-2">BestDex</span
                >
                to provide you with a secure login.
                <!-- by contacting their
                <span class="w-fit rounded bg-neutral-200 p-1 font-mono text-xs text-neutral-700">
                  .well-known
                </span>
                endpoint. -->
              </div>
            </div>
          {/if}
        </div>
        <Check class="text-green-600" />
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 rounded-t-2xl bg-white p-6">
    <button class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white" on:click={() => {}}
      >Accept connection</button
    >
    <button
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}>Reject</button
    >
  </div>

  <!-- safe-area -->
  <div class="fixed top-0 z-50 h-[var(--safe-area-inset-top)] w-full bg-white opacity-80" />
  <div
    class="fixed bottom-0 z-10 h-[var(--safe-area-inset-bottom)] w-full bg-neutral-100 opacity-80 dark:bg-slate-800"
  />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
