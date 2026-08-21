<script lang="ts">
  import { onDestroy, onMount, type Component } from 'svelte';

  import { beforeNavigate, goto } from '$app/navigation';
  import { page } from '$app/state';
  import {
    PUBLIC_DEV_MODE_MENU_EXPANDED,
    PUBLIC_DEV_SHOW_CURRENT_ROUTE,
    PUBLIC_STYLE_SAFE_AREA_INSETS,
  } from '$env/static/public';
  import LL, { setLocale } from '$i18n/i18n-svelte';
  import { loadAllLocales } from '$i18n/i18n-util.sync';
  import type { SVGAttributes } from 'svelte/elements';
  import { get, writable } from 'svelte/store';
  import { fly } from 'svelte/transition';

  import type { AppState } from '@bindings/AppState';
  import type { ProfileSteps } from '@bindings/dev/ProfileSteps';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { getCurrent, onOpenUrl } from '@tauri-apps/plugin-deep-link';
  import { attachConsole, error, info } from '@tauri-apps/plugin-log';

  import { DeprecatedSwitch, Toast } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import {
    ArrowLeftRegularIcon,
    BugRegularIcon,
    CaretDownBoldIcon,
    CaretUpBoldIcon,
    MagnifyingGlassIcon,
    TrashRegularIcon,
    XRegularIcon,
  } from '$lib/icons';
  import { state as appState, error as errorState, navigationDirection } from '$lib/stores';

  import '../app.css';

  import { loadIcons } from '$lib/icons/iconify';

  let detachConsole: UnlistenFn;
  let unlistenError: UnlistenFn;
  let unlistenStateChanged: UnlistenFn;
  let unlistenDeepLink: UnlistenFn;

  const pendingDeepLinkUrl = writable<URL | undefined>();

  async function processDeepLink(url: URL) {
    info(`App is ready, processing pending deep link: ${url}`);

    // Clear the URL from the store immediately to prevent it from being processed again.
    pendingDeepLinkUrl.set(undefined);

    switch (url.protocol) {
      // TODO: support App/Universal Links
      case 'unime:': {
        const code = url.searchParams.get('code') ?? '';
        const state = url.searchParams.get('state') ?? '';
        await dispatch({
          type: '[Credential Offer] Code received',
          payload: { code, is_pre_authorized: false, is_interactive: false, state },
        });
        break;
      }
      case 'openid-credential-offer:':
      // TODO: OpenID4VP versions prior to 1.0 referred to `openid://` as the deep link scheme instead of
      // `openid4vp://`. For now we support both for better compatibility, but we can remove support for `openid://`
      // in a future update.
      // eslint-disable-next-line no-fallthrough
      case 'openid:':
      case 'openid4vp:': {
        await dispatch({
          type: '[QR Code] Scanned',
          payload: { form_urlencoded: url.toString() },
        });
        break;
      }
      default: {
        error(`Received deep link with unhandled protocol: ${url.protocol}`);
        break;
      }
    }
  }

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

      // Process a deep link (only if the app is already unlocked).
      const url = get(pendingDeepLinkUrl);
      if ($appState?.is_unlocked && url) {
        processDeepLink(url);
      }

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

      // DEV: uncommenting this helps local development by always redirecting to the page you're working on
      // redirectPath = '/me/settings/about';
      
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

    // If the app is launched with a deep link, it is stored for later processing via the `state-changed` listener.
    const invocationUrls = await getCurrent();
    if (invocationUrls) {
      info(`App launched with deep links: ${invocationUrls}`);
      pendingDeepLinkUrl.set(new URL(invocationUrls[0]));
    }

    // If a deep link is received with the app already open, try processing it immediately.
    unlistenDeepLink = await onOpenUrl((urls) => {
      info(`Received deep link while running, storing for processing: ${urls[0]}`);
      const invocationUrl = new URL(urls[0]);
      pendingDeepLinkUrl.set(invocationUrl);

      if ($appState?.is_unlocked) {
        processDeepLink(invocationUrl);
      }
    });
  });

  onDestroy(() => {
    // Destroy in reverse order.
    unlistenDeepLink();
    unlistenStateChanged();
    unlistenError();
    detachConsole();
  });

  let expandedDevMenu = PUBLIC_DEV_MODE_MENU_EXPANDED === 'true';
  let styleSafeAreaInsets = PUBLIC_STYLE_SAFE_AREA_INSETS === 'true';
  let showDebugMessages = false;
  let showDragonProfileSteps = false;
  let resetDragonProfile = true;

  // Tailwind considers app to be in dark theme if the class `dark` is present.
  // A user can choose between 3 themes: `light`, `dark`, and `system`.

  // For `system` we need to check `prefers-color-scheme: dark` media query.
  // `prefersColorSchemeDarkStore` monitors whether this media query is applied.
  const prefersColorSchemeDarkQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const prefersColorSchemeDarkStore = writable(prefersColorSchemeDarkQuery.matches);

  prefersColorSchemeDarkQuery.addEventListener('change', () => {
    prefersColorSchemeDarkStore.set(prefersColorSchemeDarkQuery.matches);
  });

  // Function that determines if the `dark` class should be added.
  function addDarkClass() {
    if ($appState.profile_settings.profile?.theme === 'dark') {
      return true;
    }
    if ($appState.profile_settings.profile?.theme === 'light') {
      return false;
    }
    return $prefersColorSchemeDarkStore;
  }

  // `dark` is reactive and depends on the two stores.
  let dark: boolean;

  // ESLint does not understand the reactive statement.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  $: ($appState, $prefersColorSchemeDarkStore, (dark = addDarkClass()));

  $: {
    // User prompt
    let type = $appState?.current_user_prompt?.type;

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }
  }

  interface DevModeButton {
    icon: Component<SVGAttributes<SVGSVGElement>> | string;
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

  // The messages the backend collected, oldest first.
  $: debugMessages = $appState.debug_messages;

  // Debug messages are prefixed with `[YYYY-MM-DD][HH:MM:SS]` by the backend (but not all of them are).
  const debugMessageTimestamp = /^\[(\d{4}-\d{2}-\d{2})\]\[(\d{2}:\d{2}:\d{2})\]\s*/;

  function splitDebugMessage(message: string): { timestamp: string | null; text: string } {
    const match = message.match(debugMessageTimestamp);
    return match
      ? { timestamp: `${match[1]} ${match[2]}`, text: message.slice(match[0].length) }
      : { timestamp: null, text: message };
  }

  // The backend does not attach a log level to debug messages, so it has to be guessed from the text.
  // Known producers today:
  // - `command.rs` pushes a timestamped `{error:?}` whenever a state update fails (always an error).
  // - `refresh_credential_status.rs` ends its message with `(failed)` or `(success)`.
  // - `check_password.rs` pushes `Wrong Stronghold password` or `Stronghold password OK`.
  // The keyword match also covers messages added later, at the price of the occasional false positive.
  const debugMessageErrorPatterns = [debugMessageTimestamp, /\b(error|failed|failure|wrong|invalid)\b/i];

  function isDebugMessageError(message: string): boolean {
    return debugMessageErrorPatterns.some((pattern) => pattern.test(message));
  }

  type DebugMessageFilter = 'all' | 'errors' | 'success';

  const debugMessageFilters: { value: DebugMessageFilter; label: string }[] = [
    { value: 'all', label: 'All' },
    { value: 'errors', label: 'Errors' },
    { value: 'success', label: 'Success' },
  ];

  let debugMessageFilter: DebugMessageFilter = 'all';
  let debugMessageSearch = '';
  // Newest first is the more useful default when debugging: the backend appends new messages at the end.
  let debugMessagesNewestFirst = true;

  function filterDebugMessages(
    messages: string[],
    filter: DebugMessageFilter,
    search: string,
    newestFirst: boolean,
  ): string[] {
    const term = search.trim().toLowerCase();

    const filtered = messages.filter((message) => {
      const isError = isDebugMessageError(message);
      if (filter === 'errors' && !isError) return false;
      if (filter === 'success' && isError) return false;
      return term === '' || message.toLowerCase().includes(term);
    });

    // `filter` already returned a new array, so reversing in place is safe.
    return newestFirst ? filtered.reverse() : filtered;
  }

  $: visibleDebugMessages = filterDebugMessages(
    debugMessages,
    debugMessageFilter,
    debugMessageSearch,
    debugMessagesNewestFirst,
  );

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

  loadIcons();

  // Bind the div elements of th insets against these variables.
  let safeAreaInsetTop: HTMLDivElement;
  let safeAreaInsetBottom: HTMLDivElement;

  // The resize observers will update these heights.
  // Styling will be applied to the insets if the height is greater than 0.
  let safeAreaInsetTopHeight: number;
  let safeAreaInsetBottomHeight: number;

  // Resize observers.
  let resizeObserverInsetTop: ResizeObserver;
  let resizeObserverInsetBottom: ResizeObserver;

  onMount(() => {
    // ResizeObserver for safe area top inset.
    resizeObserverInsetTop = new ResizeObserver((entries) => {
      for (let entry of entries) {
        safeAreaInsetTopHeight = entry.contentRect.height;
      }
    });

    if (safeAreaInsetTop) {
      resizeObserverInsetTop.observe(safeAreaInsetTop);
    }

    // ResizeObserver for safe area bottom inset.
    resizeObserverInsetBottom = new ResizeObserver((entries) => {
      for (let entry of entries) {
        safeAreaInsetBottomHeight = entry.contentRect.height;
      }
    });

    if (safeAreaInsetBottom) {
      resizeObserverInsetBottom.observe(safeAreaInsetBottom);
    }
  });

  onDestroy(() => {
    if (resizeObserverInsetTop) {
      resizeObserverInsetTop.disconnect();
    }

    if (resizeObserverInsetBottom) {
      resizeObserverInsetBottom.disconnect();
    }
  });

  beforeNavigate(({ type, from, to }) => {
    if (!from || !to) return;

    if (type === 'popstate') {
      $navigationDirection = null;
      if (PUBLIC_DEV_SHOW_CURRENT_ROUTE === 'true') {
        info(`$navigationDirection: ${$navigationDirection}`);
      }
      return;
    }

    if (to.url.pathname.startsWith(from.url.pathname) || from.url.pathname.startsWith(to.url.pathname)) {
      if (to.url.pathname.length > from.url.pathname.length) {
        $navigationDirection = 'down';
      } else {
        $navigationDirection = 'up';
      }
    }

    if (PUBLIC_DEV_SHOW_CURRENT_ROUTE === 'true') {
      info(`$navigationDirection: ${$navigationDirection}`);
    }
  });
</script>

<!--
Stacking context: We have to deviate from the DOM-sequence.
- safe-area-inset-top (z-30): Always on top, but under ActionSheet blurr-out.
- safe-area-inset-bottom (z-50): Always on top.
- Expanded menu (z-10): flies in from under safe-area-inset-top.
- Dev menu expansion button (z-20): sits on top of the expanded menu.
- ActionSheet (z-40): between safe-area-inset-top and safe-area-inset-bottom.
-->

<!-- Set default background and text color. -->
<!-- A page can request a transparent background by setting `$page.data.transparent` (required by scan route). -->
<div class="overflow-hidden text-text {page.data.transparent ? 'bg-transparent' : 'bg-background'}" class:dark>
  <!-- Default background for `safe-area-inset-top` is `bg-background`. Make it `bg-background-alt` when flag is set.  -->
  <div
    bind:this={safeAreaInsetTop}
    class="safe-area-inset-top fixed top-0 z-30 w-full {page.data.bgAltTop ? 'bg-background-alt' : 'bg-background'}"
  >
    {#if $appState.dev_mode !== 'Off'}
      <!-- Apply border conditionally when the top inset is not 0. -->
      <div
        class="grid h-full place-items-center"
        style:border-bottom={safeAreaInsetTopHeight > 0 && styleSafeAreaInsets
          ? '2px dashed rgb(var(--color-text)'
          : 'none'}
      ></div>
    {/if}
  </div>

  <!-- safe-area-inset-bottom: highlight area when in dev mode. -->
  <div
    bind:this={safeAreaInsetBottom}
    class="safe-area-inset-bottom fixed bottom-0 z-50 w-full {page.data.bgAltBottom
      ? 'bg-background-alt'
      : 'bg-background'}"
  >
    {#if $appState.dev_mode !== 'Off'}
      <div
        class="grid h-full place-items-center"
        style:border-top={safeAreaInsetBottomHeight > 0 && styleSafeAreaInsets
          ? '2px dashed rgb(var(--color-text)'
          : 'none'}
      ></div>
    {/if}
  </div>

  <!--
  Add paddings to honor safe-area-insets.
  This is also the portal for the ActionSheet instead of default `body` to ensure it works with dark mode.
  -->
  <div id="portal" class="safe-area-insets h-screen">
    <!-- Dev Mode: Navbar -->
    {#if $appState?.dev_mode !== 'Off'}
      {#if expandedDevMenu}
        <div
          class="fixed z-10 flex hide-scrollbar w-full space-x-4 overflow-x-auto bg-linear-to-r from-red-200 to-red-300 p-4 shadow-md"
          in:fly={{ y: -64, opacity: 1 }}
          out:fly={{ y: -64, opacity: 1 }}
        >
          {#each devButtons as button}
            <button
              class="hover:ring-opacity-60 rounded-full bg-red-300 px-4 py-1 text-sm font-medium text-red-700 hover:ring-2 hover:ring-red-700 hover:outline-hidden"
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
        class="fixed top-(--safe-area-inset-top) left-[calc(50%-12px)] z-20 h-6 w-6 rounded-b-md bg-red-200 p-[2px]"
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
      <div
        class="fixed inset-0 z-40 flex flex-col bg-background pt-(--safe-area-inset-top) pb-(--safe-area-inset-bottom)"
      >
        <!-- Header: the two actions sit in opposite corners so they cannot be misclicked. -->
        <div
          class="grid shrink-0 grid-cols-[auto_1fr_auto] items-center gap-1 border-b border-slate-300 p-2 dark:border-slate-600"
        >
          <button
            class="rounded-full p-2 text-text-alt hover:bg-background-alt"
            aria-label="Clear debug messages"
            on:click={() => dispatch({ type: '[DEV] Clear debug log' })}
          >
            <TrashRegularIcon class="size-5" />
          </button>
          <p class="truncate text-center text-[13px]/[24px] font-semibold text-text">
            Debug messages
            <span class="font-mono font-normal text-text-alt">
              ({visibleDebugMessages.length === debugMessages.length
                ? debugMessages.length
                : `${visibleDebugMessages.length}/${debugMessages.length}`})
            </span>
          </p>
          <button
            class="rounded-full p-2 text-text-alt hover:bg-background-alt"
            aria-label="Close debug messages"
            on:click={() => (showDebugMessages = false)}
          >
            <XRegularIcon class="size-5" />
          </button>
        </div>

        <!-- Toolbar: search, level filter and sort order. Wraps onto a second line when it runs out of width. -->
        <div class="flex shrink-0 flex-wrap items-center gap-2 border-b border-slate-300 p-2 dark:border-slate-600">
          <!-- Search -->
          <div
            class="flex min-w-[120px] grow items-center gap-1 rounded-lg border border-slate-300 bg-background-alt px-2 dark:border-slate-600"
          >
            <MagnifyingGlassIcon class="size-4 shrink-0 text-text-alt" />
            <input
              type="text"
              bind:value={debugMessageSearch}
              placeholder="Search"
              aria-label="Search debug messages"
              class="h-7 w-full min-w-0 bg-transparent text-[12px] text-text placeholder:text-text-alt focus:outline-hidden"
            />
            <!-- Only takes up space while there is something to clear. -->
            {#if debugMessageSearch !== ''}
              <button
                class="shrink-0 rounded-full p-0.5 text-text-alt"
                aria-label="Clear search"
                on:click={() => (debugMessageSearch = '')}
              >
                <XRegularIcon class="size-3.5" />
              </button>
            {/if}
          </div>

          <!-- Level filter -->
          <div
            class="flex shrink-0 items-center gap-0.5 rounded-lg border border-slate-300 p-0.5 dark:border-slate-600"
          >
            {#each debugMessageFilters as { value, label }}
              <button
                class="rounded-md px-2 py-1 text-[11px]/[16px] font-medium {debugMessageFilter === value
                  ? 'bg-background-alt text-text'
                  : 'text-text-alt'}"
                on:click={() => (debugMessageFilter = value)}
              >
                {label}
              </button>
            {/each}
          </div>

          <!-- Sort order: one button that cycles, showing the order that is currently applied. -->
          <button
            class="flex shrink-0 items-center gap-1 rounded-lg border border-slate-300 px-2 py-1 text-[11px]/[16px] font-medium text-text-alt dark:border-slate-600"
            on:click={() => (debugMessagesNewestFirst = !debugMessagesNewestFirst)}
          >
            {#if debugMessagesNewestFirst}
              <CaretDownBoldIcon class="size-3" />
              Newest first
            {:else}
              <CaretUpBoldIcon class="size-3" />
              Oldest first
            {/if}
          </button>
        </div>

        <!-- The messages themselves: the only part that scrolls. -->
        <div class="grow overflow-y-auto p-3">
          {#if debugMessages.length === 0}
            <p class="pt-8 text-center text-[13px]/[24px] font-medium text-text-alt">No debug messages</p>
          {:else if visibleDebugMessages.length === 0}
            <p class="pt-8 text-center text-[13px]/[24px] font-medium text-text-alt">No messages match the filter</p>
          {:else}
            <div class="flex flex-col gap-2">
              {#each visibleDebugMessages as message}
                {@const { timestamp, text } = splitDebugMessage(message)}
                <div
                  class="rounded-lg border p-2 {isDebugMessageError(message)
                    ? 'border-rose-400 bg-rose-50 dark:border-rose-500/60 dark:bg-rose-500/10'
                    : 'border-slate-300 bg-background-alt dark:border-slate-600'}"
                >
                  {#if timestamp}
                    <p class="pb-1 font-mono text-[10px]/[14px] text-text-alt">{timestamp}</p>
                  {/if}
                  <p class="font-mono text-[11px]/[16px] break-all whitespace-pre-wrap text-text select-text">{text}</p>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </div>
    {/if}

    {#if showDragonProfileSteps}
      <div class="fixed z-10 flex h-screen w-screen justify-center bg-black/50 pt-24">
        <div class="mt-10 mr-10 ml-10 flex h-fit w-full flex-col rounded-sm bg-white pr-4 pb-4 pl-4">
          <!-- Headline, with a close button so the popup can be dismissed without picking a step. -->
          <div class="grid grid-cols-[auto_1fr_auto] items-center pt-2 pb-2">
            <div class="size-8"></div>
            <p class="text-center text-orange-800">Profile steps</p>
            <button
              class="rounded-full p-2 text-orange-800"
              aria-label="Close profile steps"
              on:click={() => (showDragonProfileSteps = false)}
            >
              <XRegularIcon class="size-4" />
            </button>
          </div>

          <div class="flex items-center justify-end pb-2">
            <div class="mr-2 text-xs text-orange-800">Reset profile?</div>
            <DeprecatedSwitch
              active={resetDragonProfile}
              on:change={() => {
                resetDragonProfile = !resetDragonProfile;
              }}
            />
          </div>

          {#each profileSteps as steps, i}
            <button class="mx-auto mb-2 w-full rounded-sm bg-orange-200 p-2" on:click={() => loadDragonProfile(steps)}>
              <div class="font-mono text-xs break-all text-orange-700">{i + 1}: {steps}</div>
            </button>
          {/each}
        </div>
      </div>
    {/if}

    {#if $appState?.dev_mode !== 'Off' && PUBLIC_DEV_SHOW_CURRENT_ROUTE === 'true'}
      <div class="absolute bottom-4 left-4 z-50 rounded bg-orange-100 px-3 py-2 text-xs font-medium text-orange-700">
        {page.route.id}
      </div>
    {/if}

    <slot />

    <!-- Show actual (non-localized) error message in dev mode, default (localized) message otherwise.  -->
    {#if $errorState}
      <div class="absolute right-4 bottom-[calc(16px+var(--safe-area-inset-bottom))] w-[calc(100%-32px)]">
        <Toast
          variant="error"
          title={$appState?.dev_mode !== 'Off' ? 'Error' : $LL.ERROR.TITLE()}
          detail={$appState?.dev_mode !== 'Off' ? $errorState : $LL.ERROR.DEFAULT_MESSAGE()}
          ondismissed={() => {
            // After the toast fires the "dismissed" event, we reset $errorStore.
            errorState.set(undefined);
          }}
          autoDismissAfterMs={$appState?.dev_mode !== 'Off' ? 0 : 5_000}
        />
      </div>
    {/if}
  </div>
</div>

<style>
  /* Use CSS variables instead of `env(safe-area-inset-*)` to allow adding safe areas for desktop dev. */
  .safe-area-insets {
    padding-top: var(--safe-area-inset-top);
    padding-bottom: var(--safe-area-inset-bottom);
  }

  .safe-area-inset-top {
    height: var(--safe-area-inset-top);
  }

  .safe-area-inset-bottom {
    height: var(--safe-area-inset-bottom);
  }
</style>
