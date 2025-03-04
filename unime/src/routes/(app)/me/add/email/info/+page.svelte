<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Accordion, Button, IconMessage, TopNavBar } from '$lib/components';
  import { EnvelopeOpenFillIcon } from '$lib/icons';

  const items = [
    { id: '0', title: 'What can I do with my verified email?', description: 'Present ownership over that email.' },
    { id: '1', title: 'How is it verified?', description: 'Code inside email.' },
    { id: '2', title: 'Is my personal information safe?', description: 'Yes.' },
  ];
</script>

<TopNavBar on:back={() => history.back()} title={'Verified email'} class="sticky top-0 z-10" />
<div class="flex h-[calc(100vh-48px-64px)] flex-col">
  <div class="grow space-y-4 p-4">
    <div class="pt-8">
      <IconMessage icon={EnvelopeOpenFillIcon} title="Verified email" />
    </div>

    <p class="pb-4 text-center text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Before you start</p>

    <Accordion {items} />
  </div>

  <!-- TODO: REFACTOR! -->
  <div class="absolute bottom-[64px] left-0 z-10 w-full rounded-t-3xl bg-white p-6 dark:bg-dark">
    <Button label={$LL.CONTINUE()} on:click={() => goto('/me/add/email')} />
  </div>
</div>
