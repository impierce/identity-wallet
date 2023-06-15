<script lang="ts">
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import {
    Avatar,
    BottomNavigation,
    Button,
    CredentialListEntry,
    Input,
    Label
  } from '@impierce/ui-components';
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
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger
  } from '@impierce/ui-components';
  import { Plus, XMark, AtSymbol, Phone, Home, Cake, AcademicCap } from 'svelte-heros-v2';
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';
  import { fade, fly, slide } from 'svelte/transition';

  let initials: string | undefined;

  let credentials: any[] = [];

  $: {
    // TODO: needs to be called at least once to trigger subscribers --> better way to do this?
    console.log('state', $state);
    let names = $state?.active_profile?.display_name.split(' ');
    if (names?.length === 1) {
      initials = names?.at(0)?.slice(0, 2).toUpperCase();
    } else {
      let first = names?.at(0)?.charAt(0) ?? '?';
      let last = names?.at(1)?.charAt(0) ?? '?';
      // initials = names?.at(0)?.charAt(0) ?? '' + names?.at(1)?.charAt(0) ?? '';
      initials = first + '' + last;
    }
    console.log('initials', initials);
    credentials = $state?.credentials ?? [];
  }
</script>

<div class="min-h-screen">
  <!-- Background -->
  <!-- "absolute -z-10 w-full opacity-60" -->
  <img
    src="blob-scene-haikei-slate.png"
    alt="background-blob-scene"
    class="absolute w-full opacity-50"
  />
  <!-- <div class="absolute -z-10 bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500"></div> -->
  <!-- Content overlay -->
  <div class="absolute bottom-0 h-5/6 w-full rounded-t-3xl bg-white" in:fly={{ y: 24, opacity: 1 }}>
    <div class="relative bottom-12 -mb-6 flex justify-center" in:fly={{ y: 12, opacity: 1 }}>
      <Avatar {initials} size="large" />
    </div>
    <div class="flex justify-center text-2xl font-semibold">
      {$state?.active_profile?.display_name}
    </div>

    <div class="flex flex-col space-y-8 p-8">
      <!-- <h1 class="font-serif text-2xl font-semibold">
        {$LL.WELCOME()}, {$state?.active_profile?.display_name}!
      </h1> -->
      <div class="rounded-lg bg-slate-200 p-6">
        <p class="pb-4 font-semibold text-slate-500">{$LL.CREATE_IDENTITY_SUCCESS_TITLE()}</p>
        <p class="text-slate-400">{$LL.CREATE_IDENTITY_SUCCESS_BODY()}</p>
      </div>
      <!-- <button class="flex w-full justify-center rounded-lg bg-slate-200 p-6">
        <Plus class="text-violet-700" strokeWidth="2" />
      </button> -->

      <div>
        {#if credentials.length > 0}
          <div class="flex flex-col space-y-4">
            {#each credentials as credential}
              <AlertDialog>
                <AlertDialogTrigger>
                  <CredentialListEntry
                    title={credential?.credentialSubject?.degree?.name}
                    description={credential?.issuer?.name}
                  >
                    <span slot="icon"><AcademicCap class="text-violet-500" /></span>
                  </CredentialListEntry>
                </AlertDialogTrigger>
                <AlertDialogContent>
                  <AlertDialogHeader>
                    <AlertDialogTitle
                      >{credential?.credentialSubject?.degree?.name}</AlertDialogTitle
                    >
                    <AlertDialogDescription>
                      <div class="">
                        {JSON.stringify(credential, null, 2)}
                      </div>
                    </AlertDialogDescription>
                  </AlertDialogHeader>
                  <AlertDialogFooter>
                    <AlertDialogCancel>Cancel</AlertDialogCancel>
                    <AlertDialogAction>Share</AlertDialogAction>
                  </AlertDialogFooter>
                </AlertDialogContent>
              </AlertDialog>
            {/each}
          </div>
        {/if}
      </div>

      <Sheet>
        <SheetTrigger>
          <!-- <Button>Add info (sheet)</Button> -->
          <button class="flex w-full justify-center rounded-lg bg-violet-500 p-4">
            <Plus class="text-slate-200" strokeWidth="2" />
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
            <!-- <div class="grid grid-cols-4 items-center gap-4">
            <Label for="name" class="text-right">Name</Label>
            <Input id="name" value="Pedro Duarte" class="col-span-3" />
          </div>
          <div class="grid grid-cols-4 items-center gap-4">
            <Label for="username" class="text-right">Username</Label>
            <Input id="username" value="@peduarte" class="col-span-3" />
          </div> -->
          </div>

          <!-- <SheetFooter>
            <SheetClose>
              <Button type="submit">Save changes</Button>
            </SheetClose>
          </SheetFooter> -->
        </SheetContent>
      </Sheet>
    </div>
  </div>
  <!-- Navigation -->
  <div class="sticky top-[100vh]">
    <BottomNavigation
      active="profile"
      on:settings={() => goto('/settings')}
      on:history={() => goto('/history')}
    />
  </div>
</div>
