<script lang="ts">
  import { Button } from '@impierce/ui-components';
  import { useFocus } from 'svelte-navigator';
  import LL from '../i18n/i18n-svelte';
  import LocaleSelect from '../lib/LocaleSelect.svelte';
  import { dispatch } from '../lib/dispatcher';

  const registerFocus = useFocus();

  let usernameInput: HTMLInputElement;

  const createProfile = async () =>
    dispatch({ type: '[DID] Create new', payload: { display_name: usernameInput.value } });
</script>

<div class="flex h-auto flex-col items-center justify-center space-y-8 p-8">
  <h1 data-testid="label-welcome" class="font-serif text-2xl font-semibold text-gray-800">
    {$LL.WELCOME()}!
  </h1>
  <p data-testid="label-prompt-username" class="text-gray-600">{$LL.PROMPT_NAME()}</p>
  <!-- TODO: replace with ui-components/Input -->
  <div>
    <input
      type="text"
      data-testid="input-username"
      class="w-full rounded-lg border px-4 py-2 shadow focus:outline-none focus:ring-2 focus:ring-violet-600"
      placeholder=""
      bind:this={usernameInput}
      use:registerFocus
    />
  </div>
  <Button label={$LL.CREATE_IDENTITY()} on:clicked={createProfile} />
  <LocaleSelect />
</div>
