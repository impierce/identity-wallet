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

  import { determineTheme } from './utils';

  onMount(async () => {
    const detach = await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let expandedDevMenu = true;
  let showDebugMessages = false;
  let showDragonProfileSteps = false;

  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');

  systemColorScheme.addEventListener(
    'change',
    (e) => {
      determineTheme(e.matches, $state?.profile_settings.profile?.theme);
    },
    { once: true },
  );

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('+layout.svelte: state', $state);

    // needed again?
    determineTheme(systemColorScheme.matches, $state?.profile_settings.profile?.theme);

    // User prompt
    let type = $state?.current_user_prompt?.type;

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }
  }

  interface DevModeButton {
    icon?: typeof SvelteComponent<SvelteHTMLElements['svg']> | string;
    onClick: () => void;
  }

  function createDevButtons(): DevModeButton[] {
    const backButton: DevModeButton = {
      icon: ArrowLeft,
      onClick: () => history.back(),
    };

    const resetButton: DevModeButton = {
      icon: Trash,
      onClick: () => dispatch({ type: '[App] Reset' }),
    };

    const ferrisButton: DevModeButton = {
      icon: '🦀',
      onClick: () => loadFerrisProfile(),
    };

    const dragonButton: DevModeButton = {
      icon: '🐲',
      onClick: () => (showDragonProfileSteps = !showDragonProfileSteps),
    };

    const debugButton: DevModeButton = {
      icon: ScrollText,
      onClick: () => (showDebugMessages = !showDebugMessages),
    };

    return [backButton, resetButton, ferrisButton, dragonButton, debugButton];
  }

  const devButtons = createDevButtons();

  // Order needs to match the rust side: 'ProfileSteps' enum, it needs to be the same order because every step is based upon the previous.
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
    await dispatch({ type: '[DEV] Load DEV profile', payload: { profile: 'Ferris', execute_step: null } });

    // Reload page, see why not just location.reload()
    // https://stackoverflow.com/questions/75960306/sveltekit-how-to-reload-current-page-via-the-client-side-router-using-goto'
    await goto('/');
    await goto('/me');
  }

  async function loadDragonProfile(steps: ProfileSteps) {
    await dispatch({
      type: '[DEV] Load DEV profile',
      payload: {
        profile: 'Dragon',
        execute_step: steps,
      },
    });

    showDragonProfileSteps = false;

    if (steps == 'CompleteFlow') {
      // Reload page, see why not just location.reload()
      // https://stackoverflow.com/questions/75960306/sveltekit-how-to-reload-current-page-via-the-client-side-router-using-goto'
      await goto('/');
      await goto('/me');
    }
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if $state?.dev_mode !== 'Off'}
    {#if expandedDevMenu}
      <div
        class="hide-scrollbar fixed z-20 flex w-full space-x-4 overflow-x-auto bg-gradient-to-r from-red-200 to-red-300 p-4 shadow-md"
        in:fly={{ y: -64, opacity: 1 }}
        out:fly={{ y: -64, opacity: 1 }}
      >
        {#each devButtons as button}
          <button
            class="rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60"
            on:click={button.onClick}
          >
            {#if typeof button.icon === 'string'}
              <span class="m-auto block text-xl">{button.icon}</span>
            {:else}
              <svelte:component this={button.icon} class="m-auto block text-xl" />
            {/if}
          </button>
        {/each}
      </div>
    {/if}

    <button
      class="fixed left-[calc(50%_-_12px)] top-[var(--safe-area-inset-top)] z-30 h-6 w-6 rounded-b-md bg-red-200 p-[2px]"
      on:click={() => (expandedDevMenu = !expandedDevMenu)}
    >
      {#if expandedDevMenu}
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

      <hr class="mx-8 h-1 bg-orange-800" />

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
