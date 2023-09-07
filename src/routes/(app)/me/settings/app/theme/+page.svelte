<script lang="ts">
  import { TopNavigation } from '@impierce/ui-components';

  import Button from '$src/lib/components/Button.svelte';
  import ThemeSelect from '$src/lib/customize/ThemeSelect.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let currentValue = $state.active_profile?.theme ?? 'system';
  let selected: string;
</script>

<TopNavigation on:back={() => history.back()} title="Select Theme" />
<div class="h-full bg-bg-secondary p-4 dark:bg-bg-dark-secondary">
  <ThemeSelect
    defaultValue={currentValue}
    on:change={(e) =>
      dispatch({ type: '[Settings] Update profile', payload: { theme: e.detail.value } })}
  />
</div>
