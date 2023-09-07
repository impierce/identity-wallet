<script lang="ts">
  import { beforeNavigate, goto } from '$app/navigation';
  import { page } from '$app/stores';
  import { fly } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';

  import Button from '$src/lib/components/Button.svelte';

  import type { LayoutData } from './$types';

  export let data: LayoutData;

  // $: pathname = data.pathname;

  // let from2;
  // let to2;

  // beforeNavigate(({ from, to }) => {
  //   from2 = from?.route.id;
  //   to2 = to?.route.id;
  //   console.log({ from: from2, to: to2 });
  // });
</script>

{#key data.pathname}
  <!-- <div in:fly={{ x: 128, delay: 400, opacity: 1 }} out:fly={{ x: -128, opacity: 1 }}> -->
  <div class="content-height flex flex-col bg-bg-secondary dark:bg-bg-dark-secondary">
    <slot />
    <!-- <TopNavigation title="Terms & Conditions" on:back={() => history.back()} />
    <div class="grow">terms</div>
    <div class="space-y-[10px] rounded-t-3xl bg-white p-6">
      <Button label="Continue" on:click={() => goto('/welcome/terms')} disabled />
    </div> -->
  </div>
{/key}

<style>
  .content-height {
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
