<script lang="ts">
  import {
    AlertDialog,
    AlertDialogTrigger,
    AlertDialogContent,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogCancel,
    AlertDialogAction,
    Checkbox,
    Button,
    Accordion,
    AccordionItem,
    AccordionTrigger,
    AccordionContent
  } from '@impierce/ui-components';
  import { state } from '../../stores';
  import LL from '$i18n/i18n-svelte';
  import { CheckBadge } from 'svelte-heros-v2';
  import { dispatch } from '$lib/dispatcher';
  import CredentialSubject from './CredentialSubject.svelte';
  import CredentialOffer from './CredentialOffer.svelte';

  export let isOpen: boolean;

  export let type: 'select-credentials' | 'credential-offer';
  export let title: string;
  export let imageSrc: string | undefined = undefined;
  export let options: any[];

  let selected = {}; // {"0": true, "1": false, "2": true"}

  // Set all credentials to be active by default
  if (type === 'select-credentials') {
    options.forEach((option) => (selected[option] = true));
  }

  // TODO: refactor
  if (type === 'credential-offer') {
    options = JSON.parse(decodeURI(options.at(0)));
    options.credentials.forEach((vc, i) => (selected[i] = true));
  }
</script>

<AlertDialog bind:open={isOpen}>
  <!-- <AlertDialogTrigger>
    <button>Open</button>
  </AlertDialogTrigger> -->
  <AlertDialogContent>
    <AlertDialogHeader>
      <AlertDialogTitle>{title}</AlertDialogTitle>
      <AlertDialogDescription>
        <div class="flex flex-col space-y-4">
          <!-- Image -->
          {#if imageSrc}
            <div class="mx-auto my-4 w-[160px]">
              <img src={imageSrc} alt="user-dialog" />
            </div>
          {/if}

          <!-- Content -->
          {#if type === 'select-credentials'}
            <!-- Issuer (Domain) -->
            <div class="flex w-full px-4">
              <div class="grow items-center justify-center rounded-full bg-slate-100 p-2">
                <p class="font-medium text-slate-400">example.com</p>
              </div>
              <CheckBadge
                size="32"
                class="ml-2 text-emerald-400 opacity-80"
                strokeWidth="2"
                variation="solid"
              />
            </div>

            <!-- Credentials -->
            <div class="space-y-4 p-4">
              {#each options as option}
                <button
                  class="flex items-center"
                  on:click={() => (selected[option] = !selected[option])}
                >
                  <div
                    class="flex w-full grow items-center justify-between rounded-lg bg-slate-100 p-4"
                  >
                    <div class="flex pr-4">
                      <Checkbox
                        id={`option-${option}`}
                        class="h-6 w-6 rounded-full"
                        bind:checked={selected[option]}
                      />
                      <!-- <label
                        for={option}
                        class="px-3 font-medium leading-none text-slate-300 peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                      >
                        {option.at(0)}
                      </label> -->
                    </div>
                    <CredentialSubject data={$state.credentials?.at(option)?.credentialSubject} />
                    <!-- <div>{JSON.stringify($state.credentials?.at(option)?.credentialSubject)}</div> -->
                    <!-- <Accordion type="single" collapsible class="w-full">
                      <AccordionItem value="item-1">
                        <AccordionTrigger>{$state.credentials[option].type[1]}</AccordionTrigger>
                        <AccordionContent>Here goes the content of the credential</AccordionContent>
                      </AccordionItem>
                    </Accordion> -->
                    <!-- {$state?.credentials?.at(0)?.credentialSubject?.[option.at(0)]} -->
                  </div>
                  <!-- <CheckBadge
                  size="32"
                  class="ml-2 text-emerald-400 opacity-80"
                  strokeWidth="2"
                  variation="solid"
                /> -->
                </button>
              {/each}
            </div>
          {/if}

          {#if type === 'credential-offer'}
            <!-- Issuer (Domain) -->
            <div class="flex w-full px-4">
              <div class="grow items-center justify-center rounded-full bg-slate-100 p-2">
                <p class="font-medium text-slate-400">{options.credential_issuer}</p>
              </div>
              <CheckBadge
                size="32"
                class="ml-2 text-emerald-400 opacity-80"
                strokeWidth="2"
                variation="solid"
              />
            </div>

            <!-- Credentials -->
            <!-- <div class="flex overflow-y-scroll snap-x"> -->
            <div class="flex flex-col space-y-4">
              {#each options.credentials as offer, i}
                <!-- <div class="break-all rounded bg-slate-100 px-4 py-2 w-[300px] snap-center"> -->
                <div class="flex items-center pr-4">
                  <div class="flex pr-6">
                    <Checkbox
                      id={`offer-${i}`}
                      class="h-6 w-6 rounded-full"
                      bind:checked={selected[i]}
                    />
                  </div>
                  <div class="break-all rounded bg-slate-200 px-4 py-2 w-full">
                    <CredentialOffer data={offer.credential_definition} />
                  </div>
                </div>
              {/each}
            </div>
          {/if}
        </div>
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel
        on:click={(e) => {
          e.preventDefault();
          console.log('cancel');
        }}
      >
        <!-- TODO: bug in shadcn-svelte: "Alert Dialog does not bind to on:click", https://github.com/huntabyte/shadcn-svelte/issues/137 -->
        <Button
          variant="destructive"
          on:click={() => dispatch({ type: '[User Flow] Cancel' })}
          class="w-full">{$LL.CANCEL()}</Button
        >
      </AlertDialogCancel>
      <AlertDialogAction
        disabled={false}
        on:click={(event) => {
          event.preventDefault();
          console.log('action');
        }}
      >
        <!-- TODO: bug in shadcn-svelte: "Alert Dialog does not bind to on:click", https://github.com/huntabyte/shadcn-svelte/issues/137 -->
        {#if type === 'select-credentials'}
          <Button
            on:click={() =>
              dispatch({
                type: '[Authenticate] Credentials selected',
                payload: { credential_index: 0 }
              })}>{$LL.SHARE_CREDENTIALS_CONFIRM()}</Button
          >
        {/if}
        {#if type === 'credential-offer'}
          <Button
            on:click={() =>
              dispatch({
                type: '[Credential Offer] Credentials selected',
                payload: { credential_index: 0 }
              })}>Accept</Button
          >
        {/if}
      </AlertDialogAction>
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
