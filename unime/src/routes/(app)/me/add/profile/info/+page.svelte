<script lang="ts">
  import { goto } from '$app/navigation';
  import LL from '$i18n/i18n-svelte';

  import { Accordion, Button, IconMessage, TopNavBar } from '$lib/components';
  import { IdentificationBadgeFillIcon } from '$lib/icons';

  const items = [
    {
      id: '0',
      title: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_0.TITLE(),
      description: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_0.DESCRIPTION(),
    },
    {
      id: '1',
      title: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_1.TITLE(),
      description: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_1.DESCRIPTION(),
    },
    {
      id: '2',
      title: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_2.TITLE(),
      description: $LL.ADD_CREDENTIALS.PROFILE.INFO.ITEM_2.DESCRIPTION(),
    },
  ];
</script>

<TopNavBar
  on:back={() => history.back()}
  title={$LL.ADD_CREDENTIALS.PROFILE.INFO.NAVBAR_TITLE()}
  class="sticky top-0 z-10"
/>

<div class="relative flex h-[calc(100%-50px)] flex-col">
  <div class="grow space-y-4 p-4">
    <div class="pt-8">
      <IconMessage icon={IdentificationBadgeFillIcon} title={$LL.ADD_CREDENTIALS.PROFILE.INFO.TITLE()} />
    </div>

    <p class="pb-4 text-center text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.ADD_CREDENTIALS.PROFILE.INFO.DESCRIPTION()}
    </p>

    <Accordion {items} />
  </div>

  <!-- TODO: REFACTOR! -->
  <div class="bg-background-alt absolute bottom-0 left-0 z-10 w-full rounded-t-3xl p-6">
    <Button label={$LL.CONTINUE()} on:click={() => goto('/me/add/profile')} />
  </div>
</div>
