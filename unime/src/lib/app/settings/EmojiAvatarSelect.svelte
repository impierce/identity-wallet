<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import { melt } from '@melt-ui/svelte';

  import ActionSheet from '$lib/components/molecules/dialogs/ActionSheet.svelte';
  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';

  import Plus from '~icons/ph/plus-bold';

  export let selected: string | undefined;
  export let showEditButton = false;

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
      '&#x1F4A1', // light-bulb
      '&#x1F33F', // herb
      '&#x1F6E9', // small-airplane
      '&#x1F3E1', // house-with-garden
      '&#x1F3DD', // desert-island
      '&#x1F4BC', // briefcase
      '&#x1F454', // necktie
      '&#x1F680', // rocket
      '&#x1F3B8', // guitar
    ],
  ];
</script>

<ActionSheet titleText={$LL.SETTINGS.PROFILE.DISPLAY_PICTURE.CHANGE()} descriptionText={''} isOpen={emojiSelectIsOpen}>
  <button
    slot="trigger"
    class="relative flex h-24 w-24 items-center justify-center rounded-full
      {selected ? 'bg-primary' : 'border border-slate-200 bg-white dark:border-slate-600 dark:bg-dark'}
      {showEditButton ? 'mb-[34px]' : ''}"
    use:melt={trigger}
    let:trigger
    on:click={() => (emojiSelectIsOpen = true)}
  >
    {#if selected}
      <span class="text-[44px]/[44px]">
        {@html selected}
      </span>
      {#if showEditButton}
        <div class="absolute -bottom-[34px] text-[13px]/[24px] text-slate-500 dark:text-slate-300">
          {$LL.SETTINGS.PROFILE.DISPLAY_PICTURE.EDIT()}
        </div>
      {/if}
    {:else}
      <Plus class="h-6 w-6 text-slate-700 dark:text-grey" />
    {/if}
  </button>
  <div
    slot="content"
    class="hide-scrollbar flex snap-x snap-mandatory flex-row items-start space-x-4 overflow-x-scroll"
  >
    {#each predefinedEmojis as page}
      <div
        class="grid min-w-fit snap-center grid-cols-3 place-items-center gap-2 rounded-3xl bg-silver p-2 dark:bg-navy"
      >
        {#each page as emoji}
          <button
            class="rounded-2xl border-2 p-4 text-[32px]/[32px] {emoji === selected
              ? 'border-primary'
              : 'border-grey dark:border-blue'}"
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
  <div class="mt-6 w-full" slot="close" let:close>
    {#if !!selected}
      <button
        use:melt={close}
        class="h-[48px] w-full rounded-xl border border-slate-200 px-4 py-2 text-[13px]/[24px] font-medium text-red-500 dark:border-slate-600"
        on:click={() => dispatch('change', '')}>{$LL.SETTINGS.PROFILE.DISPLAY_PICTURE.REMOVE()}</button
      >
    {:else}
      <Button variant="secondary" label={$LL.CLOSE()} trigger={close} />
    {/if}
  </div>
</ActionSheet>

<!-- TODO: needed? -->
<style>
  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the emoji drawer is open */
    position: unset !important;
  }
</style>
