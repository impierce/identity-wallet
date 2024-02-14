<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';

  import { attachConsole, error, info, trace } from '@tauri-apps/plugin-log';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { loadAllLocales } from '$src/i18n/i18n-util.sync';
  import { state } from '$src/stores';

  import ScrollText from '~icons/lucide/scroll-text';
  import ArrowLeft from '~icons/ph/arrow-left';
  import CaretDown from '~icons/ph/caret-down-bold';
  import CaretUp from '~icons/ph/caret-up-bold';
  import Trash from '~icons/ph/trash';
  import UserCircleGear from '~icons/ph/user-circle-gear';

  import '../app.css';

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

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('+layout.svelte: state', $state);

    // Set color scheme
    determineTheme(systemColorScheme.matches, $state?.active_profile?.theme);

    // User prompt
    let type = $state?.current_user_prompt?.type;

    if (type && type !== 'redirect') {
      goto(`/prompt/${type}`);
    }
  }

  const menuItemCss = 'flex flex-col content-start justify-center mr-4';
  const buttonCss =
    'flex content-center m-0 h-10 w-10 rounded-full bg-red-300 p-0 text-sm font-medium text-red-700 hover:outline-none hover:ring-2 hover:ring-red-700 hover:ring-opacity-60';
  const iconCss = 'm-auto block text-xl';

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
        <!-- Back button !-->
        <div class={menuItemCss}>
          <button class={buttonCss} on:click={() => history.back()}>
            <ArrowLeft class={iconCss} />
          </button>
        </div>

        <!-- Reset button !-->
        <div class={menuItemCss}>
          <button class={buttonCss} on:click={() => dispatch({ type: '[App] Reset' })}>
            <Trash class={iconCss} />
          </button>
        </div>

        <!-- Select Ferris profile !-->
        <div class={menuItemCss}>
          <button class={buttonCss} on:click={() => loadProfile('Ferris')}>
            <span class={iconCss}>🦀</span>
          </button>
        </div>

        <!-- Select Dragon profile !-->
        <div class={menuItemCss}>
          <button class={buttonCss} on:click={() => loadProfile('Dragon')}>
            <span class={iconCss}>🐲</span>
          </button>
        </div>

        <!-- Debug messages -->
        <div class={menuItemCss}>
          <button class={buttonCss} on:click={() => (showDebugMessages = !showDebugMessages)}>
            <ScrollText class={iconCss} />
          </button>
        </div>
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
    <div class="relative z-10 min-h-full w-screen bg-orange-100">
      <p class="p-4 text-center text-xs font-semibold uppercase text-orange-800">debug messages</p>
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
