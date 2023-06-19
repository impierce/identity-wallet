<script lang="ts">
  import { Button } from '@impierce/ui-components';
  import LL from '../../i18n/i18n-svelte';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { onMount } from 'svelte';

  //   const registerFocus = useFocus();

  let usernameInput: HTMLInputElement;

  const createProfile = async () =>
    dispatch({ type: '[DID] Create new', payload: { display_name: usernameInput.value } });

  onMount(() => {
    usernameInput.focus();
  });
</script>

<div class="flex h-auto flex-col items-center justify-center space-y-8 p-8">
  <h1 data-testid="label-welcome" class="font-serif text-2xl font-semibold text-slate-800">
    {$LL.WELCOME()}!
  </h1>
  <p data-testid="label-prompt-username" class="text-slate-600">{$LL.PROMPT_NAME()}</p>
  <!-- TODO: replace with ui-components/Input -->
  <div>
    <input
      type="text"
      data-testid="input-username"
      class="w-full rounded-lg border px-4 py-2 shadow focus:outline-none focus:ring-2 focus:ring-violet-600"
      placeholder=""
      bind:this={usernameInput}
    />
  </div>
  <Button label={$LL.CREATE_IDENTITY()} on:clicked={createProfile} />
  <LocaleSelect />
</div>
