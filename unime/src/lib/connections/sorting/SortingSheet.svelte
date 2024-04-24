<script lang="ts">
  import type { CredentialSortMethod } from '@bindings/CredentialSortMethod';
  import { melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import SortPreferencesButton from '$src/lib/connections/sorting/SortingSheetButton.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import AlphabeticalOrder from '$src/lib/static/AlphabeticalOrder.svelte';
  import { state } from '$src/stores';

  import Issued from '~icons/ph/calendar-check';
  import Added from '~icons/ph/calendar-plus';
  import Slider from '~icons/ph/sliders-horizontal';

  import ActionSheet from '../../components/molecules/dialogs/ActionSheet.svelte';

  let method: CredentialSortMethod = $state.profile_settings.sorting_preferences.credentials.sort_method;
  let reversed: boolean = $state.profile_settings.sorting_preferences.credentials.reverse;

  function updateSortingPreference(credential_sorting: CredentialSortMethod) {
    if (method == credential_sorting) {
      reversed = !reversed;
    } else {
      method = credential_sorting;
    }
    dispatch({
      type: '[Settings] Update Sorting Preference',
      payload: { credential_sorting, reverse: reversed },
    });
  }
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
          active={method == 'name_az'}
          {reversed}
          on:click={() => updateSortingPreference('name_az')}
        />
        <SortPreferencesButton
          icon={Issued}
          label={$LL.SORT.PREFERENCES.DATE_ISSUED()}
          active={method == 'issue_date_new_old'}
          {reversed}
          on:click={() => updateSortingPreference('issue_date_new_old')}
        />
        <SortPreferencesButton
          icon={Added}
          label={$LL.SORT.PREFERENCES.DATE_ADDED()}
          active={method == 'added_date_new_old'}
          {reversed}
          on:click={() => updateSortingPreference('added_date_new_old')}
        />
      </div>
    </div>
    <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
  </ActionSheet>
</div>
