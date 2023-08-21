<script lang="ts">
  import Button from '$lib/components/Button.svelte';
  import { TopNavigation } from '@impierce/ui-components';
  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
  import House from '~icons/ph/house-light';
  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  import { goto } from '$app/navigation';
  import { fly } from 'svelte/transition';
  import { page } from '$app/stores';
  import { state } from '$src/stores';

  let credential = $state.credentials.find((c) => $page.params.id === c.at(0))?.at(1);

  let color: string = 'indigo-100';
  let icon: any = House;
  let title: string = credential.type.at(-1);

  let isFavorite: boolean = false;
</script>

<div class="content-height flex w-full flex-col" in:fly={{ x: 24 }}>
  <TopNavigation title="Credential info" on:back={() => history.back()} />
  <div class="hide-scrollbar grow overflow-y-scroll px-[15px]">
    <!-- Header -->
    <!-- Background-->
    <div
      class="absolute left-0 top-[50px] -z-10 h-[250px] w-screen bg-gradient-to-b from-{color} to-white"
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
          class="bg-{color} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white"
        >
          <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
        </div>
        <button class="-mr-1 -mt-1 rounded-full p-1">
          <DotsThreeVertical class="h-6 w-6" />
        </button>
      </div>
      <!-- Text -->
      <div class="flex flex-col items-center pt-[15px]">
        <p class="text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] font-normal text-slate-500">{credential.issuer}</p>
      </div>
    </div>
    <!-- Credential Subject -->
    <div class="divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white">
      {#each Object.entries(credential.credentialSubject) as entry}
        <div class="flex flex-col items-start px-4 py-[10px]">
          <p class="text-[15px]/[24px] font-medium text-gray-600">{entry[0]}</p>
          <p class="break-all text-[13px]/[24px] font-medium text-slate-800">{entry[1]}</p>
        </div>
      {/each}
    </div>
    <!-- Issuer -->
    <div
      class="mt-[15px] divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white"
    >
      {#each Object.entries( { issuer: credential.issuer, issuanceDate: credential.issuanceDate } ) as entry}
        <div class="flex flex-col items-start px-4 py-[10px]">
          <p class="text-[15px]/[24px] font-medium text-gray-600">{entry[0]}</p>
          <p class="break-all text-[13px]/[24px] font-medium text-slate-800">{entry[1]}</p>
        </div>
      {/each}
    </div>
    <!-- ID (unime internal) -->
    <div class="p-[15px] pb-0 text-center text-xs text-slate-500">
      <pre>{$page.params.id}</pre>
    </div>
  </div>
  <div class="flex p-6"><Button label="Share" disabled /></div>
</div>

<div class="safe-area-top" />
<div class="safe-area-bottom" />

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
