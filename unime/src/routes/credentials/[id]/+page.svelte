<script lang="ts">
  import { page } from '$app/stores';
  import LL from '$i18n/i18n-svelte';
  import { fly } from 'svelte/transition';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import { Button } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { state as appState, pageTitleStore } from '$lib/stores';

  import CredentialHeader from './CredentialHeader.svelte';
  import CredentialOverview from './CredentialOverview.svelte';
  import DefaultRenderer from './DefaultRenderer.svelte';
  import OpenBadgeRenderer from './OpenBadgeRenderer.svelte';

  // Credential cannot be loaded via load function since it's stored in the application state.
  // TODO Credential should be loaded from backend via load function to handle invalid IDs properly.
  function loadCredential() {
    return $appState.credentials.find((c) => $page.params.id === c.id);
  }

  let credential: DisplayCredential | undefined = loadCredential();

  let displayName: string = credential?.display_name ?? credential?.data.name;
  let displayNameUpdated = displayName;

  let openEditMode = false;

  // The credential's metadata, e.g., the favorite status, may change.
  // ESLint does not understand the reactive statement.
  // eslint-disable-next-line @typescript-eslint/no-unused-expressions
  $: $appState, (credential = loadCredential());

  $: {
    // Use `pageTitleStore` to get page title into layout.
    pageTitleStore.set($LL.CREDENTIAL.NAVBAR_TITLE());
  }

  const credentialTypes = credential?.data?.type as string[] | undefined;

  function discard() {
    displayNameUpdated = displayName;
    openEditMode = false;
  }

  async function update() {
    await dispatch({ type: '[Credential Metadata] Update', payload: { id: credential?.id, name: displayNameUpdated } });
    openEditMode = false;
  }
</script>

{#if credential}
  <div class="flex min-h-full flex-col gap-7 bg-background-alt px-4 pb-7">
    <CredentialHeader {credential}>
      <!-- Editable title -->
      <div class="w-full px-4">
        <input
          type="text"
          maxlength="32"
          bind:value={displayNameUpdated}
          class="-my-2 h-10 w-full truncate bg-background text-center font-semibold focus:outline-none"
          placeholder="Some title"
          onfocus={() => (openEditMode = true)}
          onblur={() => {
            // If no change was made, automatically close edit mode when input loses focus.
            if (displayName === displayNameUpdated) {
              openEditMode = false;
            }
          }}
        />
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
    {:else}
      <DefaultRenderer {credential} />
    {/if}
  </div>
{/if}
