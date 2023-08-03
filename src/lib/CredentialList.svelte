<script lang="ts">
  import { state } from '../stores';
  import LL from '../i18n/i18n-svelte';
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
    Input,
    SearchInput,
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
  import Car from '~icons/lucide/car';
  import Clapperboard from '~icons/lucide/clapperboard';
  import MailCheck from '~icons/lucide/mail-check';
  import GraduationCap from '~icons/lucide/graduation-cap';
  import HomeLucide from '~icons/lucide/home';
  import Percent from '~icons/lucide/percent';
  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z';
  import { fade, fly } from 'svelte/transition';

  // TODO: improve typing
  let credentials: any[] = [];

  // Does this really have to be reactive?
  $: credentials = $state?.credentials ?? [];
</script>

<!-- List of existing credentials -->
{#if credentials.length > 0}
  <!-- Search -->
  <!-- <Input type="text" placeholder="Search credentials" class="focus-visible:ring-violet-600" /> -->

  <!-- Credentials (list) -->
  <div class="flex w-full flex-col space-y-2">
    <div class="flex pb-4">
      <div class="grow">
        <SearchInput placeholder="Search credentials" />
      </div>
      <button class="ml-2 rounded-full p-2 hover:bg-slate-100"
        ><ArrowDownAZ class="text-slate-500" /></button
      >
    </div>

    <!--Mock credentials -->
    <p class="font-semibold">A</p>
    <CredentialListEntry title="Address of residence" description="State of Pandora">
      <span slot="icon"><HomeLucide class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>

    <p class="font-semibold">B</p>
    <CredentialListEntry title="Bachelor of Science" description="University of Pandora">
      <span slot="icon"><GraduationCap class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>

    <p class="font-semibold">D</p>
    <CredentialListEntry title="Discount - 20%" description="Home Supplies & Gardening">
      <span slot="icon"><Percent class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>
    <CredentialListEntry title="Driver's license" description="State of Pandora">
      <span slot="icon"><Car class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>

    <p class="font-semibold">E</p>
    <CredentialListEntry title="Email address" description="Pandora Email Service">
      <span slot="icon"><MailCheck class="h-6 w-6 text-violet-500" /></span>
    </CredentialListEntry>

    <!-- Actual credentials -->
    <p class="font-semibold">P</p>
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
<div>
  <Sheet>
    <SheetTrigger class="w-full">
      <button
        class="absolute bottom-4 right-4 flex justify-center rounded-full bg-violet-500 p-4 shadow-neon dark:bg-slate-800"
      >
        <Plus class="text-white" strokeWidth="3" />
        <!-- <p class="pl-1 font-semibold text-white uppercase text-lg">add</p> -->
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
