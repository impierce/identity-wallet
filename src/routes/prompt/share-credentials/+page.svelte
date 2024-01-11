<script lang="ts">
  import { goto } from '$app/navigation';

  import { dispatch } from '$lib/dispatcher';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import Checkbox from '$src/lib/components/atoms/Checkbox.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import ListItemCard from '$src/lib/credentials/ListItemCard.svelte';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import RocketLaunch from '~icons/ph/rocket-launch';
  import SealCheck from '~icons/ph/seal-check-fill';

  let selected_credentials = $state.credentials?.filter((c) => $state.current_user_prompt.options.indexOf(c.id) > -1);

  let client_name = $state.current_user_prompt.client_name;
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={'Share Data'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    <!-- Header -->
    {#if $state.current_user_prompt.logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver">
        <img src={$state.current_user_prompt.logo_uri} alt="logo" />
      </div>
    {:else}
      <PaddedIcon icon={PlugsConnected} />
    {/if}
    <div class="text-center">
      <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
        {client_name}
      </p>
      <!-- <p class="pt-[10px] text-sm font-medium text-slate-500">
        {hostname}
      </p> -->
    </div>

    <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      requests the following credentials
    </p>

    <div class="w-full">
      <div class="flex items-center">
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
            <ListItemCard id={credential.id} title={credential.metadata.display.name || credential.data.type.at(-1)}>
              <div slot="right" class="mr-2">
                <Checkbox checked={true} disabled={true} />
              </div>
            </ListItemCard>
          {/each}
        </div>
      </div>
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label="Approve request"
      on:click={() =>
        dispatch({
          type: '[Authenticate] Credentials selected',
          payload: { credential_uuids: selected_credentials.map((c) => c.id) },
        })}
    />
    <Button
      label="Cancel"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
        goto('/me');
      }}
    />
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
