<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Accordion, Button, IconMessage, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { EnvelopeOpenFillIcon } from '$lib/icons';
  import { state } from '$lib/stores';

  const items = [
    {
      id: '0',
      title: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_0.TITLE(),
      description: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_0.DESCRIPTION(),
    },
    {
      id: '1',
      title: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_1.TITLE(),
      description: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_1.DESCRIPTION(),
    },
    {
      id: '2',
      title: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_2.TITLE(),
      description: $LL.ADD_CREDENTIALS.EMAIL.INFO.ITEM_2.DESCRIPTION(),
    },
  ];

  onMount(async () => {
    await dispatch({
      type: '[Verified Data] Check service health',
      payload: { service: 'email-verification-service' },
    });
    // If there is already an active verification session ongoing, skip this info page.
    if ($state.verified_data.email_verification) {
      goto('/me/add/email');
    }
  });
</script>

<TopNavBar
  on:back={() => history.back()}
  title={$LL.ADD_CREDENTIALS.EMAIL.INFO.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<div class="relative flex h-[calc(100%-50px)] flex-col">
  <div class="grow space-y-4 p-4">
    <div class="pt-8">
      <IconMessage icon={EnvelopeOpenFillIcon} title={$LL.ADD_CREDENTIALS.EMAIL.INFO.TITLE()} />
    </div>

    <p class="pb-4 text-center text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ADD_CREDENTIALS.EMAIL.INFO.DESCRIPTION()}
    </p>

    <Accordion {items} />
  </div>

  <!-- TODO: REFACTOR! -->
  <div class="absolute bottom-0 left-0 z-10 w-full rounded-t-3xl bg-background-alt p-6">
    <Button label={$LL.CONTINUE()} on:click={() => goto('/me/add/email')} />
  </div>
</div>
