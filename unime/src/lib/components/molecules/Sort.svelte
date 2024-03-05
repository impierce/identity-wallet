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

  let preferredView: 'list' | 'grid' = 'list';
  let preferences: 'alphabetical' | 'issued' | 'added';


</script>


<div class="sort_wrapper">
  <BottomDrawer titleText={$LL.SORT.TITLE()}>
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-gray-100"
      ><Slider /></button
    >
    <div slot="content" class="w-full">
      <ul class="relative pb-5">
        <li
          on:click={() => (preferredView = 'list')}
          class={`flex gap-2 border p-[10px] my-1 text-slate-800 dark:text-gray-100 ${
            preferredView == 'list' ? 'rounded-lg border-grey bg-silver' : 'border-transparent'
          }`}
        >
          <List />
          <p class="text-sm font-medium">{$LL.SORT.PREFERENCES.LIST_VIEW()}</p>
        </li>
        <li
          on:click={() => (preferredView = 'grid')}
          class={`flex gap-2 border p-[10px] my-1 text-slate-800 dark:text-gray-100 ${
            preferredView == 'grid' ? 'rounded-lg border-grey bg-silver' : 'border-transparent'
          }`}
        >
          <Grid />
          <p class="text-sm font-medium">{$LL.SORT.PREFERENCES.GRID_VIEW()}</p>
        </li>

        <hr class="full-width fill-[#efefef]" />

        <li on:click={() => (preferences = 'alphabetical')} class={`flex gap-2 border p-[10px] my-1 text-slate-800 dark:text-gray-100 ${
          preferences == 'alphabetical' ? 'rounded-lg border-grey bg-silver' : 'border-transparent'
        }`}>
          <Ascending />
          <p class="text-sm font-medium">{$LL.SORT.PREFERENCES.ALPHABETICAL()}</p>
        </li>

        <li on:click={() => (preferences = 'issued')} class={`flex gap-2 border p-[10px] my-1 text-slate-800 dark:text-gray-100 ${
          preferences == 'issued' ? 'rounded-lg border-grey bg-silver' : 'border-transparent'
        }`}>
          <Issued />
          <p class="text-sm font-medium">{$LL.SORT.PREFERENCES.DATE_ISSUED()}</p>
        </li>

        <li on:click={() => (preferences = 'added')} class={`flex gap-2 border p-[10px] my-1 text-slate-800 dark:text-gray-100 ${
          preferences == 'added' ? 'rounded-lg border-grey bg-silver' : 'border-transparent'
        }`}>
          <Added />
          <p class="text-sm font-medium">{$LL.SORT.PREFERENCES.DATE_ADDED()}</p>
        </li>
      </ul>
    </div>
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </BottomDrawer>
</div>
