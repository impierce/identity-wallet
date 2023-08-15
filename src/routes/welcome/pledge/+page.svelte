<script>
  import { beforeNavigate, goto } from '$app/navigation';
  import Button from '$src/lib/components/Button.svelte';
  import { TopNavigation } from '@impierce/ui-components';
  import { onMount } from 'svelte';
  import { fade, fly } from 'svelte/transition';
  import { page } from '$app/stores';

  $: {
    console.log('pledge: $page.data', $page.data);
  }

  beforeNavigate(({ from, to }) => {
    console.log('pledge: coming from', from, 'going to', to);
  });
</script>

<!-- <div in:fly={{ x: 128, delay: 400 }} out:fly={{ x: -128, opacity: 1 }}>
  <div class="content-height flex flex-col bg-neutral-100">
    <TopNavigation title="UniMe pledge" on:back={() => history.back()} />
    <div class="grow">pledge</div>
    <div class="space-y-[10px] rounded-t-3xl bg-white p-6">
      <Button label="Continue" on:click={() => goto('/welcome/terms')} />
    </div>
  </div>
</div> -->
<div in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <!-- <div in:fly={{ y: -50, delay: 300 }}> -->
  <TopNavigation title="UniMe Pledge" on:back={() => history.back()} />
</div>
<div class="mt-12 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <!-- <div class="mt-12 grow p-4" in:fly={{ x: 300, delay: 300 }} out:fly={{ x: -300, duration: 300 }}> -->
  <div class="py-[25px]">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      No funny <span class="text-indigo-500">business</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">Here's our pledge to you.</p>
  </div>
  <div class="mt-4 border">Accordion</div>
</div>
<div class="space-y-[10px] rounded-t-3xl bg-white p-6" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <!-- <div
  class="space-y-[10px] rounded-t-3xl bg-white p-6"
  in:fly={{ y: 96, delay: 300, opacity: 1 }}
  out:fly={{ y: 96, duration: 300, opacity: 1 }}
> -->
  <Button label="Continue" on:click={() => goto('/welcome/terms')} />
</div>

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
