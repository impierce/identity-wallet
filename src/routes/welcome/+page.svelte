<script lang="ts">
  import { Button } from '@impierce/ui-components';
  import LL from '../../i18n/i18n-svelte';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { onMount } from 'svelte';
  import { Location, Stronghold } from '@tauri-apps/plugin-stronghold';
  import Layout from '../+layout.svelte';

  //   const registerFocus = useFocus();

  let usernameInput: HTMLInputElement;
  let passwordInput: HTMLInputElement;

  let loading = false;

  const createProfile = async () => {
    // dispatch({ type: '[DID] Create new', payload: { display_name: usernameInput.value, password: passwordInput.value } });

    // TODO: Do we even want to speak to Stronghold directly? Or should we only call it in a reducer?
    console.log('Creating or loading a Stronghold ...');
    loading = true;
    const stronghold = await Stronghold.load(
      '/Users/daniel/Library/Application Support/com.impierce.identity_wallet/stronghold.bin',
      passwordInput.value
    ).catch((error) => {
      console.error(error);
      loading = false;
    });
    console.log(stronghold.path);
    const location = Location.generic('my-vault', 'my-record');
    loading = false;

    dispatch({ type: '[DID] Create new', payload: { display_name: usernameInput.value } });
    // await stronghold.save();
    // const location = new Location('my-type', { 'my-key': 'my-value' });
    // const client = await stronghold.createClient('my-client');
    // const store = client.getStore();
    // await store.insert('my-key', Array.from(new TextEncoder().encode('my-value')));
  };

  onMount(async () => {
    usernameInput.focus();
    usernameInput.value = 'Tony Stark';
    passwordInput.value = 'my-password';
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
  <p data-testid="label-prompt-username" class="text-slate-600">Password</p>
  <div>
    <input
      type="password"
      data-testid="input-password"
      class="w-full rounded-lg border px-4 py-2 shadow focus:outline-none focus:ring-2 focus:ring-violet-600"
      placeholder=""
      bind:this={passwordInput}
    />
  </div>
  <Button label={$LL.CREATE_IDENTITY()} on:clicked={createProfile} />
  {#if loading}
    Loading ...
  {/if}
  <LocaleSelect />
</div>
