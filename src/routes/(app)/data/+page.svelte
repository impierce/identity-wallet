<script lang="ts">
  import { state } from '../../../stores';
  import LL from '../../../i18n/i18n-svelte';
  import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
    AlertDialogTrigger,
    Button,
    CredentialListEntry,
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger
  } from '@impierce/ui-components';
  import CredentialDetails from '$lib/CredentialDetails.svelte';
  import { AtSymbol, Cake, Home, Phone, Plus, User } from 'svelte-heros-v2';

  // TODO: improve typing
  let credentials: any[] = [];

  // Does this really have to be reactive?
  $: credentials = $state?.credentials ?? [];
</script>

<div class="flex h-full flex-col items-center justify-center p-8 bg-orange-100">
  <!-- List of existing credentials -->
  {#if credentials.length > 0}
    <!-- Search -->
    <!-- <Input type="text" placeholder="Search credentials" class="focus-visible:ring-violet-600" /> -->

    <!-- Credentials (list) -->
    <div class="flex flex-col space-y-2 w-full pb-8">
      {#each credentials as credential}
        <AlertDialog>
          <AlertDialogTrigger>
            <div class="">
              <CredentialListEntry
                title={credential?.type?.at(1)}
                description={credential?.issuer?.name}
              >
                <span slot="icon"><User class="text-violet-500" /></span>
              </CredentialListEntry>
            </div>
          </AlertDialogTrigger>
          <AlertDialogContent>
            <AlertDialogHeader>
              <AlertDialogTitle>{credential?.type?.at(1)}</AlertDialogTitle>
              <AlertDialogDescription>
                <CredentialDetails {credential} />
              </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
              <AlertDialogCancel>Close</AlertDialogCancel>
              <AlertDialogAction>Ok</AlertDialogAction>
            </AlertDialogFooter>
          </AlertDialogContent>
        </AlertDialog>
      {/each}
    </div>
  {:else}
    <!-- Hint -->
    <div class="select-none rounded-lg bg-slate-200 p-6">
      <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p>
      <p class="text-slate-400">{$LL.CREATE_IDENTITY_SUCCESS_BODY()}</p>
    </div>
  {/if}

  <!-- Add new credential -->
  <Sheet>
    <SheetTrigger class="w-full">
      <button class="fixed bottom-8 flex justify-center rounded-full bg-violet-500 p-4 dark:bg-slate-800">
        <Plus class="text-white" strokeWidth="3" />
      </button>
    </SheetTrigger>
    <SheetContent position="bottom" size="content">
      <SheetHeader>
        <SheetTitle>Add information</SheetTitle>
        <SheetDescription>Choose a piece of information you'd like to add.</SheetDescription>
      </SheetHeader>

      <div class="grid grid-cols-2 gap-4 py-4">
        <Button variant="secondary"
          ><AtSymbol class="mr-2 text-slate-400" variation="solid" size="16" />Email</Button
        >
        <Button variant="secondary"
          ><Phone class="mr-2 text-slate-400" variation="solid" size="16" />Phone</Button
        >
        <Button variant="secondary"
          ><Home class="mr-2 text-slate-400" variation="solid" size="16" />Address</Button
        >
        <Button variant="secondary"
          ><Cake class="mr-2 text-slate-400" variation="solid" size="16" />Date of Birth</Button
        >
        <Button variant="outline">Custom</Button>
      </div>

      <!-- <SheetFooter>
        <SheetClose>
          <Button type="submit">Save changes</Button>
        </SheetClose>
      </SheetFooter> -->
    </SheetContent>
  </Sheet>
</div>
