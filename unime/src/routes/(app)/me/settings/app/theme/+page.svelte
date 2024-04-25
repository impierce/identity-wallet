<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import ThemeSelect from '$lib/app/settings/ThemeSelect.svelte';
  import TopNavBar from '$lib/components/molecules/navigation/TopNavBar.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { state } from '$lib/stores';

  let currentValue = $state.profile_settings.profile?.theme ?? 'system';
</script>

<TopNavBar on:back={() => history.back()} title={$LL.SETTINGS.APP.THEME.NAVBAR_TITLE()} />
<div class="h-full bg-silver p-4 dark:bg-navy">
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      {$LL.SETTINGS.APP.THEME.TITLE_1()}
      <span class="text-primary">{$LL.SETTINGS.APP.THEME.TITLE_2()}</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">
      {$LL.SETTINGS.APP.THEME.SUBTITLE()}
    </p>
  </div>
  <ThemeSelect
    defaultValue={currentValue}
    on:change={(e) => dispatch({ type: '[Settings] Update profile', payload: { theme: e.detail.value } })}
  />
</div>
