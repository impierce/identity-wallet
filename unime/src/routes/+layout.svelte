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

  import type { ProfileSteps } from '@bindings/dev/ProfileSteps';
  import type { ProfileType } from '@bindings/dev/ProfileType';

  import { determineTheme } from './utils';

  onMount(async () => {
    const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let expandDevMenu = true;
  let showDebugMessages = false;
  let showDragonProfileSteps = false;

  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');

  systemColorScheme.addEventListener(
    'change',
    (e) => {
      if ($state?.active_profile?.theme) {
        determineTheme(e.matches, $state.active_profile.theme);
      }
    },
    { once: true },
  );

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
    stringIcon?: string;
    svelteIcon?: typeof SvelteComponent<SvelteHTMLElements['svg']>;
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
      onClick: () => loadFerrisProfile(),
    };

    const dragonBtn: DevModeButton = {
      stringIcon: '🐲',
      onClick: () => (showDragonProfileSteps = !showDragonProfileSteps),
    };

    const debugBtn: DevModeButton = {
      svelteIcon: ScrollText,
      onClick: () => (showDebugMessages = !showDebugMessages),
    };

    return [backBtn, resetBtn, debugBtn, ferrisBtn, dragonBtn];
  }

  const devButtons = createDevButtons();

  // Order needs to match the BE: 'ProfileSteps' enum, it needs to be the same order because every step is based upon the previous.
  // 'AddCredentials' is ran after 'CreateProfile' and 'AcceptCredentials' after 'AddCredentials', etc.
  const profileSteps: ProfileSteps[] = [
    'CreateProfile',
    'AddCredentials',
    'AcceptCredentials',
    'AddConnection',
    'AcceptConnection',
    'AddPresentation',
    'ShareCredentails',
    'AddFutureEngineer',
    'CompleteFlow',
  ];

  async function loadFerrisProfile() {
    dispatch({ type: '[DEV] Load DEV profile', payload: { profile: 'Ferris', execute_steps: null } }).then(() => {
      // Reload page
      setTimeout(async () => {
        await goto('/');
        await goto('/me');
      }, 500);
    });
  }

  async function loadDragonProfile(steps: ProfileSteps) {
    await dispatch({
      type: '[DEV] Load DEV profile',
      payload: {
        profile: 'Dragon',
        execute_steps: steps,
      },
    });

    showDragonProfileSteps = false;

    if (steps == 'CompleteFlow') {
      // Reload page
      setTimeout(async () => {
        await goto('/');
        await goto('/me');
      }, 500);
    }
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if $state?.dev_mode !== 'Off'}
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
              {#if btn.stringIcon}
                <span class="m-auto block text-xl">{btn.stringIcon}</span>
              {:else}
                <svelte:component this={btn.svelteIcon} class="m-auto block text-xl" />
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

      <hr class="mx-8 h-0.5 border-t-0 bg-orange-800 opacity-100" />

      {#each $state.debug_messages as message}
        <div class="mx-2 mb-2 rounded bg-orange-200 p-2">
          <div class="break-all font-mono text-xs text-orange-700">{message}</div>
        </div>
      {/each}
    </div>
  {/if}

  {#if showDragonProfileSteps}
    <div class="fixed z-10 flex h-screen w-screen justify-center bg-black/50 pt-24">
      <div class="ml-10 mr-10 mt-10 h-fit w-full rounded bg-white pb-4">
        <p class="pb-2 pt-2 text-center text-orange-800">Profile steps</p>

        {#each profileSteps as steps, i}
          <button
            class="mx-auto mb-2 block w-11/12 rounded bg-orange-200 p-2"
            on:click={() => loadDragonProfile(steps)}
          >
            <div class="break-all font-mono text-xs text-orange-700">{i + 1}: {steps}</div>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Content -->
  <div class="fixed top-[var(--safe-area-inset-top)] h-auto w-full">
    <slot />
  </div>
</main>
