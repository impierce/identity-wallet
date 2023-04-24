<script lang="ts">
  import { dispatch } from './dispatcher';
  import { state } from '../stores';

  let selected_locale: string;

  const setLocale = async () =>
    dispatch({ type: '[Settings] Set locale', payload: { locale: selected_locale } });

  $: {
    if ($state?.locale) {
      console.log('$state.locale', $state.locale);
    }
  }
</script>

<div>
  <select
    bind:value={selected_locale}
    on:change={setLocale}
    class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
  >
    <option value="en">English</option>
    <option value="nl">Nederlands</option>
    <option value="de">Deutsch</option>
  </select>
</div>
