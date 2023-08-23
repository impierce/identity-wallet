<script lang="ts">
  import { state } from '$src/stores';
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
  import { AtSymbol, Cake, Home, Phone, Plus } from 'svelte-heros-v2';
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
  import LL from '$src/i18n/i18n-svelte';
  import User from '~icons/ph/user';

  import House from '~icons/ph/house-light';
  import NoCredentials from './credentials/NoCredentials.svelte';
  import CredentialListEntry from './components/CredentialListEntry.svelte';
  import type { TransferState } from '../../src-tauri/bindings/TransferState';

  // TODO: improve typing
  let credentials: TransferState['credentials'] = [];

  let test_credentials = [
    {
      title: 'Address of residence',
      description: 'State of Pandora',
      color: 'bg-[#ffe4e6]',
      icon: House
    },
    {
      title: 'Bachelor of Science',
      description: 'University of Pandora',
      color: 'bg-blue-100',
      icon: GraduationCap
    },
    {
      title: 'Discount - 20%',
      description: 'Home Supplies & Gardening',
      color: 'bg-orange-100',
      icon: Percent
    },
    {
      title: "Driver's license",
      description: 'State of Pandora',
      color: 'bg-emerald-100',
      icon: Car
    },
    {
      title: 'Email address',
      description: 'Pandora Email Service',
      color: 'bg-slate-400',
      icon: EnvelopeSimple
    }
  ];

  test_credentials = [];

  // Does this really have to be reactive?
  $: credentials = $state?.credentials ?? [];
</script>

<!-- List of existing credentials -->
{#if credentials?.length > 0}
  <div class="flex items-center pb-2">
    <SealCheck class="mr-2 text-indigo-500" />
    <p class="text-[15px]/[24px] font-medium text-slate-600">{$LL.MY_DATA()}</p>
  </div>
  <!-- Search -->
  <!-- <Input type="text" placeholder="Search credentials" class="focus-visible:ring-violet-600" /> -->

  <!-- Credentials (list) -->
  <div class="flex flex-col space-y-2">
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
    {#each test_credentials as credential}
      <CredentialListEntry
        title={credential.title}
        description={credential.description}
        color={credential.color}
      >
        <span slot="icon">
          <svelte:component this={credential.icon} class="h-[18px] w-[18px] text-slate-800" />
        </span>
      </CredentialListEntry>
    {/each}

    <!-- Actual (non-mock) credentials -->
    {#each credentials as credential}
      <CredentialListEntry
        id={credential?.at(0)}
        title={credential?.at(1).type.at(1)}
        description={credential?.at(1).issuer}
        color="bg-indigo-100"
      >
        <span slot="icon"><User class="h-[18px] w-[18px] text-slate-800" /></span>
      </CredentialListEntry>
    {/each}
  </div>
{:else}
  <NoCredentials />
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
