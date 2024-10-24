<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { CredentialSortMethod } from '@bindings/profile_settings/CredentialSortMethod';
  import { melt } from '@melt-ui/svelte';

  import { ActionSheet, Button } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CalendarCheckRegularIcon, CalendarPlusRegularIcon, SlidersHorizontalRegularIcon } from '$lib/icons';
  import AlphabeticalOrder from '$lib/static/svg/AlphabeticalOrder.svelte';
  import { state } from '$lib/stores';

  import SortingSheetButton from './SortingSheetButton.svelte';

  let method: CredentialSortMethod = $state.profile_settings.sorting_preferences.credentials.sort_method;
  let reversed: boolean = $state.profile_settings.sorting_preferences.credentials.reverse;

  function updateSortingPreference(credential_sorting: CredentialSortMethod) {
    if (method === credential_sorting) {
      reversed = !reversed;
    } else {
      method = credential_sorting;
    }
    dispatch({
      type: '[Settings] Update sorting preference',
      payload: { credential_sorting, reverse: reversed },
    });
  }
</script>

<ActionSheet titleText={$LL.SORT.TITLE()}>
  <button
    slot="trigger"
    let:trigger
    use:melt={trigger}
    class="flex h-10 w-10 items-center justify-center rounded-xl bg-white dark:bg-dark dark:text-grey"
    ><SlidersHorizontalRegularIcon /></button
  >
  <!-- bottom drawer and list items with preferred view and sorting preferences-->
  <div slot="content" class="w-full">
    <div class="relative pb-5">
      <SortingSheetButton
        icon={AlphabeticalOrder}
        label={$LL.SORT.PREFERENCES.ALPHABETICAL()}
        active={method === 'name_az'}
        {method}
        {reversed}
        on:click={() => updateSortingPreference('name_az')}
      />
      <SortingSheetButton
        icon={CalendarCheckRegularIcon}
        label={$LL.SORT.PREFERENCES.DATE_ISSUED()}
        active={method === 'issue_date_new_old'}
        {method}
        {reversed}
        on:click={() => updateSortingPreference('issue_date_new_old')}
      />
      <SortingSheetButton
        icon={CalendarPlusRegularIcon}
        label={$LL.SORT.PREFERENCES.DATE_ADDED()}
        active={method === 'added_date_new_old'}
        {method}
        {reversed}
        on:click={() => updateSortingPreference('added_date_new_old')}
      />
    </div>
  </div>
  <Button variant="primary" slot="close" let:close trigger={close} label={$LL.CLOSE()} />
</ActionSheet>
