<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import QRCode from 'qrcode';
  import { fly } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';

  import Button from '$lib/components/Button.svelte';
  import { getImageAsset } from '$lib/connections/utils';
  import CredentialDetailsDropdownMenu from '$src/lib/components/CredentialDetailsDropdownMenu.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import { colors, icons } from '$src/lib/credentials/customization/utils';
  import ShareButton from '$src/lib/credentials/ShareButton.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
  import SealCheck from '~icons/ph/seal-check';
  import User from '~icons/ph/user-light';

  let credential = $state.credentials.find((c) => $page.params.id === c.id)!!;

  //   let color = credential.metadata.display.color || colors.at(0);

  let icon: any = credential.metadata.display.icon || 'User';
  let title: string = credential.metadata.display.name || credential.data.type.at(-1);

  let credentialLogoUrl: string;
  let issuerLogoUrl: string;

  let qrcodeText = JSON.stringify(credential, null, 0);

  let isFavorite: boolean = credential.metadata.is_favorite;

  $: {
    const credential = $state.credentials.find((c) => $page.params.id === c.id)!!;
    // TODO: update icon, title, isFavorite when changes in store
    isFavorite = credential.metadata.is_favorite;
    title = credential.metadata.display.name || credential.data.type.at(-1);
    icon = credential.metadata.display.icon || 'User';
    // color =
    //   credential.metadata.display.color ||
    //   colors.at(
    //     credential.id
    //       .match(/[0-9]+/)
    //       .at(0)
    //       .at(0) % 8, // TODO: omits last value (white)
    //   );
  }

  // create entries to be shown
  const { id, enrichment, ...entries } = credential.data.credentialSubject;
  // entries['issuer'] = credential.data.issuer ?? credential.issuer_name;
  // entries['issuanceDate'] = new Date(credential.data.issuanceDate).toLocaleString('en-US', {
  //   dateStyle: 'long',
  //   timeStyle: 'medium'
  // });

  console.log({ credential });

  onMount(async () => {
    credentialLogoUrl = await getImageAsset($page.params.id!!);
    issuerLogoUrl = await getImageAsset('university');
  });
</script>

<div class="content-height relative flex w-full flex-col">
  <!-- TODO: allow overriding the color of the TopNavigation -->
  <TopNavigation
    title="Badge information"
    on:back={() => history.back()}
    class="bg-white text-slate-800 dark:bg-dark dark:text-grey"
  />
  <div class="hide-scrollbar grow overflow-y-scroll bg-white dark:bg-dark">
    <!-- Logo -->
    <div class="relative flex flex-col overflow-hidden bg-silver px-[15px] py-[20px] dark:bg-navy">
      <img src={credentialLogoUrl} class="absolute -top-1/4 left-0 scale-[1.75] opacity-40 blur-xl" />
      <div class="z-10 flex items-start justify-between px-2">
        <button
          class="-ml-1 -mt-1 rounded-full p-1"
          on:click={() =>
            dispatch({
              type: '[Credential Metadata] Update',
              payload: { id: credential.id, is_favorite: !isFavorite },
            })}
        >
          {#if isFavorite}
            <HeartFill class="h-6 w-6 dark:text-white" />
          {:else}
            <Heart class="h-6 w-6 dark:text-white" />
          {/if}
        </button>
        <div
          class="mr-2 flex h-[180px] w-[180px] flex-col items-center justify-center rounded-3xl bg-white dark:bg-dark"
        >
          <!-- Icon -->
          <!-- <svelte:component this={icons[icon]} class="h-[128px] w-[128px] text-slate-800" /> -->
          <img src={credentialLogoUrl} class="h-[128px] w-[128px] overflow-hidden rounded-xl object-cover" alt="logo" />
          <!-- Logo -->
          <!-- <div class="flex h-full w-full items-center justify-center bg-white p-1">
              <img src={logo_location} alt="logo" class="object-scale-down" />
            </div> -->
        </div>
        <div class="-mr-3 -mt-1">
          <CredentialDetailsDropdownMenu {credential} class="dark:text-white" />
        </div>
        <!-- <button class="-mr-1 -mt-1 rounded-full p-1">
            <DotsThreeVertical class="h-6 w-6" />
          </button> -->
      </div>
      <!-- Text -->
      <div class="z-10 flex flex-col items-center pt-[15px]">
        <p class="text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
          {credential.issuer_name ?? credential.data.issuer}
        </p>
        <p class="text-[22px]/[30px] font-semibold tracking-tight text-black dark:text-white">{title}</p>
      </div>
    </div>
    <!-- Text -->
    <div class="flex flex-col space-y-5 px-[15px]">
      <!-- Valid, Issued By -->
      <div class="flex space-x-3 pt-8">
        <!-- Valid -->
        <div class="flex w-full flex-col items-center space-y-1">
          <p class="text-xs text-black dark:text-white">Valid</p>
          <div class="flex w-full justify-center rounded-xl bg-silver py-5 dark:bg-navy">
            <SealCheck class="h-7 w-7 text-slate-800 dark:text-grey" />
          </div>
          <p class="text-xs text-black dark:text-white">03.11.2010</p>
        </div>
        <!-- Issued By -->
        <div class="flex w-full flex-col items-center space-y-1">
          <p class="text-xs text-black dark:text-white">Issued By</p>
          <div class="flex w-full justify-center rounded-xl bg-silver py-5 dark:bg-navy">
            <img src={issuerLogoUrl} class="h-7 w-7 rounded-md" alt="issuer" />
          </div>
          <p class="text-xs text-black dark:text-white">SSSC</p>
        </div>
      </div>

      <!-- Description -->
      <div>
        <p class="text-lg font-semibold text-black dark:text-white">Description</p>
        <p class="text-[13px]/[24px] text-slate-800 dark:text-grey">
          You can earn this badge by watching our 10 short video tutorials about getting started with badges. The
          tutorials show you how to create your SSSC Open Badges account, find badges and submit evidence for them.
        </p>
      </div>

      <!-- Metadata (Table: Credential Subject) -->
      <div>
        <p class="pb-2 text-lg font-semibold text-black dark:text-white">Metadata</p>
        <div
          class="divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:divide-slate-600 dark:border-slate-600 dark:bg-dark"
        >
          {#each Object.entries(entries) as entry}
            <div class="flex flex-col items-start px-4 py-[10px]">
              <p class="text-[13px]/[24px] font-medium text-slate-500">{entry[0]}</p>
              <p class="break-words text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
                {entry[1]}
              </p>
            </div>
          {/each}
        </div>
      </div>
    </div>

    {#if $state.dev_mode_enabled}
      <p class="py-5 text-center text-[13px]/[24px] text-slate-500">{credential.data.issuer}</p>
    {/if}
  </div>
  <!-- </div> -->
  <BottomDrawer>
    <!-- TODO: feature disabled: "Share credential" -->
    <!-- <ShareButton slot="trigger" let:trigger {trigger} /> -->
    <span slot="content" class="flex flex-col items-center justify-center">
      <!-- Logo -->
      <div class="flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white">
        <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
      </div>
      <!-- Description -->
      <div class="flex flex-col items-center">
        <p class="pt-4 text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] text-slate-500">
          {credential.issuer_name ?? credential.data.issuer}
        </p>
      </div>
      <!-- QR Code -->
      <div class="flex flex-col items-center p-7">
        <div class="rounded-2xl bg-white p-6">
          {#await QRCode.toDataURL(qrcodeText, { margin: 0, color: { light: '#FFFFFF' } }) then data_url}
            <img src={data_url} alt="qr-code" />
          {/await}
        </div>
        <p class="pt-5 text-xl font-semibold text-black">
          {Object.entries(credential.data.credentialSubject).at(-1).at(1)}
        </p>
      </div>
    </span>
    <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700">Close</button
    >
  </BottomDrawer>
</div>

<div class="safe-area-top bg-white dark:bg-dark" />

<!-- <div class="safe-area-bottom z-10 bg-white dark:bg-dark" /> -->

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
