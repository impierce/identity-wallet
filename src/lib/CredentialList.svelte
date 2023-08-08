<script lang="ts">
  import { state } from '../stores';
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
  import Car from '~icons/ph/car-light';
  import Clapperboard from '~icons/lucide/clapperboard';
  import MailCheck from '~icons/lucide/mail-check';
  import GraduationCap from '~icons/ph/graduation-cap-light';
  import HomeLucide from '~icons/lucide/home';
  import Percent from '~icons/ph/percent-light';
  import ArrowDownAZ from '~icons/lucide/arrow-down-a-z';
  import { fade, fly } from 'svelte/transition';
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import SealCheck from '~icons/ph/seal-check-fill';
  import LL from '$i18n/i18n-svelte';

  import House from '~icons/ph/house-light';

  // TODO: improve typing
  let credentials: any[] = [];

  // Does this really have to be reactive?
  $: credentials = $state?.credentials ?? [];
</script>

<!-- List of existing credentials -->
{#if credentials.length > 0}
  <div class="flex items-center pb-2">
    <SealCheck class="mr-2 text-indigo-500" />
    <p class="font-medium text-slate-600">{$LL.MY_DATA()}</p>
  </div>
  <!-- Search -->
  <!-- <Input type="text" placeholder="Search credentials" class="focus-visible:ring-violet-600" /> -->

  <!-- Credentials (list) -->
  <div class="flex w-full flex-col space-y-2">
    <!-- Search -->
    <!-- <div class="flex pb-4">
      <div class="grow">
        <SearchInput placeholder="Search credentials" />
      </div>
      <button class="ml-2 rounded-full p-2 hover:bg-slate-100"
        ><ArrowDownAZ class="text-slate-500" /></button
      >
    </div> -->

    <!--Mock credentials -->
    <!-- <p class="font-semibold">A</p> -->
    <CredentialListEntry
      title="Address of residence"
      description="State of Pandora"
      color="bg-[#ffe4e6]"
    >
      <!-- custom: bg-[#fecdd3] -->
      <span slot="icon"><House class="h-6 w-6" /></span>
    </CredentialListEntry>

    <!-- <p class="font-semibold">B</p> -->
    <CredentialListEntry
      title="Bachelor of Science"
      description="University of Pandora"
      color="bg-blue-100"
    >
      <span slot="icon"><GraduationCap class="h-6 w-6" /></span>
    </CredentialListEntry>

    <!-- <p class="font-semibold">D</p> -->
    <CredentialListEntry
      title="Discount - 20%"
      description="Home Supplies & Gardening"
      color="bg-orange-100"
    >
      <span slot="icon"><Percent class="h-6 w-6" /></span>
    </CredentialListEntry>
    <CredentialListEntry
      title="Driver's license"
      description="State of Pandora"
      color="bg-emerald-100"
    >
      <span slot="icon"><Car class="h-6 w-6" /></span>
    </CredentialListEntry>

    <!-- <p class="font-semibold">E</p> -->
    <CredentialListEntry
      title="Email address"
      description="Pandora Email Service"
      color="bg-slate-400"
    >
      <span slot="icon"><EnvelopeSimple class="h-6 w-6 text-slate-100" /></span>
    </CredentialListEntry>

    <!-- Actually working credentials -->
    <!-- <p class="font-semibold">P</p> -->
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
  <!-- No credentials yet -->
{/if}

<!-- Add new credential -->
<!-- <div>
  <Sheet>
    <SheetTrigger class="w-full">
      <button
        class="absolute bottom-4 right-4 flex justify-center rounded-full bg-indigo-500 p-3 dark:bg-slate-800"
      >
        <PlusCircle class="h-6 w-6 text-white" />
        <p class="pl-2 pr-1 text-base font-medium text-white">Add</p>
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
    </SheetContent>
  </Sheet>
</div> -->
