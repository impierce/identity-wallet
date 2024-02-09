<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import QRCode from 'qrcode';
  import { fly } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import { getImageAsset } from '$lib/utils';
  import LL from '$src/i18n/i18n-svelte';
  import { colors } from '$src/lib/app/colors';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import ButtonRounded from '$src/lib/components/atoms/ButtonRounded.svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import CredentialDetailsDropdownMenu from '$src/lib/credentials/CredentialDetailsDropdownMenu.svelte';
  import { state } from '$src/stores';

  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
  import QrCode from '~icons/ph/qr-code';
  import User from '~icons/ph/user-light';

  let credential = $state.credentials.find((c) => $page.params.id === c.id)!!;

  let color = credential.metadata.display.color || colors.at(0);

  let icon: any = credential.metadata.display.icon || 'User';
  let title: string = credential.metadata.display.name || credential.data.type.at(-1);

  let qrcodeText = JSON.stringify(credential, null, 0);

  let isFavorite: boolean = credential.metadata.is_favorite;

  $: {
    const credential = $state.credentials.find((c) => $page.params.id === c.id)!!;
    // TODO: update icon, title, isFavorite when changes in store
    isFavorite = credential.metadata.is_favorite;
    title = credential.metadata.display.name || credential.data.type.at(-1);
    icon = credential.metadata.display.icon || 'User';
    color =
      credential.metadata.display.color ||
      colors.at(
        credential.id
          .match(/[0-9]+/)
          .at(0)
          .at(0) % 8, // TODO: omits last value (white)
      );
  }

  // create entries to be shown
  const { id, enrichment, ...entries } = credential.data.credentialSubject;
  // entries['issuer'] = credential.data.issuer ?? credential.issuer_name;
  // entries['issuanceDate'] = new Date(credential.data.issuanceDate).toLocaleString('en-US', {
  //   dateStyle: 'long',
  //   timeStyle: 'medium'
  // });

  console.log({ credential });

  let credentialLogoUrl: string | null;

  onMount(async () => {
    credentialLogoUrl = await getImageAsset($page.params.id!!);
  });
</script>

<div class="content-height relative flex w-full flex-col">
  <!-- Background (scaled, blurred, transparent) -->
  <div class="absolute left-0 z-10 {credentialLogoUrl ? 'top-0 h-[270px]' : 'top-[50px] h-[220px]'} w-screen">
    {#if !credentialLogoUrl}
      <div class="{color} relative h-[220px]"></div>
    {:else}
      <img
        src={credentialLogoUrl}
        class="scale-[1.75] opacity-40 blur-xl"
        on:error={() => (credentialLogoUrl = null)}
      />
    {/if}
  </div>
  <TopNavBar
    title={$LL.ME.CREDENTIAL.NAVBAR_TITLE()}
    on:back={() => history.back()}
    class={credentialLogoUrl ? '' : `${color} dark:${color} text-slate-800 dark:text-slate-800`}
  />
  <div class="hide-scrollbar grow overflow-y-scroll bg-silver px-[15px] dark:bg-navy">
    <!-- Header -->
    <!-- Background-->
    <!-- <div class="absolute left-0 top-[50px] h-[220px] w-screen {color}" /> -->
    <div class="relative z-10">
      <div class="flex flex-col px-2 py-[20px]">
        <!-- Logo -->
        <div class="flex items-start justify-between">
          <button
            class="-ml-1 -mt-1 rounded-full p-1"
            on:click={() =>
              dispatch({
                type: '[Credential Metadata] Update',
                payload: {
                  id: credential.id,
                  is_favorite: !isFavorite,
                },
              })}
          >
            {#if isFavorite}
              <HeartFill class="h-6 w-6 {credentialLogoUrl ? 'dark:text-white' : ''}" />
            {:else}
              <Heart class="h-6 w-6 {credentialLogoUrl ? 'dark:text-white' : ''}" />
            {/if}
          </button>
          <!-- TODO: remove border entirely? -->
          <div
            class="{color} mr-2 flex h-[75px] w-[75px] items-center justify-center overflow-hidden rounded-3xl border-[5px] border-white bg-white p-2"
          >
            <Image id={$page.params.id} iconClass={'h-6 w-6 dark:text-slate-800'} />
          </div>
          <div class="-mr-3 -mt-1">
            <CredentialDetailsDropdownMenu {credential} class={credentialLogoUrl ? 'dark:text-white' : ''} />
          </div>
          <!-- <button class="-mr-1 -mt-1 rounded-full p-1">
          <DotsThreeVertical class="h-6 w-6" />
        </button> -->
        </div>
        <!-- Text -->
        <div class="flex flex-col items-center pt-[15px]">
          <p
            class="line-clamp-2 text-center text-[22px]/[30px] font-semibold {credentialLogoUrl
              ? 'text-slate-700 dark:text-grey'
              : 'text-slate-700'}"
          >
            {title}
          </p>
          <p
            class="text-[13px]/[24px] font-normal {credentialLogoUrl
              ? 'text-slate-500 dark:text-slate-300'
              : 'text-slate-500'}"
          >
            {credential.issuer_name ?? credential.data.issuer}
          </p>
        </div>
      </div>
      <!-- TODO: Overlays the blurred background, because "overflow-hidden" on the background also crops off at the safe-area-top -->
      <div
        class="absolute left-0 top-[220px] -z-10 -ml-[15px] h-full w-screen overflow-hidden bg-silver dark:bg-navy"
      ></div>
      <!-- Table: Credential Subject -->
      <div
        class="divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:divide-slate-600 dark:border-slate-600 dark:bg-dark"
      >
        {#each Object.entries(entries) as entry}
          <div class="flex flex-col items-start px-4 py-[10px]">
            <p class="dark:text-300 text-[13px]/[24px] font-medium text-slate-500">{entry[0]}</p>
            <p class="break-words text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">
              {entry[1]}
            </p>
          </div>
        {/each}
      </div>
      {#if $state.dev_mode_enabled}
        <p class="pt-4 text-center text-[13px]/[24px] text-slate-500">{credential.data.issuer}</p>
      {/if}
    </div>
  </div>
  <BottomDrawer>
    <!-- TODO: feature disabled: "Share credential" -->
    <!-- <ButtonRounded slot="trigger" let:trigger {trigger} label="Share" icon={QrCode} class="absolute bottom-4 right-4" /> -->
    <span slot="content" class="flex flex-col items-center justify-center">
      <!-- Logo -->
      <div
        class="{color} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
      >
        <!-- <svelte:component this={icon} class="h-6 w-6 text-slate-800" /> -->
      </div>
      <!-- Description -->
      <div class="flex flex-col items-center">
        <p class="pt-4 text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] text-slate-500">
          {credential.data.issuer ?? credential.issuer_name}
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

<div class="safe-area-top {credentialLogoUrl ? 'bg-silver dark:bg-navy' : color}" />
<div class="safe-area-bottom z-10 bg-silver dark:bg-navy" />

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
