<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import BottomDrawer from '$lib/components/molecules/dialogs/BottomDrawer.svelte';
  import Button from '$src/lib/components/Button.svelte';

  import Plus from '~icons/ph/plus-bold';

  export let selected: string | undefined;

  //export let defaultValue = '&#x1F642'; // slightly-smiling-face

  const dispatch = createEventDispatcher();

  let emojiSelectIsOpen = false;

  // TODO: switch to Unicode?
  let predefinedEmojis: Array<Array<string>> = [
    // faces
    [
      '&#x1F642', // slightly-smiling-face
      '&#128522', // smiling-face
      '&#129312', // cowboy-hat-face
      '&#129321', // star-struck
      '&#129395', // partying-face
      '&#128540', // twinking-face-with-tongue
      '&#128526', // smiling-face-with-sunglasses
      '&#129392', // smiling-face-with-3-hearts
      '&#128527', // smirk
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
      '&#128025', // octopus
    ],
    // misc
    [
      '&#128161', // light-bulb
      '&#127807', // herb
      '&#128218', // books
      '&#127969', // house-with-garden
      '&#127965', // desert-island
      '&#128188', // briefcase
      '&#128084', // necktie
    ],
  ];
</script>

<BottomDrawer titleText={'Select profile picture'} descriptionText={''} isOpen={emojiSelectIsOpen}>
  <!-- <div slot="trigger"> -->
  <button
    slot="trigger"
    class="flex h-24 w-24 items-center justify-center rounded-full border border-slate-200 bg-white dark:border-slate-600 dark:bg-dark"
    use:melt={trigger}
    let:trigger
    on:click={() => {
      emojiSelectIsOpen = true;
    }}
  >
    {#if selected}
      <span class="text-[44px]/[44px]">
        {@html selected}
      </span>
    {:else}
      <Plus class="h-6 w-6 text-slate-700 dark:text-grey" />
    {/if}
  </button>
  <!-- TODO: Popover is never shown, because not in slot -->
  <!-- {#if $open}
      <div
        use:melt={$content}
        transition:fade={{ duration: 200 }}
        class="z-10 w-1/2 rounded-2xl bg-dark p-[20px] shadow-md"
      >
        <div use:melt={$arrow} />
        <div>
          <p class="text-[12px] font-semibold text-white">Add your profile image</p>
          <p class="pt-2 text-[11px]/[14px] font-normal text-grey">
            Customize your UniMe with your own picture or emoji.
          </p>
        </div>
      </div>
    {/if} -->
  <!-- </div> -->
  <div
    slot="content"
    class="hide-scrollbar flex snap-x snap-mandatory flex-row items-start space-x-4 overflow-x-scroll"
  >
    {#each predefinedEmojis as page}
      <div
        class="grid min-w-fit snap-center grid-cols-3 place-items-center gap-2 rounded-3xl bg-silver p-2 dark:bg-navy"
      >
        {#each page as emoji}
          <!-- TODO: when button pressed (on picture changes, then close drawer) -->
          <button
            class="rounded-2xl border border-slate-300 bg-white p-4 text-[32px]/[32px] dark:border-slate-600 dark:bg-dark"
            on:click={() => {
              dispatch('change', emoji);
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
  <!-- <button
      slot="close"
      let:close
      use:melt={close}
      class="mt-4 w-full rounded-lg bg-red-100 px-4 py-2 text-[13px]/[24px] font-medium text-red-500"
      >Close</button
    > -->
  <div class="mt-6 w-full" slot="close" let:close>
    <Button variant="secondary" label="Close" trigger={close} />
  </div>
</BottomDrawer>

<!-- TODO: needed? -->
<style>
  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the emoji drawer is open */
    position: unset !important;
  }
</style>
