<script lang="ts">
  import { goto } from '$app/navigation';

  import { TopNavigation } from '@impierce/ui-components';

  import Button from '$src/lib/components/Button.svelte';
  import CredentialListEntry from '$src/lib/components/CredentialListEntry.svelte';
  import CredentialOfferEntry from '$src/lib/components/CredentialOfferEntry.svelte';
  import PaddedIcon from '$src/lib/components/PaddedIcon.svelte';
  import { icons } from '$src/lib/credentials/customization/utils';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import RocketLaunch from '~icons/ph/rocket-launch';
  import SealCheck from '~icons/ph/seal-check-fill';

  let selected_credentials = $state.credentials?.filter(
    (c) => $state.current_user_prompt.options.indexOf(c.id) > -1
  );

  let client_name = $state.current_user_prompt.client_name;
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavigation title={'Share Data'} on:back={() => history.back()} />

  <div class="mt-[40px] flex grow flex-col items-center justify-start p-4">
    <!-- Header -->
    <div
      class="w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
    >
      <!-- Logo -->
      <div class="flex flex-col items-center space-y-[10px]">
        <!-- Placeholder -->
        <!-- <PaddedIcon icon={PlugsConnected} /> -->
        <div
          class="flex h-[64px] w-[64px] overflow-hidden rounded-2xl border-slate-300 hover:border dark:border-slate-600 dark:bg-silver"
        >
          <img src={$state.current_user_prompt.logo_uri} alt="logo" class="object-scale-down" />
        </div>
        <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
          {client_name}
        </p>
      </div>
      <!-- Data sensitivity -->
      <div class="p-2">
        <!-- TODO: data sensitivity -->
      </div>
    </div>

    <div class="mt-[30px] flex w-full items-center">
      <SealCheck class="mr-2 text-primary" />
      <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">Requested</p>
    </div>

    <!-- Credentials selection -->
    <!-- <div class="w-full space-y-2 rounded-2xl bg-white p-3"></div> -->
    <div
      class="mt-3 w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
    >
      <div class="flex w-full flex-col space-y-2">
        {#each selected_credentials as credential}
          <CredentialOfferEntry
            index={credential.id}
            title={credential.metadata.display.name || credential.data.type.at(-1)}
            description={credential.data.issuer}
            color={credential.metadata.display.color || 'bg-indigo-100'}
          >
            <span slot="icon">
              <svelte:component
                this={icons[credential.metadata.display.icon] || icons['User']}
                class="h-[18px] w-[18px] text-slate-800"
              />
            </span>
          </CredentialOfferEntry>
          <!-- <div class="flex items-center">
            <div class="grow">
              <CredentialListEntry
                id={credential.id}
                title={credential.metadata.display.name || credential.data.type.at(-1)}
                description={new URL(credential.data.issuer).hostname}
                color={credential.metadata.display.color || 'bg-indigo-100'}
              >
                <span slot="icon">
                  <svelte:component
                    this={icons[credential.metadata.display.icon] || icons['User']}
                    class="h-[18px] w-[18px] text-slate-800"
                  />
                </span>
              </CredentialListEntry>
            </div>
            <div class="px-3">
              <button
                use:melt={$root}
                class="flex h-6 w-6 appearance-none items-center justify-center
                  rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
                  {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
                id="checkbox"
              >
                {#if $isChecked}
                  <Check class="h-3 w-3" />
                {/if}
                <input use:melt={$input} />
              </button>
            </div>
          </div> -->
        {/each}
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div
    class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark"
  >
    <Button
      label="Approve request"
      on:click={() =>
        dispatch({
          type: '[Authenticate] Credentials selected',
          payload: { credential_uuids: selected_credentials.map((c) => c.id) }
        })}
    />
    <Button
      label="Cancel"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}
    />
  </div>
</div>

<div class="safe-area-top bg-white dark:bg-dark" />
<div class="safe-area-bottom" />

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
