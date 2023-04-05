<script lang="ts">
  import { Button } from '@impierce/ui-components';
  import { invoke } from '@tauri-apps/api/tauri';
  import { useFocus } from 'svelte-navigator';
  import LL, { locale } from '../i18n/i18n-svelte';

  const registerFocus = useFocus();

  //   export let location;
  //   export let navigate;

  let usernameInput: HTMLInputElement;

  const createProfile = async () => {
    await invoke('execute_command', {
      commandMessage: { command: '[DID] Create new', payload: usernameInput.value }
    });
  };
</script>

<div class="grid place-items-center space-y-8 p-8">
  <h1 class="text-2xl font-semibold font-serif">{$LL.WELCOME()}!</h1>
  <p>{$LL.PROMPT_NAME()}</p>
  <!-- TODO: replace with ui-components/Input -->
  <div>
    <input
      type="text"
      class="rounded-md border border-slate-200 px-4 py-2 shadow"
      placeholder=""
      bind:this={usernameInput}
      use:registerFocus
    />
  </div>
  <Button label={$LL.CREATE_IDENTITY()} on:clicked={createProfile} />
</div>
