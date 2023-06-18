<script lang="ts">
  import { Button, LoadingSpinner, Input, Label } from '@impierce/ui-components';
  import LL from '../../i18n/i18n-svelte';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { onMount } from 'svelte';

  //   const registerFocus = useFocus();

  let display_name: string | undefined;
  let password: string | undefined;

  let loading = false;

  const createProfile = async () => {
    loading = true;
    dispatch({ type: '[DID] Create new', payload: { display_name: display_name, password } });
  };

  onMount(async () => {
    // usernameInput.focus();
    display_name = 'Tony Stark';
    password = 'my-password';
  });
</script>

<div class="flex h-auto flex-col items-center justify-center space-y-8 p-8">
  <h1 data-testid="label-welcome" class="font-serif text-2xl font-semibold text-slate-800">
    {$LL.WELCOME()}!
  </h1>

  <img
            src="image/undraw_welcome_re_h3d9.svg"
            alt="undraw_fingerprint"
            class="mx-auto my-4 w-[180px]"
          />

  <LocaleSelect />


  <div class="grid w-full max-w-sm items-center gap-1.5">
    <Label for="name">Name</Label>
    <Input type="text" id="name" placeholder="" bind:value={display_name} />
    <p class="text-muted-foreground text-sm">You can change this later.</p>
  </div>

  <div class="grid w-full max-w-sm items-center gap-1.5">
    <Label for="password">Password</Label>
    <Input type="password" id="password" placeholder="" bind:value={password} />
    <p class="text-muted-foreground text-sm">Please choose a strong password.</p>
  </div>

  <Button disabled={loading} on:click={createProfile}>
    {#if loading}
      <div class="mr-2">
        <LoadingSpinner />
      </div>
    {/if}
    {$LL.CREATE_IDENTITY()}
  </Button>
</div>
