<script lang="ts">
  import type { Action } from '../../src-tauri/bindings/Action';
  import { invoke } from '@tauri-apps/api/tauri';

  let selected_locale;

  const setLocale = async () => {
    await invoke('execute_command', {
      action: { type: '[SETTINGS] Set locale', payload: selected_locale } as Action
    });
  };
</script>

<div>
  <select
    bind:value={selected_locale}
    on:change={() => setLocale()}
    class="block w-full rounded-lg border border-gray-300 bg-gray-50 p-2.5 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500 dark:border-gray-600 dark:bg-gray-700 dark:text-white dark:placeholder-gray-400 dark:focus:border-blue-500 dark:focus:ring-blue-500"
  >
    <option value="en">English</option>
    <option value="nl">Nederlands</option>
    <option value="de">Deutsch</option>
  </select>
</div>
