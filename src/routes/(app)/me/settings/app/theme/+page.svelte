<script lang="ts">
  import Button from '$src/lib/components/Button.svelte';
  import TopNavigation from '$src/lib/components/molecules/navigation/TopNavigation.svelte';
  import ThemeSelect from '$src/lib/customize/ThemeSelect.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  let currentValue = $state.active_profile?.theme ?? 'system';
  let selected: string;
</script>

<TopNavigation on:back={() => history.back()} title="Select Theme" />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <ThemeSelect
    defaultValue={currentValue}
    on:change={(e) => dispatch({ type: '[Settings] Update profile', theme: e.detail.value })}
  />
</div>
