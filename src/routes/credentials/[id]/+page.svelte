<script lang="ts">
  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import QRCode from 'qrcode';
  import { fly } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';

  import Button from '$lib/components/Button.svelte';
  import CredentialDetailsDropdownMenu from '$src/lib/components/CredentialDetailsDropdownMenu.svelte';
  import ShareButton from '$src/lib/credentials/ShareButton.svelte';
  import { colors, icons } from '$src/lib/credentials/customization/utils';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
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
          .at(0) % 8 // TODO: omits last value (white)
      );
  }

  // create entries to be shown
  const { id, ...entries } = credential.data.credentialSubject;
  entries['issuer'] = credential.data.issuer;
  entries['issuanceDate'] = new Date(credential.data.issuanceDate).toLocaleString('en-US', {
    dateStyle: 'long',
    timeStyle: 'medium'
  });
</script>

<div class="content-height relative flex w-full flex-col" in:fly={{ x: 24 }}>
  <TopNavigation title="Credential info" on:back={() => history.back()} />
  <div class="hide-scrollbar grow overflow-y-scroll bg-silver px-[15px] dark:bg-navy">
    <!-- Header -->
    <!-- Background-->
    <div class="absolute left-0 top-[50px] h-[220px] w-screen {color}" />
    <div class="relative z-10">
      <div class="flex flex-col py-[20px]">
        <!-- Logo -->
        <div class="flex items-start justify-between">
          <button
            class="-ml-1 -mt-1 rounded-full p-1"
            on:click={() =>
              dispatch({
                type: '[Credential Metadata] Update',
                payload: { id: credential.id, is_favorite: !isFavorite }
              })}
          >
            {#if isFavorite}
              <HeartFill class="h-6 w-6" />
            {:else}
              <Heart class="h-6 w-6" />
            {/if}
          </button>
          <div
            class="{color} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
          >
            <!-- Icon -->
            <svelte:component this={icons[icon]} class="h-6 w-6 text-slate-800" />
            <!-- Logo -->
            <!-- <div class="flex h-full w-full items-center justify-center bg-white p-1">
            <img src={logo_location} alt="logo" class="object-scale-down" />
          </div> -->
          </div>
          <div class="-mr-1 -mt-1">
            <CredentialDetailsDropdownMenu {credential} />
          </div>
          <!-- <button class="-mr-1 -mt-1 rounded-full p-1">
          <DotsThreeVertical class="h-6 w-6" />
        </button> -->
        </div>
        <!-- Text -->
        <div class="flex flex-col items-center pt-[15px]">
          <p class="text-[22px]/[30px] font-semibold text-slate-700">{title}</p>
          <p class="text-[13px]/[24px] font-normal text-slate-500">
            {credential.data.issuer}
          </p>
        </div>
      </div>
      <!-- Table: Credential Subject -->
      <div
        class="divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:divide-slate-600 dark:border-slate-600 dark:bg-dark"
      >
        {#each Object.entries(entries) as entry}
          <div class="flex flex-col items-start px-4 py-[10px]">
            <p class="text-[13px]/[24px] font-medium text-slate-500">{entry[0]}</p>
            <p class="break-all text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
              {entry[1]}
            </p>
          </div>
        {/each}
      </div>
    </div>
  </div>
  <BottomDrawer>
    <!-- TODO: feature disabled: "Share credential" -->
    <!-- <ShareButton slot="trigger" let:trigger {trigger} /> -->
    <span slot="content" class="flex flex-col items-center justify-center">
      <!-- Logo -->
      <div
        class="{color} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
      >
        <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
      </div>
      <!-- Description -->
      <div class="flex flex-col items-center">
        <p class="pt-4 text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] text-slate-500">{credential.data.issuer}</p>
      </div>
      <!-- QR Code -->
      <div class="flex flex-col items-center p-7">
        <div class="rounded-2xl bg-white p-6">
          {#await QRCode.toDataURL( qrcodeText, { margin: 0, color: { light: '#FFFFFF' } } ) then data_url}
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

<div class="safe-area-top" />
<div class="safe-area-bottom" />

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
