<script lang="ts">
  import { Button } from '@impierce/ui-components';
  import { invoke } from '@tauri-apps/api/tauri';
  import { useFocus } from 'svelte-navigator';

  const registerFocus = useFocus();

//   export let location;
//   export let navigate;

  let usernameInput: HTMLInputElement;

  const createProfile = async () => {
    await invoke('execute_command', {
        commandMessage: { command: '[DID] Create new', payload: usernameInput.value }
    });
  }

</script>

<div>
  <h1>Welcome!</h1>
  <p>Please enter your name</p>
  <!-- TODO: replace with Input -->
  <div>
    <input type="text" class="rounded-md py-2 px-4 shadow border border-slate-200" placeholder="Your name ..." bind:this={usernameInput} use:registerFocus/>
  </div>
  <Button label="Create identity" on:clicked={createProfile} />
</div>
