<script lang="ts">
  import { dispatch } from '$lib/dispatcher';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import ThemeSelect from '$src/lib/settings/ThemeSelect.svelte';
  import { state } from '$src/stores';

  let currentValue = $state.active_profile?.theme ?? 'system';
  let selected: string;
</script>

<TopNavBar on:back={() => history.back()} title="Select Theme" />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <ThemeSelect
    defaultValue={currentValue}
    on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { theme: e.detail.value } })}
  />
</div>
