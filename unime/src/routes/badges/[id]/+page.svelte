<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import { page } from '$app/stores';
  import MarkdownIt from 'markdown-it';
  import QRCode from 'qrcode';
  import { fly } from 'svelte/transition';

  import { melt } from '@melt-ui/svelte';
  import { dataDir } from '@tauri-apps/api/path';

  import { dispatch } from '$lib/dispatcher';
  import { getImageAsset } from '$lib/utils';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import ButtonRounded from '$src/lib/components/atoms/ButtonRounded.svelte';
  import Image from '$src/lib/components/atoms/Image.svelte';
  import BottomDrawer from '$src/lib/components/molecules/dialogs/BottomDrawer.svelte';
  import TopNavBar from '$src/lib/components/molecules/navigation/TopNavBar.svelte';
  import CredentialDetailsDropdownMenu from '$src/lib/credentials/CredentialDetailsDropdownMenu.svelte';
  import { state } from '$src/stores';

  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  import Heart from '~icons/ph/heart-straight';
  import HeartFill from '~icons/ph/heart-straight-fill';
  import QrCode from '~icons/ph/qr-code';
  import SealCheck from '~icons/ph/seal-check';

  let credential = $state.credentials.find((c) => $page.params.id === c.id)!!;

  let icon: any = credential.metadata.display.icon || 'User';
  let title: string = credential.metadata.display.name || credential.data.type.at(-1);

  let credentialLogoUrl: string | null;

  // TODO: add issuer id which can then be used to link to the connection
  let issuerId: string | undefined;

  let qrcodeText = JSON.stringify(credential, null, 0);

  let isFavorite: boolean = credential.metadata.is_favorite;

  const markdown = new MarkdownIt();

  $: {
    const credential = $state.credentials.find((c) => $page.params.id === c.id)!!;
    // TODO: update icon, title, isFavorite when changes in store
    isFavorite = credential.metadata.is_favorite;
    title = credential.metadata.display.name || credential.data.type.at(-1);
    icon = credential.metadata.display.icon || 'User';
  }

  const hiddenStandardFields: string[] = ['id', 'type', 'name', 'description', 'image'];
  const hiddenCustomFields: string[] = ['enrichment'];

  const entries = { ...credential.data.credentialSubject.achievement };
  hiddenStandardFields.concat(hiddenCustomFields).forEach((key) => delete entries[key]);

  console.log({ credential });

  onMount(async () => {
    credentialLogoUrl = await getImageAsset($page.params.id!!);
  });
</script>

<div class="content-height relative flex w-full flex-col">
  <!-- TODO: allow overriding the color of the TopNavBar -->
  <TopNavBar
    title="Badge information"
    on:back={() => history.back()}
    class="bg-white text-slate-800 dark:bg-dark dark:text-grey"
  />
  <div class="hide-scrollbar grow overflow-y-scroll bg-white dark:bg-dark">
    <!-- Logo -->
    <div class="relative flex flex-col overflow-hidden bg-silver px-[15px] py-[20px] dark:bg-navy">
      {#if credentialLogoUrl}
        <img
          src={credentialLogoUrl}
          alt=""
          class="absolute -top-1/4 left-0 scale-[1.75] opacity-40 blur-xl"
          on:error={() => (credentialLogoUrl = null)}
        />
      {/if}
      <div class="z-10 flex items-start justify-between px-2">
        <button
          class="-ml-1 -mt-1 rounded-full p-1"
          on:click={() =>
            dispatch({
              type: '[Credential Metadata] Update',
              payload: {
                id: credential.id,
                is_favorite: !isFavorite,
              },
            })}
        >
          {#if isFavorite}
            <HeartFill class="h-6 w-6 dark:text-white" />
          {:else}
            <Heart class="h-6 w-6 dark:text-white" />
          {/if}
        </button>
        <div class="mr-2 flex h-[165px] w-[165px] flex-col items-center justify-center rounded-3xl bg-white">
          <Image
            id={$page.params.id}
            imgClass="h-[128px] w-[128px] rounded-xl"
            iconFallback="Certificate"
            iconClass="h-10 w-10 dark:text-slate-800"
          />
        </div>
        <div class="-mr-3 -mt-1">
          <!-- <CredentialDetailsDropdownMenu {credential} class="dark:text-white" /> -->
          <!-- Editing the appearance of a badge is not supported, therefore the menu is hidden -->
          <DotsThreeVertical class="m-1 h-6 w-6 opacity-0 dark:text-white" />
        </div>
      </div>
      <!-- Text -->
      <div class="z-10 flex flex-col items-center pt-[15px]">
        <p class="line-clamp-2 text-center text-[22px]/[30px] font-semibold tracking-tight text-black dark:text-white">
          {credential.data.credentialSubject.achievement?.name ?? title}
        </p>
      </div>
    </div>
    <!-- Text -->
    <div class="flex flex-col space-y-5 px-[15px] pb-[15px]">
      <div class="flex space-x-3 pt-8">
        <!-- Valid -->
        <div class="flex w-full flex-col items-center space-y-1">
          <p class="text-xs text-black dark:text-white">{$LL.BADGE.DETAILS.VALID()}</p>
          <div class="flex w-full justify-center rounded-xl bg-silver py-5 dark:bg-white">
            <SealCheck class="h-7 w-7 text-slate-800" />
          </div>
          <p class="text-xs text-black dark:text-white">
            {#if credential.data.issuanceDate}
              {new Date(credential.data.issuanceDate).toLocaleString($state.locale, {
                dateStyle: 'long',
                // timeStyle: 'medium',
              })}
            {/if}
          </p>
        </div>
        <!-- Issued by -->
        <div class="flex w-full flex-col items-center space-y-1">
          <p class="text-xs text-black dark:text-white">{$LL.BADGE.DETAILS.ISSUED_BY()}</p>
          <div class="flex h-[68px] w-full items-center justify-center rounded-xl bg-silver p-2 dark:bg-white">
            <Image
              id={issuerId}
              iconFallback="Bank"
              imgClass="w-auto rounded-lg m-2"
              iconClass="h-7 w-7 dark:text-slate-800"
            />
          </div>
          <p class="text-center text-xs text-black [word-break:break-word] dark:text-white">
            {credential.data.issuer.name ?? credential.data.issuer ?? credential.issuer_name}
          </p>
        </div>
      </div>

      <!-- Description -->
      <div>
        <!-- <p class="text-lg font-semibold text-black dark:text-white">{$LL.BADGE.DETAILS.DESCRIPTION()}</p> -->
        <p class="text-[13px]/[24px] text-slate-800 dark:text-grey">
          {@html markdown.render(credential.data.credentialSubject.achievement?.description ?? '')}
        </p>
      </div>

      <!-- Contents (Table: Credential Subject) -->
      <div>
        <p class="pb-2 text-lg font-semibold text-black dark:text-white">{$LL.BADGE.DETAILS.CONTENTS()}</p>
        <div
          class="divide-y divide-solid divide-slate-200 rounded-xl border border-slate-200 bg-white dark:divide-slate-600 dark:border-slate-600 dark:bg-dark"
        >
          {#each Object.entries(entries) as entry}
            <div class="flex flex-col items-start px-4 py-[10px]">
              <p class="text-[13px]/[24px] font-medium text-slate-500">{entry[0]}</p>
              <div class="w-full break-words text-[13px]/[24px] font-medium text-slate-800 dark:text-white">
                <!-- TODO: this is a hacky way to display nested data -->
                <pre class="whitespace-pre-wrap [font-family:inherit]">{JSON.stringify(entry[1], null, 2)}</pre>
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>

    {#if $state.dev_mode_enabled}
      <p class="break-all px-4 pb-5 text-center text-[13px]/[24px] text-slate-500">
        {JSON.stringify(credential.data.issuer)}
      </p>
    {/if}

    <div class="h-[var(--safe-area-inset-bottom)]"></div>
  </div>
  <!-- </div> -->
  <BottomDrawer>
    <!-- TODO: feature disabled: "Share credential" -->
    <!-- <ButtonRounded slot="trigger" let:trigger {trigger} label="Share" icon={QrCode} class="absolute bottom-4 right-4" /> -->
    <span slot="content" class="flex flex-col items-center justify-center">
      <!-- Logo -->
      <div class="flex h-[75px] w-[75px] flex-col items-center justify-center rounded-[20px] border-[5px] border-white">
        <svelte:component this={icon} class="h-6 w-6 text-slate-800" />
      </div>
      <!-- Description -->
      <div class="flex flex-col items-center">
        <p class="pt-4 text-2xl font-semibold text-black">{title}</p>
        <p class="text-[13px]/[24px] text-slate-500">
          {credential.issuer_name ?? credential.data.issuer}
        </p>
      </div>
      <!-- QR Code -->
      <div class="flex flex-col items-center p-7">
        <div class="rounded-2xl bg-white p-6">
          {#await QRCode.toDataURL(qrcodeText, { margin: 0, color: { light: '#FFFFFF' } }) then data_url}
            <img src={data_url} alt="qr-code" />
          {/await}
        </div>
        <p class="pt-5 text-xl font-semibold text-black">
          {Object.entries(credential.data.credentialSubject).at(-1).at(1)}
        </p>
      </div>
    </span>
    <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-2 w-full rounded-lg border bg-white px-4 py-2 text-neutral-700">Close</button
    >
  </BottomDrawer>
</div>

<div class="safe-area-top bg-white dark:bg-dark" />

<!-- <div class="safe-area-bottom z-10 bg-white dark:bg-dark" /> -->

<style>
  .content-height {
    /* bottom-navigation: 64px + 2 * 8px (y-padding) + 1px (border top) = 81px */
    height: calc(100vh - var(--safe-area-inset-top));
  }
</style>
