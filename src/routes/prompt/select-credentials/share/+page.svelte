<script lang="ts">
  import { goto } from '$app/navigation';

  import { TopNavigation } from '@impierce/ui-components';
  import { createCheckbox, melt } from '@melt-ui/svelte';

  import Button from '$src/lib/components/Button.svelte';
  import CredentialListEntry from '$src/lib/components/CredentialListEntry.svelte';
  import PaddedIcon from '$src/lib/components/PaddedIcon.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import RocketLaunch from '~icons/ph/rocket-launch';
  import SealCheck from '~icons/ph/seal-check-fill';
  import UserCircle from '~icons/ph/user-circle-light';

  const {
    elements: { root, input },
    helpers: { isChecked }
  } = createCheckbox({});

  let selected_credentials = $state.credentials?.filter(
    (c) => $state.current_user_prompt.options.indexOf(c.at(0)) > -1
  );
</script>

<div class="content-height flex flex-col items-stretch bg-neutral-100">
  <TopNavigation title={'Share Information'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-start space-y-4 p-4">
    <!-- Header -->
    <div class="w-full rounded-[20px] bg-white p-[10px]">
      <!-- Logo -->
      <div class="flex flex-col items-center space-y-[10px]">
        <!-- Placeholder -->
        <!-- <PaddedIcon icon={PlugsConnected} /> -->
        <div class="h-12 w-12 rounded-2xl border border-slate-300" />
        <p class="text-2xl font-semibold text-neutral-900">bestdex.com</p>
      </div>
      <!-- Data sensitivity -->
      <div>
        <!-- TODO: data sensitivity -->
      </div>
    </div>

    <div class="flex w-full items-center">
      <SealCheck class="mr-2 text-primary" />
      <p class="font-medium text-slate-600">Requested</p>
    </div>

    <!-- Credentials selection -->
    <div class="w-full rounded-[20px] bg-white p-[10px]">
      <div class="flex w-full flex-col space-y-2">
        {#each selected_credentials as credential}
          <div class="flex items-center">
            <div class="grow">
              <CredentialListEntry
                id={credential?.at(0)}
                title={credential?.at(1).type.at(1)}
                description={credential?.at(1).issuer}
                color="bg-indigo-100"
              >
                <span slot="icon"><UserCircle class="h-[18px] w-[18px] text-slate-800" /></span>
              </CredentialListEntry>
            </div>
            <div class="px-3">
              <button
                use:melt={$root}
                class="flex h-6 w-6 appearance-none items-center justify-center
                  rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
                  {$isChecked ? 'border-none bg-primary' : 'bg-white'}"
                id="checkbox"
              >
                {#if $isChecked}
                  <Check class="h-3 w-3" />
                {/if}
                <input use:melt={$input} />
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 pb-0">
    <Button
      label="Approve request"
      on:click={() =>
        dispatch({
          type: '[Authenticate] Credentials selected',
          payload: { credential_uuids: selected_credentials.map((c) => c.at(0)) }
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

  <div class="safe-area-top" />
  <div class="safe-area-bottom" />
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
