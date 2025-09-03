<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { Toast, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { HeartFillIcon } from '$lib/icons';
  import UniMeLogo from '$lib/static/svg/logo/UniMeLogo.svelte';
  import { state as appState } from '$lib/stores';

  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  // In order to show the developer mode in the app settings,
  // the user has to tap the UniMe logo 7 times in a row.
  const REQUIRED_CLICKS = 7;

  let counter = $state(0);

  function handleClick() {
    counter++;
    if (counter === REQUIRED_CLICKS) {
      dispatch({ type: '[DEV] Show DEV mode setting', payload: { show: !$appState.show_dev_mode_setting } });
      counter = 0;
    }
  }

  let title = $appState.show_dev_mode_setting ? 'Hide Developer Mode' : 'Unlock Developer Mode';
  let detail = $derived(
    `Tap the logo ${REQUIRED_CLICKS - counter} more time${REQUIRED_CLICKS - counter > 1 ? 's' : ''} to ${$appState.show_dev_mode_setting ? 'hide' : 'unlock'}.`,
  );

  let showMessage = $derived(() => {
    if (counter >= 5 && counter < REQUIRED_CLICKS) {
      return true;
    } else {
      return false;
    }
  });
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.SUPPORT.ABOUT.NAVBAR_TITLE()} class="sticky top-0 z-10" />

<div class="flex flex-col bg-silver dark:bg-navy">
  <h1 class="sr-only">{$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}</h1>
  <div class="flex w-full scale-75 justify-center">
    <button onclick={handleClick}>
      <UniMeLogo class="text-blue dark:text-silver" />
    </button>
  </div>
  <div
    class="flex flex-col items-center gap-6 pt-4 text-[13px]/[24px] font-normal text-slate-500 opacity-50 dark:text-slate-300"
  >
    {#if $appState.dev_mode !== 'Off'}
      <section class="flex flex-col items-center">
        <h2 class="mb-3 font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.SPECIFICATIONS()}</h2>
        <dl class="flex flex-col items-center gap-3">
          {#each data.specifications as spec (spec.id)}
            <div class="flex flex-col items-center">
              <dt class="font-semibold">{`${spec.description} (${spec.id}):`}</dt>
              <!-- `target="_blank"` opens the spec link in the default browser. -->
              <dd><a href={spec.url} target="_blank" class="underline">{spec.version}</a></dd>
            </div>
          {/each}
        </dl>
      </section>
    {/if}
    <section class="mb-4 flex flex-col items-center">
      <h2 class="font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.VERSION()}</h2>
      <div class="mb-3">0.10.0</div>
      <div class="flex items-center">
        <p>{$LL.SETTINGS.SUPPORT.ABOUT.BUILT_WITH()}</p>
        <HeartFillIcon class="pl-1" />
      </div>
    </section>
    <section class="mb-4 flex flex-col items-center">
      <h2 class="font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.LICENSE()}</h2>
      <div class="mb-3">Apache License 2.0</div>
      <div>{`${new Date().getFullYear()} Impierce Technologies B.V.`}</div>
    </section>
  </div>
</div>

{#if showMessage()}
  <div class="absolute bottom-[calc(64px_+_16px_+_var(--safe-area-inset-bottom))] left-4 w-[calc(100%_-_32px)]">
    <Toast variant="info" {title} {detail} dismissible={false} />
  </div>
{/if}
