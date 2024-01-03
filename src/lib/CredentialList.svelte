<script lang="ts">
  import { onMount } from 'svelte';

  import { colors, icons } from '$lib/credentials/customization/utils';
  import LL from '$src/i18n/i18n-svelte';
  import { state } from '$src/stores';

  import Car from '~icons/ph/car-light';
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import GraduationCap from '~icons/ph/graduation-cap-light';
  import House from '~icons/ph/house-light';
  import Percent from '~icons/ph/percent-light';
  import SealCheck from '~icons/ph/seal-check-fill';

  import type { DisplayCredential } from '../../src-tauri/bindings/display-credential/DisplayCredential';
  import CredentialListEntry from './components/CredentialListEntry.svelte';
  import NoCredentials from './credentials/NoCredentials.svelte';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  // TODO: improve typing
  let credentials: Array<DisplayCredential> = []; // = $state.credentials.filter((c) => !c.metadata.is_favorite);
  // $: credentials = $state.credentials.filter((c) => !c.metadata.is_favorite);

  let test_credentials = [
    {
      title: 'Address of residence',
      description: 'State of Pandora',
      color: 'bg-yellow-100',
      icon: House,
    },
    {
      title: 'Bachelor of Science',
      description: 'University of Pandora',
      color: 'bg-blue-100',
      icon: GraduationCap,
      image: 'https://placehold.co/400',
    },
    {
      title: 'Discount - 20%',
      description: 'Home Supplies & Gardening',
      color: 'bg-orange-100',
      icon: Percent,
      image: 'https://placehold.co/32',
    },
    {
      title: "Driver's license",
      description: 'State of Pandora',
      color: 'bg-emerald-100',
      icon: Car,
      image: 'https://placehold.co/250x100',
    },
    {
      title: 'Email address',
      description: 'Pandora Email Service',
      color: 'bg-slate-400',
      icon: EnvelopeSimple,
    },
  ];

  test_credentials = [];

  // Does this really have to be reactive?
  // $: credentials = $state?.credentials ?? [];

  onMount(async () => {
    credentials = $state.credentials.filter((c) => !c.metadata.is_favorite);
    if (credentialType === 'badges') {
      credentials = credentials.filter((c) => (c.data.type as string[]).includes('OpenBadgeCredential'));
    } else if (credentialType === 'data') {
      credentials = credentials.filter((c) => !(c.data.type as string[]).includes('OpenBadgeCredential'));
    }
  });
</script>

<!-- List of existing credentials -->
{#if credentials?.length > 0}
  <!-- <div class="flex items-center pb-2">
    <SealCheck class="mr-2 text-primary" />
    <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-white">{$LL.MY_DATA()}</p>
  </div> -->

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
      <CredentialListEntry title={credential.title} description={credential.description} color={credential.color}>
        <span slot="icon" class="h-full">
          <!-- <img src={credential.image} class="h-full object-cover" /> -->
          <!-- <svelte:component this={credential.icon} class="h-[18px] w-[18px] text-slate-800" /> -->
        </span>
      </CredentialListEntry>
    {/each}

    <!-- Actual (non-mock) credentials -->
    {#each credentials as credential}
      <CredentialListEntry
        id={credential.id}
        title={credential.metadata.display.name ??
          credential.data.credentialSubject.achievement?.name ??
          credential.data.type.at(-1)}
        description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
        color={credential.metadata.display.color ||
          colors.at(
            credential.id
              .match(/[0-9]+/)
              .at(0)
              .at(0) % 8, // TODO: omits last value (white)
          )}
        type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
      >
        <span slot="icon">
          <svelte:component
            this={icons[credential.metadata.display.icon] ||
              (credential.data.type.includes('OpenBadgeCredential') ? icons['Certificate'] : icons['User'])}
            class="h-[18px] w-[18px] text-slate-800 dark:text-grey"
          />
        </span>
      </CredentialListEntry>
    {/each}
  </div>
{:else if $state?.credentials?.length === 0}
  <!-- Only show "No credentials" when there's also no favorites -->
  <NoCredentials />
{/if}

<!-- Add new credential -->
<!-- <div>
  <Sheet>
    <SheetTrigger class="w-full">
      <button
        class="absolute bottom-4 right-4 flex justify-center rounded-full bg-primary p-3 dark:bg-slate-800"
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
