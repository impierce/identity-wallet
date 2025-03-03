<script lang="ts">
  import type { Component } from 'svelte';

  import type { SVGAttributes } from 'svelte/elements';

  import { TopNavBar } from '$lib/components';
  import { CaretRightBoldIcon, EnvelopeOpenFillIcon } from '$lib/icons';

  type Data = {
    title: string;
    description: string;
    icon: Component<SVGAttributes<SVGSVGElement>>;
    link: string;
  };

  const data: Data[] = [
    { title: 'Email', description: 'Verify your email address', icon: EnvelopeOpenFillIcon, link: '/me/add/email' },
  ];
</script>

<TopNavBar on:back={() => history.back()} title={'Add data'} class="sticky top-0 z-10" />

<div class="px-4 py-8">
  {#each data as { title, description, icon } (title)}
    <button class="flex w-full items-center justify-between rounded-xl bg-background-alt p-4">
      <div class="flex items-center space-x-4">
        <svelte:component this={icon} class="size-6 text-primary" />
        <div class="flex flex-col text-left">
          <p class="text-[14px]/[22px] font-medium text-slate-800 dark:text-grey">{title}</p>
          <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{description}</p>
        </div>
      </div>
      <CaretRightBoldIcon class="size-4 text-slate-500" />
    </button>
  {/each}
</div>
