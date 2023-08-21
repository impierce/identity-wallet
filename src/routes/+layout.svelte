<script lang="ts">
  import '../app.css';

  import { fly } from 'svelte/transition';
  import {
    ChevronUp,
    ChevronDown,
    ArrowLeft,
    Trash,
    Clipboard,
    ExclamationTriangle
  } from 'svelte-heros-v2';
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
  // import { trace, info, error, attachConsole } from '@tauri-apps/plugin-log';
  import Alert from '$lib/alert/Alert.svelte';
  import OfferAlert from '$lib/offer_alert/OfferAlert.svelte';
  import type { CurrentUserPromptType } from '../../src-tauri/bindings/user-prompt/CurrentUserPromptType';
  import type { Selection } from '../../src-tauri/bindings/user-prompt/Selection';
  import type { CredentialOffer } from '../../src-tauri/bindings/user-prompt/CredentialOffer';

  let clipboard: string | undefined;

  onMount(async () => {
    // const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = false;

  // alert (selection)
  let alertOpen = false;
  let alertOptions: string[] = [];
  let alertTitle: string = 'title';

  let isSelectCredentials: boolean = false;
  let isCredentialOffer: boolean = false;

  let showDebugMessages = false;

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('+layout.svelte: state', $state);
    if ($state?.current_user_prompt?.type === 'select-credentials') {
      isSelectCredentials = true;
      isCredentialOffer = false;
      alertOpen = true;
      alertOptions = ($state.current_user_prompt as Selection).options;
      alertTitle = $LL.SHARE_CREDENTIALS_TITLE();
    } else if ($state?.current_user_prompt?.type === 'credential-offer') {
      isSelectCredentials = false;
      isCredentialOffer = true;
      alertOpen = true;
      alertOptions = ($state.current_user_prompt as CredentialOffer).credential_offer.credentials;
      alertTitle = ($state.current_user_prompt as CredentialOffer).credential_offer.credential_issuer;
    }
    if ($state?.current_user_prompt === null) {
      alertOpen = false;
    }
  }
</script>

<main class="h-screen">
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
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => (showDebugMessages = !showDebugMessages)}><ExclamationTriangle /></button
      >
    </div>
  {/if}
  <button
    class="safe-top-padding absolute right-3 z-10 rounded-full bg-red-200 p-2"
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
  {#if isSelectCredentials}
    <Alert isOpen={alertOpen} title={alertTitle} options={alertOptions} />
  {:else if isCredentialOffer}
    <OfferAlert isOpen={alertOpen} title={alertTitle} options={alertOptions} />
  {/if}

  {#if showDebugMessages}
    <div class="absolute left-0 top-16 z-50 h-screen w-screen bg-orange-100">
      <p class="p-4 text-center text-xs font-semibold uppercase text-orange-800">debug messages</p>
      {#each $state.debug_messages as message}
        <div class="mx-2 mb-2 rounded bg-orange-200 bg-opacity-60 p-2">
          <div class="break-all font-mono text-xs text-orange-700">{message}</div>
        </div>
      {/each}
    </div>
  {/if}
</main>

<style>
  .safe-top-padding {
    top: calc(env(safe-area-inset-top) + 0.75rem);
  }
</style>
