<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';
  import {
    ArrowLeft,
    ChevronDown,
    ChevronUp,
    Clipboard,
    ExclamationTriangle,
    Trash
  } from 'svelte-heros-v2';
  import { fly } from 'svelte/transition';

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
  import { attachConsole, error, info, trace } from '@tauri-apps/plugin-log';

  import Alert from '$lib/alert/Alert.svelte';
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { loadAllLocales } from '$src/i18n/i18n-util.sync';
  import { state } from '$src/stores';

  import CaretDown from '~icons/ph/caret-down-bold';
  import CaretUp from '~icons/ph/caret-up-bold';

  import type { CurrentUserFlowType } from '../../src-tauri/bindings/user-flow/CurrentUserFlowType';
  import type { Selection } from '../../src-tauri/bindings/user-flow/Selection';
  import '../app.css';
  import { determineTheme } from './utils';

  let clipboard: string | undefined;

  onMount(async () => {
    const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = false;

  // Alert (global)
  // TODO: refactor: move to separate component
  let alertOpen = false;

  let dialog: UserDialog | undefined;

  interface UserDialog {
    type: 'select-credentials' | 'credential-offer';
    title: string;
    imageSrc?: string;
    options: string[];
  }

  let showDebugMessages = false;

  // set color scheme
  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');
  systemColorScheme.addEventListener('change', (e) => {
    determineTheme(e.matches, $state?.active_profile?.theme);
  });
  systemColorScheme.removeEventListener('change', () => {});

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('+layout.svelte: state', $state);

    // needed again?
    determineTheme(systemColorScheme.matches, $state?.active_profile?.theme);

    // User prompt
    let type = $state?.current_user_prompt?.type;
    console.log('options', ($state?.current_user_prompt as Selection)?.options);

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }

    // Enable deprecated "dialog"?
    if (false) {
      if (type === 'select-credentials') {
        dialog = {
          type: 'select-credentials',
          title: $LL.SHARE_CREDENTIALS_TITLE(),
          imageSrc: 'image/undraw_fingerprint_login_re_t71l.svg',
          options: ($state.current_user_prompt as Selection).options
        };
      } else if (type === 'credential-offer') {
        dialog = {
          type: 'credential-offer',
          title: 'Credential Offer',
          imageSrc: 'image/undraw_agreement_re_d4dv.svg',
          options: ($state.current_user_prompt as Selection).options
        };
      }
      alertOpen = true;
      if ($state?.current_user_prompt === null) {
        dialog = undefined;
        alertOpen = false;
      }
    }
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if showDevMode}
    <div
      class="hide-scrollbar fixed z-20 flex w-full space-x-4 overflow-x-auto bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
      in:fly={{ y: -64, opacity: 1 }}
      out:fly={{ y: -64, opacity: 1 }}
    >
      <button
        class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => history.back()}
      >
        <ArrowLeft />
      </button>
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[App] Reset' })}><Trash /></button
      >
      <button
        class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
        on:click={() => dispatch({ type: '[DEV] Load profile' })}
      >
        <span class="text-[18px]/[18px]">🦀</span>
      </button>
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
    class="fixed left-[calc(50%_-_12px)] top-[var(--safe-area-inset-top)] z-30 h-6 w-6 rounded-b-md bg-red-200 p-[2px]"
    on:click={() => (showDevMode = !showDevMode)}
  >
    {#if showDevMode}
      <CaretUp class="text-red-700" />
    {:else}
      <CaretDown class="text-red-700" />
    {/if}
  </button>

  <!-- Dev Mode: Debug messages -->
  {#if showDebugMessages}
    <div class="relative z-10 h-full w-screen bg-orange-100">
      <p class="p-4 text-center text-xs font-semibold uppercase text-orange-800">debug messages</p>
      {#each $state.debug_messages as message}
        <div class="mx-2 mb-2 rounded bg-orange-200 bg-opacity-60 p-2">
          <div class="break-all font-mono text-xs text-orange-700">{message}</div>
        </div>
      {/each}
    </div>
  {/if}

  <!-- Content -->
  <div class="fixed top-[var(--safe-area-inset-top)] h-auto w-full">
    <slot />
  </div>

  <!-- Alert -->
  {#if dialog}
    <Alert isOpen={alertOpen} {...dialog} />
  {/if}
</main>
