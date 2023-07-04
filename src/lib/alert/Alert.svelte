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
    Button
  } from '@impierce/ui-components';
  import { state } from '../../stores';
  import LL from '../../i18n/i18n-svelte';
  import { CheckBadge } from 'svelte-heros-v2';
  import { dispatch } from '$lib/dispatcher';

  export let isOpen: boolean;
  export let title: string;
  export let options: string[];
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
          <img
            src="image/undraw_fingerprint_login_re_t71l.svg"
            alt="undraw_fingerprint"
            class="mx-auto my-4 w-[180px]"
          />
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

          <div class="space-y-4 p-4">
            {#each options as option, i}
              <div class="flex items-center">
                <div
                  class="flex w-[1px] grow items-center justify-between rounded-lg bg-slate-100 p-4"
                >
                  <div class="flex">
                    <Checkbox id={`${i}-${option}`} />
                    <label
                      for={`${i}-${option}`}
                      class="px-3 font-medium leading-none text-slate-300 peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                      {option.at(0)}
                    </label>
                  </div>
                  <div class="truncate font-semibold">
                    {$state?.credentials?.at(0)?.credentialSubject?.[option.at(0)]}
                  </div>
                </div>
                <!-- <CheckBadge
                  size="32"
                  class="ml-2 text-emerald-400 opacity-80"
                  strokeWidth="2"
                  variation="solid"
                /> -->
              </div>
            {/each}
          </div>
        </div>
      </AlertDialogDescription>
    </AlertDialogHeader>
    <AlertDialogFooter>
      <AlertDialogCancel on:click={(e) => {e.preventDefault(); console.log('cancel')}}>
        <!-- <Button on:click={() => dispatch({ type: '[User Flow] Cancel' })} class="w-full">button_text</Button> -->
        {$LL.CANCEL()}
      </AlertDialogCancel>
      <AlertDialogAction
        disabled={true}
        on:click={(event) => {
          event.preventDefault();
          console.log('action');
        }}>{$LL.SHARE_CREDENTIALS_CONFIRM()}</AlertDialogAction
      >
    </AlertDialogFooter>
  </AlertDialogContent>
</AlertDialog>
