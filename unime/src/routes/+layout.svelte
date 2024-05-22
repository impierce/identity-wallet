<script lang="ts">
  import { onMount, SvelteComponent } from 'svelte';

  import { goto } from '$app/navigation';
  import { PUBLIC_DEV_MODE_MENU_EXPANDED } from '$env/static/public';
  import { loadAllLocales } from '$i18n/i18n-util.sync';
  import { fly } from 'svelte/transition';

  import { attachConsole } from '@tauri-apps/plugin-log';

  import { dispatch } from '$lib/dispatcher';
  import { error, state } from '$lib/stores';

  import ScrollText from '~icons/lucide/scroll-text';
  import ArrowLeft from '~icons/ph/arrow-left';
  import CaretDown from '~icons/ph/caret-down-bold';
  import CaretUp from '~icons/ph/caret-up-bold';
  import Trash from '~icons/ph/trash';

  import '../app.css';

  import type { SvelteHTMLElements } from 'svelte/elements';

  import type { ProfileSteps } from '@bindings/dev/ProfileSteps';

  import Switch from '$lib/components/atoms/Switch.svelte';
  import ErrorToast from '$lib/components/molecules/toast/ErrorToast.svelte';

  import { determineTheme } from './utils';

  onMount(async () => {
    await attachConsole();
    loadAllLocales(); //TODO: performance: only load locale on user request
    dispatch({ type: '[App] Get state' });
  });

  let expandedDevMenu = PUBLIC_DEV_MODE_MENU_EXPANDED === 'true';
  let showDebugMessages = false;
  let showDragonProfileSteps = false;
  let resetDragonProfile = true;

  const systemColorScheme = window.matchMedia('(prefers-color-scheme: dark)');

  systemColorScheme.addEventListener('change', (e) => {
    if ($state?.profile_settings.profile?.theme) {
      determineTheme(e.matches, $state.profile_settings.profile.theme);
    } else {
      determineTheme(systemColorScheme.matches, 'system');
    }
  });

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    if ($state?.profile_settings.profile?.theme) {
      determineTheme(systemColorScheme.matches, $state.profile_settings.profile.theme);
    } else {
      determineTheme(systemColorScheme.matches, 'system');
    }

    // User prompt
    let type = $state?.current_user_prompt?.type;

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }
  }

  $error = 'This is a quite long test error message. We should handle it accordingly.';
  $: err = $error;

  interface DevModeButton {
    icon: typeof SvelteComponent<SvelteHTMLElements['svg']> | string;
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
    {#if err}
      <div class="absolute bottom-4 right-4 w-[calc(100%_-_32px)]">
        <ErrorToast
          title={'Whoops!'}
          detail={err}
          on:dismissed={() => ($error = undefined)}
          autoDismissAfterMs={5000}
        />
      </div>
    {/if}
  </div>
</main>
