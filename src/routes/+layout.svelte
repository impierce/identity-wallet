<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import { readText } from '@tauri-apps/plugin-clipboard-manager';
  import { attachConsole, error, info, trace } from '@tauri-apps/plugin-log';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { loadAllLocales } from '$src/i18n/i18n-util.sync';
  import { developer_mode, state } from '$src/stores';

  import ArrowLeft from '~icons/ph/arrow-left';
  import CaretDown from '~icons/ph/caret-down-bold';
  import CaretUp from '~icons/ph/caret-up-bold';
  import Clipboard from '~icons/ph/clipboard';
  import Trash from '~icons/ph/trash';
  import Warning from '~icons/ph/warning';

  import '../app.css';

  import { melt } from '@melt-ui/svelte';

  import Button from '$src/lib/components/Button.svelte';
  import BottomDrawer from '$srSelectionc/lib/components/molecules/dialogs/BottomDrawer.svelte';

  import { determineTheme } from './utils';

  let clipboard: string | undefined;

  onMount(async () => {
    const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let showDevMode = false;

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

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if $developer_mode}
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
          <ArrowLeft class="h-6 w-6" />
        </button>
        <button
          class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
          on:click={() => dispatch({ type: '[App] Reset' })}><Trash class="h-6 w-6" /></button
        >
        <button
          class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
          on:click={() => dispatch({ type: '[DEV] Load profile' })}
        >
          <span class="text-[18px]/[18px]">🦀</span>
        </button>

        <!-- Paste from Clipboard -->
        <BottomDrawer titleText="Paste from clipboard?">
          <button
            slot="trigger"
            let:trigger
            use:melt={trigger}
            class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
            on:click={async () => (clipboard = await readText())}><Clipboard class="h-6 w-6" /></button
          >
          <div slot="content" class="w-full pb-[10px] pt-[20px]">
            <div class="mb-4 rounded-2xl bg-slate-200 p-4">
              <div class="break-all font-mono text-sm text-slate-400">{clipboard}</div>
            </div>
            <Button label="Paste" disabled />
          </div>
        </BottomDrawer>

        <!-- Debug messages -->
        <button
          class="flex-shrink-0 rounded-full bg-red-300 px-4 py-2 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
          on:click={() => (showDebugMessages = !showDebugMessages)}><Warning class="h-6 w-6" /></button
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
  {/if}

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
</main>
