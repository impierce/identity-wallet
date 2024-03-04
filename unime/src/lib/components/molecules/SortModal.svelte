<script lang="ts">
  import { onMount } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import { state } from '$src/stores';

  //import Arrows from '~icons/ph/arrows-down-up';
  import Issued from '~icons/ph/calendar-check';
  import Added from '~icons/ph/calendar-plus';
  import List from '~icons/ph/list-dashes';
  import Slider from '~icons/ph/sliders-horizontal';
  import Ascending from '~icons/ph/sort-ascending';
  import Grid from '~icons/ph/squares-four';

  import BottomDrawer from './dialogs/BottomDrawer.svelte';
</script>

<!-- To do: add dark version (dark:bg-dark), overlay, focus, close and open bottomdrawer-->
<!-- 
<div class="rounded-t-2xl bg-white">
  <div class="flex justify-center pt-3">
    <h3 class="font-medium">Sorting</h3>
  </div>
  <div>
    <ul class="relative">
      <li class=" flex gap-2 rounded-lg px-5 py-3 hover:bg-slate-200">
        <List />
        <p class="text-sm font-medium">List view</p>
      </li>

      <li class=" flex gap-2 rounded-lg px-5 py-3 hover:bg-slate-200">
        <Grid />
        <p class="text-sm font-medium">Grid view</p>
      </li>

      <hr class="mx-5 fill-[#EFEFEF]" />

      <li class="flex gap-2 rounded-lg px-5 py-3 hover:bg-slate-200">
        <Ascending />
        <p class="text-sm font-medium">Alphabetical</p>
      <Arrows class="ml-auto fill-primary"/> 
      </li>

      <li class="flex gap-2 rounded-lg px-5 py-3 hover:bg-slate-200">
        <Issued />
        <p class="text-sm font-medium">Date issued</p>
      </li>

      <li class="flex gap-2 rounded-lg px-5 py-3 hover:bg-slate-200">
        <Added />
        <p class="text-sm font-medium">Date added</p>
      </li>
    </ul>
  </div>

  <div class="p-5">
    <Button variant="primary" label={$LL.CLOSE()}/>
  </div>
</div>
-->

<!-- Button for sorting -->
<div class="mt-8">
  <BottomDrawer titleText="Sorting">
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class=" flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100"
      ><Slider /></button
    >
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </BottomDrawer>
</div>
