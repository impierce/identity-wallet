<script lang="ts">
  import type { Component } from 'svelte';

  import { goto } from '$app/navigation';
  import type { SVGAttributes } from 'svelte/elements';

  import { TopNavBar } from '$lib/components';
  import { CaretRightBoldIcon, EnvelopeOpenFillIcon, PhoneFillIcon } from '$lib/icons';

  type Data = {
    title: string;
    description: string;
    icon: Component<SVGAttributes<SVGSVGElement>>;
    link: string;
    disabled?: boolean;
  };

  const data: Data[] = [
    {
      title: 'Email',
      description: 'Get your email address verified',
      icon: EnvelopeOpenFillIcon,
      link: '/me/add/email/info',
    },
    {
      title: 'Phone number',
      description: 'Get your phone number verified',
      icon: PhoneFillIcon,
      link: '/me/add',
      disabled: true,
    },
  ];
</script>

<TopNavBar on:back={() => history.back()} title={'Add data'} class="sticky top-0 z-10" />

<div class="flex flex-col space-y-4 px-4 py-8">
  {#each data as { title, description, icon, link, disabled } (title)}
    <button
      class="flex w-full items-center justify-between rounded-xl bg-background-alt p-4 disabled:opacity-50"
      onclick={() => goto(link)}
      {disabled}
    >
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
