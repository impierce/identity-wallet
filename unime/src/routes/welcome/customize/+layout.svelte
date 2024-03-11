<script lang="ts">
  import { goto } from '$app/navigation';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { onboarding_state } from '$src/stores';

  function skipCustomization() {
    onboarding_state.set({ ...$onboarding_state, picture: '&#x1F642' });
    goto('/welcome/password');
  }
</script>

<TopNavBar on:back={() => history.back()} title={$LL.ONBOARDING.CUSTOMIZE.NAVBAR_TITLE()}>
  <!-- TODO: replace BottomDrawer with AlertDialog -->
  <BottomDrawer
    titleText={$LL.ONBOARDING.CUSTOMIZE.SKIP.TITLE()}
    descriptionText={$LL.ONBOARDING.CUSTOMIZE.SKIP.TEXT()}
  >
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary">{$LL.SKIP()}</button
    >
    <div slot="content" class="w-full pb-[10px] pt-[20px]">
      <Button label={$LL.ONBOARDING.CUSTOMIZE.SKIP.CONFIRM()} on:click={skipCustomization} />
    </div>
    <Button variant="secondary" slot="close" let:close trigger={close} label={$LL.ONBOARDING.CUSTOMIZE.SKIP.ABORT()} />
  </BottomDrawer>
</TopNavBar>
<slot />
