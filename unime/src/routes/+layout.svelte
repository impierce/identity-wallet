<script lang="ts">
  import { onDestroy, onMount, type SvelteComponent } from 'svelte';

  import { goto } from '$app/navigation';
  import { PUBLIC_DEV_MODE_MENU_EXPANDED } from '$env/static/public';
  import LL, { setLocale } from '$i18n/i18n-svelte';
  import { loadAllLocales } from '$i18n/i18n-util.sync';
  import type { SvelteHTMLElements } from 'svelte/elements';
  import { fly } from 'svelte/transition';

  import type { AppState } from '@bindings/AppState';
  import type { ProfileSteps } from '@bindings/dev/ProfileSteps';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { attachConsole, error, info } from '@tauri-apps/plugin-log';

  import { Switch } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    ArrowLeftRegularIcon,
    BugRegularIcon,
    CaretDownBoldIcon,
    CaretUpBoldIcon,
    TrashRegularIcon,
  } from '$lib/icons';
  import { state as appState, error as errorState } from '$lib/stores';

  import ErrorToast from './ErrorToast.svelte';
  import { determineTheme } from './utils';

  import '../app.css';

  let detachConsole: UnlistenFn;
  let unlistenError: UnlistenFn;
  let unlistenStateChanged: UnlistenFn;

  onMount(async () => {
    detachConsole = await attachConsole();

    loadAllLocales(); //TODO: performance: only load locale on user request

    unlistenError = await listen('error', (event) => {
      error(`Error: ${event.payload}`);
      errorState.set(event.payload as string);
    });

    unlistenStateChanged = await listen('state-changed', (event) => {
      // Set frontend state to state received from backend.
      appState.set(event.payload as AppState);

      // Update locale based on the frontend state.
      setLocale($appState.profile_settings.locale);

      let redirectPath: string | undefined;

      if ($appState.current_user_prompt) {
        // Generic redirect.
        if ($appState.current_user_prompt.type === 'redirect') {
          redirectPath = `/${$appState.current_user_prompt.target}`;
        }
        // Prompt redirect.
        else {
          redirectPath = `/prompt/${$appState.current_user_prompt.type}`;
        }
      }

      if (redirectPath) {
        info(`Redirecting to: ${redirectPath}.`);
        try {
          goto(redirectPath);
        } catch (e) {
          error(`Failed to redirect to ${redirectPath}: ${e}`);
        }
      }
    });

    dispatch({ type: '[App] Get state' });
  });

  onDestroy(() => {
    // Destroy in reverse order.
    unlistenStateChanged();
    unlistenError();
    detachConsole();
  });

  let expandedDevMenu = PUBLIC_DEV_MODE_MENU_EXPANDED === 'true';
  let showDebugMessages = false;
  let showDragonProfileSteps = false;
  let resetDragonProfile = true;

  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');

  systemColorScheme.addEventListener('change', (e) => {
    if ($appState?.profile_settings.profile?.theme) {
      determineTheme(e.matches, $appState.profile_settings.profile.theme);
    } else {
      determineTheme(systemColorScheme.matches, 'system');
    }
  });

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    if ($appState?.profile_settings.profile?.theme) {
      determineTheme(systemColorScheme.matches, $appState.profile_settings.profile.theme);
    } else {
      determineTheme(systemColorScheme.matches, 'system');
    }
  }

  interface DevModeButton {
    icon: typeof SvelteComponent<SvelteHTMLElements['svg']> | string;
    onClick: () => void;
  }

  function createDevButtons(): DevModeButton[] {
    const backButton: DevModeButton = {
      icon: ArrowLeftRegularIcon,
      onClick: () => history.back(),
    };

    const resetButton: DevModeButton = {
      icon: TrashRegularIcon,
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
      icon: BugRegularIcon,
      onClick: () => (showDebugMessages = !showDebugMessages),
    };

    return [backButton, resetButton, ferrisButton, dragonButton, debugButton];
  }

  const devButtons = createDevButtons();

  // Order needs to match the rust side: 'ProfileSteps' enum, it needs to be the same order because every step is based upon the previous.
  // 'AddCredentials' is ran after 'CreateProfile' and 'AcceptCredentials' after 'AddCredentials', etc.
  const profileSteps: ProfileSteps[] = [
    'Create profile',
    'Add credentials',
    'Accept credentials',
    'Add connection',
    'Accept connection',
    'Add presentation',
    'Share credentials',
    'Add future engineer',
    'Complete flow',
  ];

  async function loadFerrisProfile() {
    await dispatch({
      type: '[DEV] Load DEV profile',
      payload: { profile: 'Ferris', execute_step: null, reset_profile: false },
    });

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
        reset_profile: resetDragonProfile,
        execute_step: steps,
      },
    });

    showDragonProfileSteps = false;

    if (steps == 'Complete flow') {
      // Reload page, see why not just location.reload()
      // https://stackoverflow.com/questions/75960306/sveltekit-how-to-reload-current-page-via-the-client-side-router-using-goto'
      await goto('/');
      await goto('/me');
    }
  }
</script>

<main class="absolute h-screen">
  <!-- Dev Mode: Navbar -->
  {#if $appState?.dev_mode !== 'Off'}
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
        <CaretUpBoldIcon class="text-red-700" />
      {:else}
        <CaretDownBoldIcon class="text-red-700" />
      {/if}
    </button>
  {/if}

  <!-- Dev Mode: Debug messages -->
  {#if showDebugMessages}
    <div class="relative z-10 min-h-full w-screen bg-orange-100 pt-24">
      <p class="pb-2 pt-2 text-center text-xs font-semibold uppercase text-orange-800">debug messages</p>

      <hr class="mx-8 h-1 bg-orange-800" />

      {#each $appState.debug_messages as message}
        <div class="mx-2 mb-2 rounded bg-orange-200 p-2">
          <div class="break-all font-mono text-xs text-orange-700">{message}</div>
        </div>
      {/each}
    </div>
  {/if}

  {#if showDragonProfileSteps}
    <div class="fixed z-10 flex h-screen w-screen justify-center bg-black/50 pt-24">
      <div class="ml-10 mr-10 mt-10 flex h-fit w-full flex-col rounded bg-white pb-4 pl-4 pr-4">
        <p class="pb-2 pt-2 text-center text-orange-800">Profile steps</p>

        <div class="flex items-center justify-end pb-2">
          <div class="mr-2 text-xs text-orange-800">Reset profile?</div>
          <Switch
            active={resetDragonProfile}
            on:change={() => {
              resetDragonProfile = !resetDragonProfile;
            }}
          />
        </div>

        {#each profileSteps as steps, i}
          <button class="mx-auto mb-2 w-full rounded bg-orange-200 p-2" on:click={() => loadDragonProfile(steps)}>
            <div class="break-all font-mono text-xs text-orange-700">{i + 1}: {steps}</div>
          </button>
        {/each}
      </div>
    </div>
  {/if}

  <!-- Content -->
  <div class="fixed top-[var(--safe-area-inset-top)] h-auto w-full">
    <slot />
    <!-- Show error if exists -->
    {#if $errorState}
      <div class="absolute bottom-4 right-4 w-[calc(100%_-_32px)]">
        <ErrorToast
          title={$appState?.dev_mode !== 'Off' ? 'Error' : $LL.ERROR.TITLE()}
          detail={$appState?.dev_mode !== 'Off' ? $errorState : $LL.ERROR.DEFAULT_MESSAGE()}
          on:dismissed={() => {
            // After the toast fires the "dismissed" event, we reset $errorStore.
            errorState.set(undefined);
          }}
          autoDismissAfterMs={$appState?.dev_mode !== 'Off' ? 0 : 5_000}
        />
      </div>
    {/if}
  </div>
</main>
