<script lang="ts">
  import { state } from '$src/stores';
  import { TopNavigation } from '@impierce/ui-components';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import WarningCircle from '~icons/ph/warning-circle-fill';
  import DownloadSimple from '~icons/ph/download-simple-fill';
  import X from '~icons/ph/x-bold';
  import Check from '~icons/ph/check-bold';
  import Question from '~icons/ph/question';
  import { createCheckbox, createPopover, melt } from '@melt-ui/svelte';
  import { fade } from 'svelte/transition';
  import { dispatch } from '$lib/dispatcher';
  import { goto } from '$app/navigation';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import Button from '$lib/components/Button.svelte';

  const {
    elements: { root, input },
    helpers: { isChecked }
  } = createCheckbox({});

  console.log($state.current_user_prompt);
  let credential_offer: any[] = $state.current_user_prompt.credential_offer;
  console.log(credential_offer);

  let all_offer_indices = credential_offer.credentials.map((_, i) => i);
</script>

<div class="content-height flex flex-col items-stretch bg-neutral-100">
  <TopNavigation title={'Credential Offer'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-6">
    <PaddedIcon icon={DownloadSimple} />
    <p class="text-2xl font-medium">{new URL(credential_offer.credential_issuer).hostname}</p>

    <!-- Text -->
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4">
      <!-- <WarningCircle class="mr-2 h-6 w-6 text-amber-500" /> -->
      <p class="text-sm font-medium text-slate-800">
        <span class="text-indigo-500">{credential_offer.credential_issuer}</span>
        offers you the following credentials
      </p>
    </div>

    <!-- Details -->
    <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white">
      {#each credential_offer.credentials as credential, i}
        <div class="flex justify-between rounded-lg bg-white px-4 py-4">
          <p>{credential.credential_definition.type.at(-1)}</p>
          <button
            use:melt={$root}
            class="flex h-6 w-6 appearance-none items-center justify-center
              rounded-md border-[1.5px] border-[#C5C6CC] p-[6px] text-white
              {$isChecked ? 'border-none bg-indigo-500' : 'bg-white'}"
            id="checkbox"
          >
            {#if $isChecked}
              <Check class="h-3 w-3" />
            {/if}
            <input use:melt={$input} />
          </button>
        </div>
      {/each}
    </div>
  </div>

  <!-- Controls -->
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 pb-0">
    <Button
      label="Accept credentials"
      on:click={() => {
        dispatch({
          type: '[Offer] Credential offers selected',
          payload: { offer_indices: all_offer_indices }
        });
      }}
      disabled={!$isChecked}
    />
    <Button
      label="Reject"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}
    />
    <!-- <button class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white" on:click={() => {}}
        >Accept connection</button
      > -->
    <!-- <button
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}>Reject</button
    > -->
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
