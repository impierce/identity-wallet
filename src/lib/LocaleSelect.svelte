<script lang="ts">
  import { dispatch } from './dispatcher';
  import { state } from '../stores';
  import { Select } from '@impierce/ui-components';

  let selected_locale: string = 'en';

  const setLocale = async () =>
    dispatch({ type: '[Settings] Set locale', payload: { locale: selected_locale } });

  $: {
    if ($state?.locale) {
      selected_locale = $state.locale;
      console.log(`selected_locale: ${selected_locale}`);
    }
  }
</script>

<!-- <Select bind:selected={selected_locale} on:value={setLocale} /> -->
