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

  let color: string = 'bg-orange-100';
  let icon: any = House;
  let title: string = `Driver's License`;

  let isFavorite: boolean = false;
</script>

<div class="content-height flex w-full flex-col" in:fly={{ x: 24 }}>
  <TopNavigation title="Credential info" on:back={() => history.back()} />
  <div class="grow px-[15px]">
    <!-- Header -->
    <!-- Background-->
    <div
      class="absolute left-0 top-[50px] -z-10 h-[250px] w-screen bg-gradient-to-b from-orange-100 to-white"
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
          class={`${color} flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white`}
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
        <p class="text-[13px]/[24px] font-normal text-slate-500">{$page.params.id ?? ''}</p>
      </div>
    </div>
    <!-- Data -->
    <div class="divide-y divide-solid divide-gray-200 rounded-xl border border-gray-200 bg-white">
      <!-- TODO: loop -->
      <div class="flex flex-col items-start px-4 py-[10px]">
        <p class="text-[15px]/[24px] font-medium text-gray-600">First name</p>
        <p class="text-[13px]/[24px] font-medium text-slate-800">Tony</p>
      </div>
      <div class="flex flex-col items-start px-4 py-[10px]">
        <p class="text-[15px]/[24px] font-medium text-gray-600">Last name</p>
        <p class="text-[13px]/[24px] font-medium text-slate-800">Stark</p>
      </div>
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
