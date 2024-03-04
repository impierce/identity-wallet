<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';

  import Issued from '~icons/ph/calendar-check';
  import Added from '~icons/ph/calendar-plus';
  import List from '~icons/ph/list-dashes';
  import Slider from '~icons/ph/sliders-horizontal';
  import Ascending from '~icons/ph/sort-ascending';
  import Grid from '~icons/ph/squares-four';

  import BottomDrawer from './dialogs/BottomDrawer.svelte';
</script>

<!--
<div class="sort_wrapper">
  <button class=" flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100">
    <Slider class="m-2.5 h-5 w-5"></Slider>
  </button>
</div>
-->

<div class="sort_wrapper">
  <BottomDrawer titleText="Sorting">
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100"
      ><Slider /></button
    >
    <div slot="content" class="w-full">
      <ul class="relative pb-5">
        <li class="flex gap-2 py-3 dark:text-gray-100">
          <List />
          <p class="text-sm font-medium">List view</p>
        </li>
        <li class="flex gap-2 py-3 dark:text-gray-100">
          <Grid />
          <p class="text-sm font-medium">Grid view</p>
        </li>

        <hr class="full-width fill-[#EFEFEF]" />

        <li class="flex gap-2 py-3 dark:text-gray-100">
          <Ascending />
          <p class="text-sm font-medium">Alphabetical</p>
        </li>

        <li class="flex gap-2 py-3 dark:text-gray-100">
          <Issued />
          <p class="text-sm font-medium">Date issued</p>
        </li>

        <li class="flex gap-2 py-3 dark:text-gray-100">
          <Added />
          <p class="text-sm font-medium">Date added</p>
        </li>
      </ul>
    </div>
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </BottomDrawer>
</div>
