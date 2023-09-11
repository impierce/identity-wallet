<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { TopNavigation } from '@impierce/ui-components';
  import { createCheckbox, createPopover, melt } from '@melt-ui/svelte';

  import Button from '$lib/components/Button.svelte';
  import PaddedIcon from '$lib/components/PaddedIcon.svelte';
  import { colors, icons } from '$lib/credentials/customization/utils';
  import { dispatch } from '$lib/dispatcher';
  import CredentialOfferEntry from '$src/lib/components/CredentialOfferEntry.svelte';
  import { state } from '$src/stores';

  import Check from '~icons/ph/check-bold';
  import DownloadSimple from '~icons/ph/download-simple-fill';
  import PlugsConnected from '~icons/ph/plugs-connected-fill';
  import Question from '~icons/ph/question';
  import WarningCircle from '~icons/ph/warning-circle-fill';
  import X from '~icons/ph/x-bold';

  // const {
  //   elements: { root, input },
  //   helpers: { isChecked }
  // } = createCheckbox({});

  console.log($state.current_user_prompt);
  let credential_offer: any[] = $state.current_user_prompt.credential_offer;
  console.log({ credential_offer });

  // const checkboxes = credential_offer.credentials.map((c, i) => {
  //   const {
  //     elements: { root, input },
  //     helpers: { isChecked }
  //   } = createCheckbox();
  //   return { c, root, input, isChecked };
  // });

  // console.log(checkboxes);

  // const map = credential_offer.credentials.map((credential, i) => {
  //   const {
  //     elements: { root, input },
  //     helpers: { isChecked }
  //   } = createCheckbox();
  //   // return { credential, root, input, isChecked };
  //   return { credential };
  // });

  // console.log({ map });

  let all_offer_indices = credential_offer.credentials.map((_, i) => i);
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavigation title={'Credential Offer'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    {#if $state.current_user_prompt.logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver">
        <div class="flex overflow-hidden rounded-2xl">
          <img src={$state.current_user_prompt.logo_uri} alt="logo" />
        </div>
      </div>
    {:else}
      <PaddedIcon icon={DownloadSimple} />
    {/if}
    <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-white">
      {#if $state.current_user_prompt.issuer_name}
        {$state.current_user_prompt.issuer_name}
      {:else}
        {new URL(credential_offer.credential_issuer).hostname}
      {/if}
    </p>

    <!-- Text -->
    <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
      <!-- <WarningCircle class="mr-2 h-6 w-6 text-amber-500" /> -->
      <p class="text-sm font-medium text-slate-800 dark:text-slate-300">
        <span class="text-primary">{credential_offer.credential_issuer}</span>
        is offering you the following credentials
      </p>
    </div>

    <!-- Details -->
    <!-- <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white">
      {#each credential_offer.credentials as credential, i}
        <div class="flex justify-between rounded-lg bg-white px-4 py-4">
          <p>{credential.credential_definition.type.at(-1)}</p>
          <button
            use:melt={checkboxes.at(i).$root}
            class="flex h-6 w-6 appearance-none items-center justify-center
              rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
              {checkboxes.at(i).isChecked ? 'border-none bg-primary' : 'bg-white'}"
            id="checkbox"
          >
            {#if checkboxes.at(i).$isChecked}
              <Check class="h-3 w-3" />
            {/if}
            <input use:melt={checkboxes.at(i).$input} />
          </button>
        </div>
      {/each}
    </div> -->

    <div
      class="mt-3 w-full rounded-[20px] border border-slate-200 bg-white p-[10px] dark:border-slate-600 dark:bg-dark"
    >
      <!-- <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white"> -->
      {#each credential_offer.credentials as credential, index}
        <CredentialOfferEntry
          {index}
          title={credential.credential_definition.type.at(-1)}
          color={'bg-grey'}
        >
          <span slot="logo" class="p-1">
            <!-- {#if $state.current_user_prompt.logo_uri}
              <img src={$state.current_user_prompt.logo_uri} alt="logo" class="object-scale-down" />
            {:else} -->
            <svelte:component this={icons['User']} class="h-[18px] w-[18px] text-slate-800" />
            <!-- {/if} -->
          </span>
        </CredentialOfferEntry>
      {/each}
    </div>

    <!-- <div class="w-full space-y-2 rounded-2xl p-3 ring-2 ring-inset ring-white">
      {#each checkboxes as { c, root, input, isChecked }, i}
        <div class="flex justify-between rounded-lg bg-white px-4 py-4">
          <p>{credential_offer.at(i).credential.credential_definition.type.at(-1)}</p>
          <button
            use:melt={root}
            class="flex h-6 w-6 appearance-none items-center justify-center
              rounded-md border-[1.5px] border-slate-300 p-[6px] text-white
              {isChecked ? 'border-none bg-primary' : 'bg-white'}"
            id="checkbox"
          >
            {#if isChecked}
              <Check class="h-3 w-3" />
            {/if}
            <input use:melt={input} />
          </button>
        </div>
      {/each}
    </div> -->
  </div>

  <!-- Controls -->
  <div
    class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark"
  >
    <Button
      label="Accept credentials"
      on:click={() => {
        dispatch({
          type: '[Credential Offer] Selected',
          payload: { offer_indices: all_offer_indices }
        });
      }}
    />
    <Button
      label="Reject"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel' });
        goto('/me');
      }}
    />
    <!-- <button class="w-full rounded-lg bg-primary px-4 py-2 text-white" on:click={() => {}}
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
