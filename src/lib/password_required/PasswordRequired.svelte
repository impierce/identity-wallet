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
    import LL from '../../i18n/i18n-svelte';
    import { CheckBadge } from 'svelte-heros-v2';
    import { dispatch } from '$lib/dispatcher';
  
    export let isOpen: boolean;
  
    let selected: number[] = [];
  </script>
  
  <AlertDialog bind:open={isOpen}>
    <!-- <AlertDialogTrigger>
      <button>Open</button>
    </AlertDialogTrigger> -->
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>{"Enter pasword"}</AlertDialogTitle>
        <AlertDialogDescription>
          <div class="flex flex-col space-y-4">
            <img
              src="image/undraw_fingerprint_login_re_t71l.svg"
              alt="undraw_fingerprint"
              class="mx-auto my-4 w-[180px]"
            />
            <div class="flex w-full px-4">
              <CheckBadge
                size="32"
                class="ml-2 text-emerald-400 opacity-80"
                strokeWidth="2"
                variation="solid"
              />
            </div>
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
          <Button variant="destructive" on:click={() => dispatch({ type: '[User Flow] Cancel' })}
            >{$LL.CANCEL()}</Button
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
          <Button
            on:click={() =>
              dispatch({
                type: '[Storage] Unlock',
                payload: { password: 'my-password' }
              })}>{"Unlock"}</Button
          >
        </AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
  