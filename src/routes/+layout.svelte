<script lang="ts">
  import '../app.css';

  import { fly } from 'svelte/transition';
  import { ChevronUp, ChevronDown, ArrowLeft, Trash, UserPlus, Clipboard } from 'svelte-heros-v2';
  import { state } from '../stores';
  import LL from '../i18n/i18n-svelte';
  import { onMount } from 'svelte';
  import { loadAllLocales } from '../i18n/i18n-util.sync';
  import { dispatch } from '$lib/dispatcher';
  import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
    Button
  } from '@impierce/ui-components';
  import { readText } from '@tauri-apps/plugin-clipboard-manager';
  import { trace, info, error, attachConsole } from "@tauri-apps/plugin-log";
  import Alert from '$lib/alert/Alert.svelte';
  import type { CurrentUserFlowType } from '../../src-tauri/bindings/user-flow/CurrentUserFlowType';

  let clipboard: string | undefined;

  onMount(async () => {
  const detach = await attachConsole();
    console.log('+layout.svelte: onMount');
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = false;

  // alert (selection)
  let alertOpen = false;
  let alertOptions: string[] = [];
  let alertTitle: string = 'title';

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('state', $state);
    if ($state?.current_user_flow?.Selection) {
      alertOpen = true;
      alertOptions = $state.current_user_flow.Selection.options;
      if (($state.current_user_flow.Selection.type as CurrentUserFlowType) === 'select-credentials') {
        alertTitle = $LL.SHARE_CREDENTIALS_TITLE();
      }
    }
  }
</script>

<main class="h-screen bg-slate-100">
  <!-- begin: dev mode -->
  {#if showDevMode}
    <div
      class="hide-scrollbar fixed z-10 flex w-full space-x-4 overflow-x-auto bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
      in:fly={{ y: -64, opacity: 1 }}
      out:fly={{ y: -64, opacity: 1 }}
    >
      <button
        class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
      >
        <ArrowLeft />
      </button>
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[App] Reset' })}><Trash /></button
      >
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[DEV] Load profile' })}>🦀</button
      >
      <!-- Paste from Clipboard -->
      <AlertDialog>
        <AlertDialogTrigger>
          <button
            class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
            on:click={async () => (clipboard = await readText())}><Clipboard /></button
          >
        </AlertDialogTrigger>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Paste from clipboard?</AlertDialogTitle>
            <AlertDialogDescription>
              <div class="rounded-lg bg-slate-200 p-6">
                <div class="text-mono break-all text-slate-400">{clipboard}</div>
              </div>
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction>Paste</AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  {/if}
  <button
    class="fixed right-3 top-3 z-10 rounded-full p-2 hover:bg-red-200 {showDevMode
      ? 'bg-red-200'
      : ''}"
    on:click={() => (showDevMode = !showDevMode)}
  >
    {#if showDevMode}
      <ChevronUp class="text-red-700" strokeWidth="2" />
    {:else}
      <ChevronDown class="text-red-700" strokeWidth="2" />
    {/if}
  </button>
  <!-- end: dev mode -->
  <div class="h-auto">
    <!-- <Route path="welcome" component={Welcome} /> -->
    <!-- <Route path="profile" component={Profile} primary={false} /> -->
    <slot />
  </div>
  <Alert rootOpen={alertOpen} title={alertTitle} options={alertOptions}/>
</main>
