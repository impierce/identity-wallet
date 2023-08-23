<script lang="ts">
  import { Button, LoadingSpinner, Input, Label } from '@impierce/ui-components';
  import LL from '$src/i18n/i18n-svelte';
  import LocaleSelect from '$lib/LocaleSelect.svelte';
  import { dispatch } from '$lib/dispatcher';
  import { onMount } from 'svelte';

  import GB from '~icons/flag/gb-4x3';
  import NL from '~icons/flag/nl-4x3';
  import DE from '~icons/flag/de-4x3';

  //   const registerFocus = useFocus();

  let display_name: string | undefined;
  let password: string | undefined;

  let loading = false;

  const createProfile = async () => {
    loading = true;
    dispatch({ type: '[DID] Create new', payload: { display_name: display_name, password } });
  };

  const setLanguage = (locale: string) => {
    active_language = locale;
    dispatch({ type: '[Settings] Set locale', payload: { locale } });
  };

  let active_language = 'en';
  let active_language_class = 'rounded-lg ring-2 ring-violet-500 ring-offset-4';

  onMount(async () => {
    // usernameInput.focus();
    display_name = 'Tony Stark';
    password = 'sup3rSecr3t';
  });
</script>

<div class="flex h-auto flex-col items-center justify-center space-y-8 p-8">
  <h1 data-testid="label-welcome" class="font-serif text-2xl font-semibold text-slate-800">
    Welcome!
  </h1>

  <img
    src="image/undraw_welcome_re_h3d9.svg"
    alt="undraw_fingerprint"
    class="mx-auto my-4 w-[180px]"
  />

  <!-- <LocaleSelect /> -->

  <div class="grid grid-flow-col space-x-4">
    <button
      class={active_language === 'en' ? active_language_class : ''}
      on:click={() => setLanguage('en')}><GB class="h-[27px] w-[36px] rounded-lg" /></button
    >
    <button
      class={active_language === 'nl' ? active_language_class : ''}
      on:click={() => setLanguage('nl')}><NL class="h-[27px] w-[36px] rounded-lg" /></button
    >
    <button
      class={active_language === 'de' ? active_language_class : ''}
      on:click={() => setLanguage('de')}><DE class="h-[27px] w-[36px] rounded-lg" /></button
    >
  </div>

  <div class="grid w-full max-w-sm items-center gap-1.5">
    <Label for="name">{$LL.PROFILE_NAME()}</Label>
    <Input type="text" id="name" placeholder="" bind:value={display_name} />
    <p class="text-muted-foreground text-sm">{$LL.CHANGE_LATER()}</p>
  </div>

  <div class="grid w-full max-w-sm items-center gap-1.5">
    <Label for="password">{$LL.PASSWORD()}</Label>
    <Input type="password" id="password" placeholder="" bind:value={password} />
    <p class="text-muted-foreground text-sm">{$LL.STRONG_PASSWORD()}</p>
  </div>

  <Button disabled={loading} on:click={createProfile} class="shadow-neon">
    {#if loading}
      <div class="mr-2">
        <LoadingSpinner />
      </div>
    {/if}
    {$LL.CREATE_IDENTITY()}
  </Button>
</div>
