<script lang="ts">
  import { onDestroy, onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { createCheckbox, createPopover, melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import { getImageAsset } from '$lib/utils';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import Checkbox from '$src/lib/components/atoms/Checkbox.svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import PaddedIcon from '$src/lib/components/atoms/PaddedIcon.svelte';
  import ListItemCard from '$src/lib/components/molecules/ListItemCard.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
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

  // if ($state.current_user_prompt == null) {
  //   goto('/scan');
  // }

  console.log($state.current_user_prompt);
  // TODO: type
  let credential_offer: any = $state.current_user_prompt?.credential_offer ?? {};
  console.log({ credential_offer });

  let issuer_name = $state.current_user_prompt?.issuer_name;

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

  let issuerLogoUrl: string | null;
  // let credentialLogoUrl: string | null; // TODO: batch credentials, see "all_offer_indices"

  // let images = new Map<string, string>();

  let credentialLogoUrls = {};

  onMount(async () => {
    issuerLogoUrl = await getImageAsset('issuer_0', true);
    // credentialLogoUrl = await getImageAsset('credential', true);

    // $state?.current_user_prompt?.display?.forEach(async (display, i) => {
    //   if (display.logo_uri) {
    //     credentialLogoUrl = await getImageAsset('credential', true, display.logo_uri);
    //   }
    // });
    $state.current_user_prompt.display.forEach(async (display, i) => {
      if (display.logo?.url) {
        credentialLogoUrls[i] = await getImageAsset(`credential_${i}`, true);
      }
    });
  });

  onDestroy(async () => {
    dispatch({ type: '[User Flow] Cancel', payload: { } });
  });
</script>

<div class="content-height flex flex-col items-stretch bg-silver dark:bg-navy">
  <TopNavBar title={'Credential Offer'} on:back={() => history.back()} />

  <div class="flex grow flex-col items-center justify-center space-y-6 p-4">
    <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl">
      <!-- <div class="flex w-full items-center justify-center overflow-hidden rounded-2xl"> -->
      <!-- TODO: should fallback to <PaddedIcon> instead of icon -->

      <Image
        id={'issuer_0'}
        isTempAsset={true}
        iconClass="dark:text-slate-800"
        imgClass="flex w-full items-center justify-center overflow-hidden rounded-3xl p-2 object-contain"
      >
        <div slot="fallback">
          <PaddedIcon icon={DownloadSimple} />
        </div>
      </Image>
      <!-- </div> -->
    </div>
    <!-- {#if $state.current_user_prompt.logo_uri}
      <div class="flex h-[75px] w-[75px] overflow-hidden rounded-3xl bg-white p-2 dark:bg-silver">
        <div class="flex w-full items-center justify-center overflow-hidden rounded-2xl">
          {#if issuerLogoUrl}
            <img src={issuerLogoUrl} alt="logo" on:error={() => (issuerLogoUrl = null)} />
          {:else}
            <svelte:component this={icons['User']} class="h-6 w-6 text-slate-800" />
          {/if}
        </div>
      </div>
    {:else}
      <PaddedIcon icon={DownloadSimple} />
    {/if} -->
    <p class="text-[22px]/[30px] font-semibold text-slate-700 dark:text-grey">
      {#if issuer_name}
        {issuer_name}
      {:else}
        {new URL(credential_offer.credential_issuer).hostname}
      {/if}
    </p>

    <p class="w-full text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      is offering you the following credentials
    </p>

    <!-- Text -->
    <!-- <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
      <p class="text-sm font-medium text-slate-800 dark:text-slate-300">
        <span class="text-primary">{issuer_name}</span>
        is offering you the following credentials
      </p>
    </div> -->

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
        <!-- TODO: careful with long list! -->
        <ListItemCard
          id={`credential_${index}`}
          title={$state?.current_user_prompt?.display?.at(index)?.name ?? credential.credential_definition.type.at(-1)}
          isTempAsset={true}
        >
          <div slot="right" class="mr-2">
            <Checkbox checked={true} disabled={true} />
          </div>
        </ListItemCard>
        <!-- {#if !credentialLogoUrl}
            <div class="{color} relative h-[-webkit-fill-available] w-screen"></div>
          {:else}
            <img
              src={credentialLogoUrl}
              class="scale-[1.75] opacity-40 blur-xl"
              on:error={() => (credentialLogoUrl = '')}
            />
          {/if} -->
        <!-- <span slot="logo" class="flex h-full w-full items-center justify-center bg-silver p-1 dark:bg-navy">
            <Image id={'credential_0'} isTempAsset={true} /> -->

        <!-- {#if credentialLogoUrl}
              <img src={credentialLogoUrl} alt="logo" on:error={() => (credentialLogoUrl = null)} />
            {:else} -->

        <!-- {#if credentialLogoUrls[index]}
              <img
                src={credentialLogoUrls[index]}
                alt="logo-{index}"
                on:error={() => console.warn('error opening the image')}
              />
            {:else}
              <svelte:component this={icons['User']} class="h-[18px] w-[18px] text-slate-800 dark:text-grey" />
            {/if} -->
        <!-- </span> -->

        <!-- <span slot="logo">
            <img src={credentialLogoUrl} alt="logo" />
          </span> -->
        <!-- {#if $state.current_user_prompt.logo_uri}
              <img src={$state.current_user_prompt.logo_uri} alt="logo" class="object-scale-down" />
            {:else} -->
        <!-- <svelte:component this={icons['User']} class="h-[18px] w-[18px] text-slate-800" /> -->
        <!-- {/if} -->
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
  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label="Accept credentials"
      on:click={() => {
        dispatch({
          type: '[Credential Offer] Selected',
          payload: {
            offer_indices: all_offer_indices,
          },
        });
      }}
    />
    <Button
      label="Reject"
      variant="secondary"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { redirect: 'me' } });
      }}
    />
    <!-- <button class="w-full rounded-lg bg-primary px-4 py-2 text-white" on:click={() => {}}
        >Accept connection</button
      > -->
    <!-- <button
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700"
      on:click={() => {
        dispatch({ type: '[User Flow] Cancel', payload: { } });
        goto('/me');
      }}>Reject</button
    > -->
  </div>
</div>

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
