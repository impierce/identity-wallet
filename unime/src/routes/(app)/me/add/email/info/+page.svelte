<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Accordion, Button, IconMessage, TopNavBar } from '$lib/components';
  import { EnvelopeOpenFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  const items = [
    {
      id: '0',
      title: 'What can I do with my verified email?',
      description:
        'You can prove ownership of that email address to websites, shops and other people. It also allows you to log in to websites that support it without needing to remember a password or setting up a passkey.',
    },
    {
      id: '1',
      title: 'How is it verified?',
      description:
        'UniMe sends a message containing your email to one of our verification systems. That system then creates a one-time code and sends an email to your inbox containing that code. When you enter that code in UniMe, you prove you have access to that email inbox. UniMe now sends another message to our verification system, redeeming the code for a verifiable credential.',
    },
    {
      id: '2',
      title: 'Is my personal information safe?',
      description:
        'We treat any data you enter in UniMe with the greatest respect. All messages from UniMe to our servers are fully encrypted when they travel through the internet. They are also encrypted on our servers using the latest security standards. In order to successfully deliver an email to your inbox, we initially have to receive it in plain text, so a full "zero-access encryption" is unfortunately not applicable to this use case.',
    },
  ];

  onMount(() => {
    // If there is already an active verification session ongoing, skip this info page.
    if ($state.verified_data.email_verification) {
      goto('/me/add/email');
    }
  });
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
