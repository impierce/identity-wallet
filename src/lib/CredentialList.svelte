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
      image:
        'data:image/png;base64,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',
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
            this={icons[credential.metadata.display.icon] || icons['User']}
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
