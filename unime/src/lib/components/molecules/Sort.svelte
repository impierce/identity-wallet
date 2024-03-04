<script lang="ts">
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import Slider from '~icons/ph/sliders-horizontal';
  import BottomDrawer from './dialogs/BottomDrawer.svelte';
  import { melt } from '@melt-ui/svelte';
</script>

<!--
<div class="sort_wrapper">
  <button class=" flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100">
    <Slider class="m-2.5 h-5 w-5"></Slider>
  </button>
</div>
-->


<div class="sort_wrapper">
  <BottomDrawer titleText='Sorting' >
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class=" flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100"
      ><Slider/></button
    >
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </BottomDrawer>
</div>