<script lang="ts">
  import { onMount } from 'svelte';

  import { goto } from '$app/navigation';
  import type { DisplayCredential } from 'src-tauri/bindings/display-credential/DisplayCredential';

  import LL from '$src/i18n/i18n-svelte';
  import IconMessage from '$src/lib/components/molecules/IconMessage.svelte';
  import ListItemCard from '$src/lib/credentials/ListItemCard.svelte';
  import { state } from '$src/stores';

  import Car from '~icons/ph/car-light';
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import Ghost from '~icons/ph/ghost-fill';
  import GraduationCap from '~icons/ph/graduation-cap-light';
  import House from '~icons/ph/house-light';
  import Percent from '~icons/ph/percent-light';
  import SealCheck from '~icons/ph/seal-check-fill';

  export let credentialType: 'all' | 'data' | 'badges' = 'all';

  // TODO: improve typing
  let credentials: Array<DisplayCredential> = []; // = $state.credentials.filter((c) => !c.metadata.is_favorite);
  // $: credentials = $state.credentials.filter((c) => !c.metadata.is_favorite);

  // let test_credentials = [
  //   {
  //     title: 'Address of residence',
  //     description: 'State of Pandora',
  //     color: 'bg-yellow-100',
  //     icon: House,
  //   },
  //   {
  //     title: 'Bachelor of Science',
  //     description: 'University of Pandora',
  //     color: 'bg-blue-100',
  //     icon: GraduationCap,
  //     image: 'https://placehold.co/400',
  //   },
  //   {
  //     title: 'Discount - 20%',
  //     description: 'Home Supplies & Gardening',
  //     color: 'bg-orange-100',
  //     icon: Percent,
  //     image: 'https://placehold.co/32',
  //   },
  //   {
  //     title: "Driver's license",
  //     description: 'State of Pandora',
  //     color: 'bg-emerald-100',
  //     icon: Car,
  //     image: 'https://placehold.co/250x100',
  //   },
  //   {
  //     title: 'Email address',
  //     description: 'Pandora Email Service',
  //     color: 'bg-slate-400',
  //     icon: EnvelopeSimple,
  //   },
  // ];

  // test_credentials = [];

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

    <!-- Actual (non-mock) credentials -->
    {#each credentials as credential}
      <ListItemCard
        id={credential.id}
        title={credential.metadata.display.name ??
          credential.data.credentialSubject.achievement?.name ??
          credential.data.type.at(-1)}
        description={credential.issuer_name ?? credential.data.issuer?.name ?? credential.data.issuer}
        type={credential.data.type.includes('OpenBadgeCredential') ? 'badge' : 'data'}
        on:click={() =>
          credential.data.type.includes('OpenBadgeCredential')
            ? goto(`/badges/${credential.id}`)
            : goto(`/credentials/${credential.id}`)}
      ></ListItemCard>
    {/each}
  </div>
{:else if $state?.credentials?.length === 0}
  <!-- Only show "No credentials" when there's also no favorites -->
  <div class="flex grow flex-col items-center justify-center">
    <IconMessage icon={Ghost} title={$LL.EMPTY_CREDENTIALS_LIST_TITLE()} />
    <p class="w-[280px] pt-[15px] text-center text-[13px]/[24px] font-normal text-slate-500 dark:text-slate-300">
      Visit <span class="font-semibold text-primary">https://demo.ngdil.com</span> on a desktop computer to get started.
    </p>
  </div>
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
