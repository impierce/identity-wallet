<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { createPopover, melt } from '@melt-ui/svelte';
  import { fade } from 'svelte/transition';
  import Plus from '~icons/ph/plus-bold';
  import LL from '$src/i18n/i18n-svelte';

  const {
    elements: { trigger, content, arrow, close },
    states: { open }
  } = createPopover({
    defaultOpen: true
  });

  // hex codes
  let predefinedEmojis: Array<string> = [
    '&#128522;', // smiling-face
    '&#129412;', // unicorn
    '&#129312;', // cowboy-hat-face
    '&#129321;', // star-struck
    '&#128053', // monkey-face
    '&#128048', // rabbit-face
    '&#128060', // panda-face
    '&#129418', // fox-face
    '&#128540' // twinking-face-with-tongue
  ];

  let emojiChoice: string | undefined = undefined;

  console.log($open);
</script>

<!-- <TopNavigation title="Avatar" on:back={() => history.back()} /> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      Set a display <span class="text-indigo-500">picture</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">Make it yours.</p>
    <div class="mt-[70px] flex w-full items-center justify-center">
      <BottomDrawer titleText={'Select profile picture'} descriptionText={''}>
        <!-- <span slot="trigger"> -->
        <button
          slot="trigger"
          let:trigger
          class="flex h-24 w-24 items-center justify-center rounded-full border border-slate-200 bg-white"
          use:melt={trigger}
        >
          {#if emojiChoice}
            <span class="text-[44px]/[44px]">
              {@html emojiChoice}
            </span>
          {:else}
            <Plus class="h-6 w-6" />
          {/if}
        </button>
        <!-- TODO: is never shown, because not in slot -->
        {#if $open}
          <div
            use:melt={$content}
            transition:fade={{ duration: 200 }}
            class="z-10 w-1/2 rounded-2xl bg-[#2F3036] p-[20px] text-[#D4D6DD] shadow-md"
          >
            <div use:melt={$arrow} />
            <div>
              <p class="text-[12px] font-semibold text-white">Add your profile image</p>
              <p class="pt-2 text-[11px]/[14px] font-normal">
                Customize your UniMe with your own picture or emoji.
              </p>
              <!-- <div class="break-keep text-sm">
              Your UniMe app automatically tries to verify the identity of <span
                class="underline underline-offset-2">BestDex</span
              >
              to provide you with a secure login.
            </div> -->
            </div>
          </div>
        {/if}
        <div slot="content" class="flex flex-col items-center space-y-4">
          <div
            class="grid w-fit grid-cols-3 place-items-center gap-2 rounded-3xl bg-neutral-100 p-2"
          >
            {#each predefinedEmojis as emoji}
              <button
                class="rounded-2xl border bg-white p-4 text-[32px]/[32px]"
                on:click={() => {
                  emojiChoice = emoji;
                }}
                >{@html emoji}
              </button>
            {/each}
          </div>
          <!-- <Button label="Choose from photo library" variant="secondary" disabled /> -->
          <!-- TODO: add multiple steps inline in drawer -->
          <!-- <Button label={$LL.CONTINUE()} on:click={() => goto('/goals')} /> -->
          <!-- <button
          class="w-full rounded-lg bg-indigo-500 px-4 py-2 text-white"
          on:click={() => goto('/goals')}>{$LL.CONTINUE()}</button
        > -->
        </div>
        <button
          slot="close"
          let:close
          use:melt={close}
          class="mt-4 w-full rounded-lg bg-red-100 px-4 py-2 text-[13px]/[24px] font-medium text-red-700"
          >Close</button
        >
      </BottomDrawer>
    </div>
  </div>
</div>
<div
  class="space-y-[10px] rounded-t-3xl bg-white p-6"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
  <Button label="Continue" on:click={() => goto('/welcome/password')} disabled={!emojiChoice} />
</div>
