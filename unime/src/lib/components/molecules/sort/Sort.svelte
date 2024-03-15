<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import ArrowsDown from '$src/lib/static/svg/ArrowsDown.svelte';

  import Alphabetical from '~icons/mdi/order-alphabetical-ascending';
  import Issued from '~icons/ph/calendar-check';
  import Added from '~icons/ph/calendar-plus';
  import Checks from '~icons/ph/checks';
  import List from '~icons/ph/list-dashes';
  import Slider from '~icons/ph/sliders-horizontal';
  import Ascending from '~icons/ph/sort-ascending';
  import Grid from '~icons/ph/squares-four';

  import BottomDrawer from '../dialogs/BottomDrawer.svelte';
  import SortPreferencesButton from './SortPreferencesButton.svelte';

  let preferredView: 'list' | 'grid' = 'list';
  let preferences: 'alphabetical' | 'issued' | 'added';
  let sortingOrder: 'ascending' | 'descending';
</script>

<!-- bottom drawer and sorting button-->
<div class="sort_wrapper">
  <BottomDrawer titleText={$LL.SORT.TITLE()}>
    <button
      slot="trigger"
      let:trigger
      use:melt={trigger}
      class="flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-grey"
      ><Slider /></button
    >
    <!-- bottom drawer and list items with preferred view and sorting preferences-->
    <div slot="content" class="w-full">
      <div class="relative pb-5">
        <SortPreferencesButton
          icon={List}
          label={$LL.SORT.PREFERENCES.LIST_VIEW()}
          active={preferredView == 'list'}
          on:click={() => (preferredView = 'list')}
        />
        <SortPreferencesButton
          icon={Grid}
          label={$LL.SORT.PREFERENCES.GRID_VIEW()}
          active={preferredView == 'grid'}
          on:click={() => (preferredView = 'grid')}
        />

        <hr class="full-width fill-[#efefef]" />

        <SortPreferencesButton
          icon={Alphabetical}
          label={$LL.SORT.PREFERENCES.ALPHABETICAL()}
          active={preferences == 'alphabetical'}
          {sortingOrder}
          on:click={() => {
            // Preference already selected
            if (preferences == 'alphabetical') {
              if (sortingOrder == 'descending') {
                dispatch({
                  type: '[User Data] Query',
                  payload: { target: 'Credentials', sort_method: 'NameAZ', sort_reverse: true },
                });
                sortingOrder = 'ascending';
              } else {
                dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', sort_method: 'NameAZ' } });
                sortingOrder = 'descending';
              }
              // Preference not yet selected
            } else {
              dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', sort_method: 'NameAZ' } });
              preferences = 'alphabetical';
              sortingOrder = 'descending';
            }
          }}
        />
        <SortPreferencesButton
          icon={Issued}
          label={$LL.SORT.PREFERENCES.DATE_ISSUED()}
          active={preferences == 'issued'}
          {sortingOrder}
          on:click={() => {
            dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', sort_method: 'IssuanceNewOld' } });
            preferences = 'issued';
          }}
        />
        <SortPreferencesButton
          icon={Added}
          label={$LL.SORT.PREFERENCES.DATE_ADDED()}
          active={preferences == 'added'}
          {sortingOrder}
          on:click={() => {
            dispatch({ type: '[User Data] Query', payload: { target: 'Credentials', sort_method: 'AddedNewOld' } });
            preferences = 'added';
          }}
        />
      </div>
    </div>
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </BottomDrawer>
</div>
