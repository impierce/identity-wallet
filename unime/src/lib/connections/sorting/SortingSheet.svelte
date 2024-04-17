<script lang="ts">
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import SortPreferencesButton from '$src/lib/connections/sorting/SortingSheetButton.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import AlphabeticalOrder from '$src/lib/static/AlphabeticalOrder.svelte';

  import Issued from '~icons/ph/calendar-check';
  import Added from '~icons/ph/calendar-plus';
  import Slider from '~icons/ph/sliders-horizontal';

  import ActionSheet from '../../components/molecules/dialogs/ActionSheet.svelte';

  let preferences: 'alphabetical' | 'issued' | 'added';
  let sortingOrder: 'ascending' | 'descending';
</script>

<!-- bottom drawer and sorting button-->
<div class="sort_wrapper">
  <ActionSheet titleText={$LL.SORT.TITLE()}>
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
          icon={AlphabeticalOrder}
          label={$LL.SORT.PREFERENCES.ALPHABETICAL()}
          active={preferences == 'alphabetical'}
          {sortingOrder}
          on:click={() => {
            // Preference already selected
            if (preferences == 'alphabetical') {
              if (sortingOrder == 'descending') {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'name_az', reverse: true },
                });
                sortingOrder = 'ascending';
              } else {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'name_az', reverse: false },
                });
                sortingOrder = 'descending';
              }
              // Preference not yet selected
            } else {
              dispatch({
                type: '[Settings] Update Sorting Preference',
                payload: { credential_sorting: 'name_az', reverse: false },
              });
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
            // Preference already selected
            if (preferences == 'issued') {
              if (sortingOrder == 'ascending') {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'issue_date_new_old', reverse: true },
                });
                sortingOrder = 'descending';
              } else {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'issue_date_new_old', reverse: false },
                });
                sortingOrder = 'ascending';
              }
              // Preference not yet selected
            } else {
              dispatch({
                type: '[Settings] Update Sorting Preference',
                payload: { credential_sorting: 'issue_date_new_old', reverse: false },
              });
              preferences = 'issued';
              sortingOrder = 'ascending';
            }
          }}
        />
        <SortPreferencesButton
          icon={Added}
          label={$LL.SORT.PREFERENCES.DATE_ADDED()}
          active={preferences == 'added'}
          {sortingOrder}
          on:click={() => {
            // Preference already selected
            if (preferences == 'added') {
              if (sortingOrder == 'ascending') {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'added_date_new_old', reverse: true },
                });
                sortingOrder = 'descending';
              } else {
                dispatch({
                  type: '[Settings] Update Sorting Preference',
                  payload: { credential_sorting: 'added_date_new_old', reverse: false },
                });
                sortingOrder = 'ascending';
              }
              // Preference not yet selected
            } else {
              dispatch({
                type: '[Settings] Update Sorting Preference',
                payload: { credential_sorting: 'added_date_new_old', reverse: false },
              });
              preferences = 'added';
              sortingOrder = 'ascending';
            }
          }}
        />
      </div>
    </div>
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </ActionSheet>
</div>
