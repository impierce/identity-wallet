<script lang="ts">
  import { onMount, SvelteComponent } from 'svelte';

  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import { attachConsole } from '@tauri-apps/plugin-log';

  import { dispatch } from '$lib/dispatcher';
  import { loadAllLocales } from '$src/i18n/i18n-util.sync';
  import { state } from '$src/stores';

  import ScrollText from '~icons/lucide/scroll-text';
  import ArrowLeft from '~icons/ph/arrow-left';
  import CaretDown from '~icons/ph/caret-down-bold';
  import CaretUp from '~icons/ph/caret-up-bold';
  import Trash from '~icons/ph/trash';

  import '../app.css';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import type { ProfileType } from '@bindings/actions/DevProfileType';

  import { determineTheme } from './utils';

  onMount(async () => {
    const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let expandDevMenu = true;
  let showDebugMessages = false;

  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');

  systemColorScheme.addEventListener('change', (e) => {
    determineTheme(e.matches, $state?.active_profile?.theme);
    systemColorScheme.removeEventListener('change', () => {});
  });

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

  interface DevModeButton {
    svelteIcon?: typeof SvelteComponent<SvelteHTMLElements['svg']>;
    stringIcon?: string;
    onClick: () => void;
  }

  function createDevButtons(): DevModeButton[] {
    const backBtn: DevModeButton = {
      svelteIcon: ArrowLeft,
      onClick: () => history.back(),
    };

    const resetBtn: DevModeButton = {
      svelteIcon: Trash,
      onClick: () => dispatch({ type: '[App] Reset' }),
    };

    const ferrisBtn: DevModeButton = {
      stringIcon: '🦀',
      onClick: () => loadProfile('Ferris'),
    };

    const dragonBtn: DevModeButton = {
      stringIcon: '🐲',
      onClick: () => loadProfile('Dragon'),
    };

    const debugBtn: DevModeButton = {
      svelteIcon: ScrollText,
      onClick: () => (showDebugMessages = !showDebugMessages),
    };

    return [backBtn, resetBtn, ferrisBtn, dragonBtn, debugBtn];
  }

  const devButtons = createDevButtons();

  async function loadProfile(profile: ProfileType) {
    dispatch({ type: '[DEV] Load DEV profile', payload: { profile } }).then(() => {
      // Reload page
      setTimeout(async () => {
        await goto('/');
        await goto('/me');
      }, 500);
    });
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if $state?.dev_profile}
    {#if expandDevMenu}
      <div
        class="hide-scrollbar fixed z-20 flex w-full content-center overflow-x-auto bg-gradient-to-r from-red-200 to-red-300 p-4 pt-8 shadow-md"
        in:fly={{ y: -64, opacity: 1 }}
        out:fly={{ y: -64, opacity: 1 }}
      >
        {#each devButtons as btn}
          <div class="mr-4 flex flex-col content-start justify-center">
            <button
              class="m-0 flex h-10 w-10 content-center rounded-full bg-red-300 p-0 text-sm
                font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
              on:click={btn.onClick}
            >
              {#if btn.svelteIcon}
                <svelte:component this={btn.svelteIcon} class="m-auto block text-xl" />
              {:else}
                <span class="m-auto block text-xl">{btn.stringIcon}</span>
              {/if}
            </button>
          </div>
        {/each}
      </div>
    {/if}

    <button
      class="fixed left-[calc(50%_-_12px)] top-[var(--safe-area-inset-top)] z-30 h-6 w-6 rounded-b-md bg-red-200 p-[2px]"
      on:click={() => (expandDevMenu = !expandDevMenu)}
    >
      {#if expandDevMenu}
        <CaretUp class="text-red-700" />
      {:else}
        <CaretDown class="text-red-700" />
      {/if}
    </button>
  {/if}

  <!-- Dev Mode: Debug messages -->
  {#if showDebugMessages}
    <div class="relative z-10 min-h-full w-screen bg-orange-100 pt-24">
      <p class="pb-2 pt-2 text-center text-xs font-semibold uppercase text-orange-800">debug messages</p>

      <hr class="mx-8 h-0.5 border-t-0 bg-orange-400 opacity-100" />

      {#each $state.debug_messages as message}
        <div class="mx-2 mb-2 rounded bg-orange-200 p-2">
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
