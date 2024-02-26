<script lang="ts">
  import { dispatch } from '$lib/dispatcher';
  import LL from '$src/i18n/i18n-svelte';
  import ThemeSelect from '$src/lib/app/settings/ThemeSelect.svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import { state } from '$src/stores';

  let currentValue = $state.profile_settings.profile?.theme ?? 'system';
  let selected: string;
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.THEME.NAVBAR_TITLE()} />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <ThemeSelect
    defaultValue={currentValue}
    on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { theme: e.detail.value } })}
  />
</div>
