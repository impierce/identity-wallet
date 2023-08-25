<script>
  import { dispatch } from '$src/lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { melt } from '@melt-ui/svelte';
</script>

<TopNavigation on:back={() => history.back()} title="Customization">
  <!-- TODO: replace BottomDrawer with AlertDialog -->
  <BottomDrawer titleText={$LL.SETUP.SKIP_TITLE()} descriptionText={$LL.SETUP.SKIP_TEXT()}>
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary">{$LL.SKIP()}</button
    >
    <!-- <button
        slot="trigger"
        let:trigger
        use:melt={trigger}
        class="-mr-2 p-2 text-left text-[13px]/[24px] font-medium text-primary"
      >
        <div class="h-6 w-6 bg-slate-200" />
      </button> -->
    <button
      slot="content"
      class="w-full rounded-lg bg-red-100 px-4 py-2 text-red-600"
      on:click={() => dispatch({ type: '[User Journey] Cancel' })}>Yes</button
    >
    <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      >No, let's continue</button
    >
  </BottomDrawer>
</TopNavigation>
<slot />
