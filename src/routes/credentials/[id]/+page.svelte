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
  import { state } from '$src/stores';

  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
  import House from '~icons/ph/house-light';

  let credential = $state.credentials.find((c) => $page.params.id === c.at(0))?.at(1);

  let color = {
    bg: 'bg-indigo-100',
    gradient: 'from-indigo-100'
  };

  let icon: any = House;
  let title: string = credential.type.at(-1);

  let qrcodeText = JSON.stringify(credential, null, 0);

  let isFavorite: boolean = false;
</script>

<div class="content-height relative flex w-full flex-col" in:fly={{ x: 24 }}>
  <TopNavigation title="Credential info" on:back={() => history.back()} />
  <div class="hide-scrollbar grow overflow-y-scroll px-[15px]">
    <!-- Header -->
    <!-- Background-->
    <div
      class="absolute left-0 top-[50px] -z-10 h-[250px] w-screen bg-gradient-to-b {color.gradient} to-white"
    />
    <div class="flex flex-col py-[20px]">
      <!-- Logo -->
      <div class="flex items-start justify-between">
        <button class="-ml-1 -mt-1 rounded-full p-1" on:click={() => (isFavorite = !isFavorite)}>
          {#if isFavorite}
            <HeartFill class="h-6 w-6" />
          {:else}
            <Heart class="h-6 w-6" />
          {/if}
        </button>
        <div
          class="{color.bg} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
        >
          <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
        </div>
        <div class="-mr-1 -mt-1">
          <CredentialDetailsDropdownMenu />
        </div>
        <!-- <button class="-mr-1 -mt-1 rounded-full p-1">
          <DotsThreeVertical class="h-6 w-6" />
        </button> -->
      </div>
      <!-- Text -->
      <div class="flex flex-col items-center pt-[15px]">
        <p class="text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] font-normal text-slate-500">
          {new URL(credential.issuer).hostname}
        </p>
      </div>
    </div>
    <!-- Table: Credential Subject -->
    <div class="divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white">
      {#each Object.entries(credential.credentialSubject) as entry}
        <div class="flex flex-col items-start px-4 py-[10px]">
          <p class="text-[15px]/[24px] font-medium text-[#6E82A4]">{entry[0]}</p>
          <p class="break-all text-[13px]/[24px] font-medium text-slate-800">{entry[1]}</p>
        </div>
      {/each}
    </div>
    <!-- Table: Issuer -->
    <div
      class="mt-[15px] divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white"
    >
      {#each Object.entries( { issuer: credential.issuer, issuanceDate: credential.issuanceDate } ) as entry}
        <div class="flex flex-col items-start px-4 py-[10px]">
          <p class="text-[15px]/[24px] font-medium text-[#6E82A4]">{entry[0]}</p>
          <p class="break-all text-[13px]/[24px] font-medium text-slate-800">{entry[1]}</p>
        </div>
      {/each}
    </div>
    <!-- ID (unime internal) -->
    <div class="p-[15px] pb-0 text-center text-xs text-slate-500">
      <pre>{$page.params.id}</pre>
    </div>
  </div>
  <BottomDrawer>
    <!-- TODO: Share functionality currently disabled until decided how sharing works -->
    <ShareButton slot="trigger" let:trigger {trigger} />
    <span slot="content" class="flex flex-col items-center justify-center">
      <!-- Logo -->
      <div
        class="{color.bg} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
      >
        <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
      </div>
      <!-- Description -->
      <div class="flex flex-col items-center">
        <p class="pt-4 text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] text-slate-500">{credential.issuer}</p>
      </div>
      <!-- QR Code -->
      <div class="flex flex-col items-center p-7">
        <div class="rounded-2xl bg-white p-6">
          {#await QRCode.toDataURL( qrcodeText, { margin: 0, color: { light: '#FFFFFF' } } ) then data_url}
            <img src={data_url} alt="qr-code" />
          {/await}
        </div>
        <p class="pt-5 text-xl font-semibold text-black">
          {Object.entries(credential.credentialSubject).at(-1)?.at(1)}
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
