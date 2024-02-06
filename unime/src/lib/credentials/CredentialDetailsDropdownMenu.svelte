<script lang="ts">
  import type { DisplayCredential } from '@bindings/display-credential/DisplayCredential';
  import { fade, fly } from 'svelte/transition';

  import { createDialog, createDropdownMenu, melt } from '@melt-ui/svelte';

  import { dispatch } from '$lib/dispatcher';
  import { colors } from '$src/lib/app/colors';
  import { icons } from '$src/lib/app/icons';

  import Bank from '~icons/ph/bank-light';
  import Car from '~icons/ph/car-light';
  import Certificate from '~icons/ph/certificate-light';
  import DotsThreeVertical from '~icons/ph/dots-three-vertical-bold';
  import EnvelopeSimple from '~icons/ph/envelope-simple-light';
  import House from '~icons/ph/house-light';
  import Leaf from '~icons/ph/leaf-light';
  import Question from '~icons/ph/question-light';
  import User from '~icons/ph/user-light';

  import Button from '../components/atoms/Button.svelte';

  export let credential: DisplayCredential;

  const {
    elements: { trigger, menu, item, arrow },
    states: { open },
  } = createDropdownMenu({
    forceVisible: true,
  });

  // Dialog to change the name
  const {
    elements: {
      trigger: triggerNameDialog,
      overlay: overlayNameDialog,
      content: contentNameDialog,
      title,
      description,
      close,
      portalled: portalledNameDialog,
    },
    states: { open: openNameDialog },
  } = createDialog({
    forceVisible: true,
  });

  // Dialog to customize the appearance
  const {
    elements: {
      trigger: triggerAppearanceDialog,
      overlay: overlayAppearanceDialog,
      content: contentAppearanceDialog,
      title: titleAppearanceDialog,
      description: descriptionAppearanceDialog,
      close: closeAppearanceDialog,
      portalled: portalledAppearanceDialog,
    },
    states: { open: openAppearanceDialog },
  } = createDialog({
    forceVisible: true,
  });

  let displayName: string = credential.metadata.display.name || credential.data.type.at(-1);

  let currentAppearance = {
    color: credential.metadata.display.color || 'bg-indigo-100',
    icon: credential.metadata.display.icon || 'User',
  };
</script>

<button type="button" class="trigger p-1" use:melt={$trigger} aria-label="Edit credential details">
  <DotsThreeVertical class="h-6 w-6 {$$props.class}" />
  <span class="sr-only">Open Dropdown Menu</span>
</button>

{#if $open}
  <div
    class="z-10 flex max-h-[300px] flex-col items-end rounded-2xl bg-white p-3 shadow-md outline-none dark:bg-dark"
    use:melt={$menu}
    transition:fly={{ duration: 150, y: -10 }}
  >
    <!-- TODO: refactor BottomDrawer to support tiggers outside component / slots -->
    <!-- <BottomDrawer titleText="Change name">
      <button
        slot="trigger"
        let:trigger
        use:melt={trigger}
        class="rounded-xl px-3 py-2 font-medium text-slate-800 outline-none hover:cursor-pointer hover:bg-slate-100 active:bg-slate-100 disabled:opacity-50"
        disabled
      >
        Change name (via BottomDrawer)
      </button>

      <div slot="content">
        <input
          type="text"
          class="w-full rounded-lg border bg-white px-4 py-2 text-slate-800"
          placeholder="Enter a name"
          value={credential.metadata.display.name}
        />
        <Button
          label="Change"
          on:click={() =>
            dispatch({
              type: '[Credential Metadata] Update',
              payload: {
                id: credential.id, 
                name: displayName
              }
            })}
        />
      </div>

      <button
        slot="close"
        let:close
        use:melt={close}
        class="mt-2 w-full rounded-lg border bg-red-100 px-4 py-2 text-sm font-medium text-red-500"
        >Close</button
      >
    </BottomDrawer> -->

    <button
      use:melt={$item}
      use:melt={$triggerNameDialog}
      class="rounded-xl px-3 py-2 text-[14px]/[22px] font-medium text-slate-800 outline-none dark:text-white"
      >Change name</button
    >

    <button
      use:melt={$item}
      use:melt={$triggerAppearanceDialog}
      class="rounded-xl px-3 py-2 text-[14px]/[22px] font-medium text-slate-800 outline-none dark:text-white"
      >Customize appearance</button
    >

    <!-- TODO: feature disabled: "Delete credential" -->
    <!-- <button
      class="rounded-xl px-3 py-2 font-medium text-red-500 outline-none active:bg-red-100"
      use:melt={$item}>Delete credential</button
    > -->
    <!-- <div use:melt={$arrow} /> -->
  </div>
{/if}

<!-- Dialog to change the name -->
<div use:melt={$portalledNameDialog}>
  {#if $openNameDialog}
    <div
      use:melt={$overlayNameDialog}
      class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
      transition:fade={{ duration: 150 }}
    />
    <div
      use:melt={$contentNameDialog}
      class="fixed left-[50%] top-[50%] z-50 flex max-h-[85vh]
      w-[90vw] max-w-[450px] translate-x-[-50%] translate-y-[-50%] flex-col
      space-y-4 rounded-xl bg-white p-6 shadow-lg dark:bg-dark"
    >
      <!-- Content -->
      <input
        type="text"
        class="h-12 w-full rounded-xl border border-slate-300 bg-white px-4 py-3 text-[13px]/[24px] text-slate-500 dark:border-slate-600 dark:bg-dark dark:text-slate-300"
        placeholder="Enter a new name"
        bind:value={displayName}
      />
      <Button
        label="Update"
        on:click={() => {
          dispatch({
            type: '[Credential Metadata] Update',
            payload: {
              id: credential.id,
              name: displayName,
            },
          });
          openNameDialog.set(false);
        }}
      />
      <!-- Close -->
      <!-- <button
        use:melt={$close}
        class="inline-flex h-8 items-center justify-center rounded-lg
                bg-zinc-100 px-4 font-medium leading-none text-zinc-600"
      >
        Cancel
      </button> -->
    </div>
  {/if}
</div>

<!-- Dialog to customize the appearance -->
<div use:melt={$portalledAppearanceDialog}>
  {#if $openAppearanceDialog}
    <div
      use:melt={$overlayAppearanceDialog}
      class="fixed inset-0 z-50 bg-black/50 backdrop-blur-sm"
      transition:fade={{ duration: 150 }}
    />
    <div
      use:melt={$contentAppearanceDialog}
      class="fixed left-[50%] top-[50%] z-50 flex max-h-[85vh]
      w-[90vw] max-w-[450px] translate-x-[-50%] translate-y-[-50%] flex-col
      space-y-4 rounded-xl bg-white p-6 shadow-lg dark:bg-dark"
    >
      <!-- Content -->
      <!-- <input
        type="text"
        class="w-full rounded-lg border bg-white px-4 py-2 text-slate-800"
        placeholder="Enter a new name"
        bind:value={displayName}
      />
      -->
      <div class="flex items-center justify-between">
        <!-- Colors -->
        <div class="grid w-fit grid-cols-3 place-items-center gap-1">
          {#each colors as color}
            <button
              class="{color} h-9 w-9 rounded-md border border-slate-300 dark:border-slate-600"
              on:click={() => (currentAppearance.color = color)}
            />
          {/each}
        </div>

        <!-- Combined -->
        <div class="rounded-2xl {currentAppearance.color} flex h-14 w-14 items-center justify-center">
          <svelte:component this={icons[currentAppearance.icon || 'Bank']} class="h-6 w-6" />
        </div>

        <!-- Icons -->
        <div class="grid w-fit grid-cols-3 place-items-center gap-1">
          {#each Object.entries(icons) as icon}
            <button
              class="h-9 w-9 rounded-md border border-slate-300 p-2 dark:border-slate-600"
              on:click={() => (currentAppearance.icon = icon.at(0))}
            >
              <svelte:component this={icon.at(1)} class="h-5 w-5 text-slate-800 dark:text-grey" />
            </button>
          {/each}
        </div>
      </div>

      <Button
        label="Update"
        on:click={() => {
          dispatch({
            type: '[Credential Metadata] Update',
            payload: {
              id: credential.id,
              icon: currentAppearance.icon,
              color: currentAppearance.color,
            },
          });
          openAppearanceDialog.set(false);
        }}
      />

      <!-- Close -->
      <!-- <button
        use:melt={$close}
        class="inline-flex h-8 items-center justify-center rounded-lg
                bg-zinc-100 px-4 font-medium leading-none text-zinc-600"
      >
        Cancel
      </button> -->
    </div>
  {/if}
</div>
