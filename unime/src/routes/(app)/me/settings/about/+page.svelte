<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { TopNavBar } from '$lib/components';
  import { HeartFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  import type { PageData } from './$types';

  export let data: PageData;
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.SUPPORT.ABOUT.NAVBAR_TITLE()} />

<div class="content-height flex flex-col bg-silver dark:bg-navy">
  <h1 class="sr-only">{$LL.SETTINGS.SUPPORT.ABOUT.TITLE()}</h1>
  <div
    class="flex flex-col items-center gap-6 pt-4 text-[13px]/[24px] font-normal text-slate-500 opacity-50 dark:text-slate-300"
  >
    {#if $state.dev_mode !== 'Off'}
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
    <section class="flex flex-col items-center">
      <h2 class="mb-3 font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.VERSION()}</h2>
      <div>0.6.8</div>
      <div class="flex items-center pb-4">
        <p>{$LL.SETTINGS.SUPPORT.ABOUT.BUILT_WITH()}</p>
        <HeartFillIcon class="pl-1" />
      </div>
    </section>
    <section class="flex flex-col items-center">
      <h2 class="mb-3 font-bold">{$LL.SETTINGS.SUPPORT.ABOUT.LICENSE()}</h2>
      <div>Apache License 2.0</div>
      <div>{`${new Date().getFullYear()} Impierce Technologies`}</div>
    </section>
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px, top-navigation: 50px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom) - 64px - 50px);
  }
</style>
