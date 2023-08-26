<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { BottomDrawer, TopNavigation } from '@impierce/ui-components';
  import { createPopover, melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/Button.svelte';
  import { dispatch } from '$src/lib/dispatcher';
  import { onboarding_state } from '$src/stores';

  import Plus from '~icons/ph/plus-bold';

  const {
    elements: { trigger, content, arrow, close },
    states: { open }
  } = createPopover({
    defaultOpen: true
  });

  // hex codes
  let predefinedEmojis: Array<Array<string>> = [
    // faces
    [
      '&#128578', // slightly-smiling-face
      '&#128522', // smiling-face
      '&#129312', // cowboy-hat-face
      '&#129321', // star-struck
      '&#129395', // partying-face
      '&#128540', // twinking-face-with-tongue
      '&#128526', // smiling-face-with-sunglasses
      '&#129392', // smiling-face-with-3-hearts
      '&#128527' // smirk
    ],
    // animals
    [
      '&#128053', // monkey-face
      '&#128048', // rabbit-face
      '&#128060', // panda-face
      '&#129418', // fox-face
      '&#128040', // koala
      '&#128037', // baby-chick
      '&#129417', // owl
      '&#129412', // unicorn
      '&#128025' // octopus
    ],
    // misc
    [
      '&#128161', // light-bulb
      '&#127807', // herb
      '&#128218', // books
      '&#127969', // house-with-garden
      '&#127965', // desert-island
      '&#128188', // briefcase
      '&#128084' // necktie
    ]
  ];

  $: {
    console.log($onboarding_state);
  }

  let emojiSelectIsOpen = false;

  $: {
    console.log({ emojiSelectIsOpen });
  }
</script>

<!-- <TopNavigation title="Avatar" on:back={() => history.back()} /> -->
<div class="mt-8 grow p-4" in:fade={{ delay: 200 }} out:fade={{ duration: 200 }}>
  <div class="px-2 pb-8 pt-4">
    <p class="pb-4 text-3xl font-semibold text-slate-800">
      Set a display <span class="text-primary">picture</span>
    </p>
    <p class="text-[15px]/[24px] font-medium text-slate-500">Make it yours.</p>
    <div class="mt-[70px] flex w-full items-center justify-center">
      <BottomDrawer
        titleText={'Select profile picture'}
        descriptionText={''}
        isOpen={emojiSelectIsOpen}
      >
        <!-- <div slot="trigger"> -->
        <button
          slot="trigger"
          let:trigger
          class="flex h-24 w-24 items-center justify-center rounded-full border border-slate-200 bg-white"
          use:melt={trigger}
          on:click={() => {
            emojiSelectIsOpen = true;
          }}
        >
          {#if $onboarding_state.picture}
            <span class="text-[44px]/[44px]">
              {@html $onboarding_state.picture}
            </span>
          {:else}
            <Plus class="h-6 w-6" />
          {/if}
        </button>
        <!-- TODO: Popover is never shown, because not in slot -->
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
            </div>
          </div>
        {/if}
        <!-- </div> -->
        <div
          slot="content"
          class="hide-scrollbar flex snap-x snap-mandatory flex-row items-start space-x-4 overflow-x-scroll"
        >
          {#each predefinedEmojis as page}
            <div
              class="grid min-w-fit snap-center grid-cols-3 place-items-center gap-2 rounded-3xl bg-neutral-100 p-2"
            >
              {#each page as emoji}
                <!-- TODO: when button pressed (on picture changes, then close drawer) -->
                <button
                  class="rounded-2xl border bg-white p-4 text-[32px]/[32px]"
                  on:click={() => {
                    $onboarding_state.picture = emoji;
                    emojiSelectIsOpen = false;
                  }}
                  >{@html emoji}
                </button>
              {/each}
            </div>
            <!-- <Button label="Choose from photo library" variant="secondary" disabled /> -->
            <!-- TODO: add multiple steps inline in drawer -->
            <!-- <Button label={$LL.CONTINUE()} on:click={() => goto('/goals')} /> -->
            <!-- <button
          class="w-full rounded-lg bg-primary px-4 py-2 text-white"
          on:click={() => goto('/goals')}>{$LL.CONTINUE()}</button
        > -->
          {/each}
        </div>
        <button
          slot="close"
          let:close
          use:melt={close}
          class="mt-4 w-full rounded-lg bg-red-100 px-4 py-2 text-[13px]/[24px] font-medium text-red-500"
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
  <Button
    label="Continue"
    on:click={() => goto('/welcome/password')}
    disabled={!$onboarding_state.picture}
  />
</div>
