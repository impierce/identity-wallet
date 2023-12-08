<script lang="ts">
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';
  import type { Action } from '../../src-tauri/bindings/Action';


  let selected_locale: string = 'en';

  const setLocale = async () => dispatch({ type: '[Settings] Set locale', locale: selected_locale });

  $: {
    if ($state?.locale) {
      selected_locale = $state.locale;
      console.log(`selected_locale: ${selected_locale}`);
    }
  }
</script>

<!-- <LanguageSelect bind:selected={selected_locale} /> -->
