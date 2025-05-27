<script lang="ts">
  import { page } from '$app/stores';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import Icon from '@iconify/svelte';

  import { Button } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState, pageTitleStore } from '$lib/stores';

  import AddressRenderer from './AddressRenderer.svelte';
  import CredentialHeader from './CredentialHeader.svelte';
  import CredentialOverview from './CredentialOverview.svelte';
  import DefaultRenderer from './DefaultRenderer.svelte';
  import OpenBadgeRenderer from './OpenBadgeRenderer.svelte';
  import PidRenderer from './PidRenderer.svelte';

  // Credential cannot be loaded via load function since it's stored in the application state.
  // TODO Credential should be loaded from backend via load function to handle invalid IDs properly.
  function loadCredential(): DisplayCredential {
    const credential = $appState.credentials.find((c) => $page.params.id === c.id);
    if (!credential) {
      throw new Error('No credential not found for id: ' + $page.params.id);
    }
    return credential;
  }

  let credential: DisplayCredential = loadCredential();

  let displayName: string = credential.display_name ?? credential.data.name;
  let displayNameUpdated = displayName;

  let openEditMode = false;

  const maxLength = 32;

  let labelInput: HTMLInputElement;

  // The credential's metadata, e.g., the favorite status, may change.
  // ESLint does not understand the reactive statement.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  $: $appState, (credential = loadCredential());

  $: {
    // Use `pageTitleStore` to get page title into layout.
    pageTitleStore.set($LL.CREDENTIAL.NAVBAR_TITLE());
  }

  const credentialTypes = credential.data?.type as string[] | undefined;

  function discard() {
    displayNameUpdated = displayName;
    openEditMode = false;
  }

  async function update() {
    await dispatch({ type: '[Credential Metadata] Update', payload: { id: credential.id, name: displayNameUpdated } });
    openEditMode = false;
  }
</script>

{#if credential}
  <div class="flex min-h-full flex-col gap-7 bg-background-alt px-4 pb-7">
    <CredentialHeader
      {credential}
      on:edit={() => {
        openEditMode = true;
        labelInput.focus();
      }}
    >
      <!-- Editable title -->
      <div class="relative w-full px-4">
        <input
          type="text"
          maxlength={maxLength}
          bind:value={displayNameUpdated}
          bind:this={labelInput}
          class="-my-2 h-[40px] w-full truncate rounded-lg border border-slate-300 bg-background-alt text-center font-semibold focus:outline-none disabled:border-none disabled:bg-background dark:border-slate-600"
          placeholder="Some title"
          disabled={!openEditMode}
        />
        {#if openEditMode}
          <!-- Character count -->
          <div
            class="absolute -top-6 right-5 font-mono text-xs font-medium tracking-tight text-slate-500 dark:text-slate-300"
          >
            {displayNameUpdated.length}/{maxLength}
          </div>
          <!-- Inline pencil icon -->
          <div class="absolute right-6 top-1/2 -translate-y-1/2">
            <Icon class="size-5 text-slate-500 dark:text-slate-300" icon="ph:pencil-fill" />
          </div>
        {/if}
      </div>
    </CredentialHeader>
    {#if openEditMode}
      <!-- Similar to ActionSheet, but without backdrop -->
      <div
        class="fixed bottom-0 left-0 flex w-screen flex-col space-y-[10px] rounded-t-[20px] bg-background-alt p-6"
        transition:fly={{
          y: 350,
          duration: 300,
          opacity: 1,
        }}
      >
        <Button label={$LL.DISCARD()} variant="secondary" on:click={discard} />
        <Button
          label={$LL.CREDENTIAL.ACTIONS.EDIT.CONFIRM_BUTTON()}
          disabled={displayNameUpdated.length === 0 || displayNameUpdated === displayName}
          on:click={update}
        />
      </div>
    {/if}
    <CredentialOverview {credential} />
    {#if credentialTypes?.includes('OpenBadgeCredential') || credentialTypes?.includes('AchievementCredential')}
      <OpenBadgeRenderer {credential} />
      <!-- TODO: the renderer should be determined by a `metadata` field -->
    {:else if credentialTypes?.includes('ResidenceCredential')}
      <AddressRenderer {credential} />
    {:else if credentialTypes?.includes('NaturalPersonCredential')}
      <PidRenderer {credential} />
    {:else}
      <DefaultRenderer {credential} />
    {/if}
  </div>
{/if}
