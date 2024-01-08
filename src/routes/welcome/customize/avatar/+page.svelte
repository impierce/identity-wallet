<script lang="ts">
  import { goto } from '$app/navigation';
  import { fade } from 'svelte/transition';

  import { createPopover, melt } from '@melt-ui/svelte';

  import LL from '$src/i18n/i18n-svelte';
  import Button from '$src/lib/components/atoms/Button.svelte';
  import EmojiAvatarSelect from '$src/lib/settings/EmojiAvatarSelect.svelte';
  import { onboarding_state } from '$src/stores';

  // TODO: unused
  const {
    elements: { trigger, content, arrow, close },
    states: { open },
  } = createPopover({
    forceVisible: true,
    defaultOpen: true,
  });

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
    <p class="pb-4 text-3xl font-semibold text-slate-700 dark:text-grey">
      Set a display <span class="text-primary">picture</span>
    </p>
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Make it yours.</p>
    <div class="mt-[70px] flex w-full items-center justify-center">
      <EmojiAvatarSelect
        selected={$onboarding_state.picture}
        on:change={(e) => ($onboarding_state.picture = e.detail)}
      />
    </div>
  </div>
</div>
<div
  class="space-y-[10px] rounded-t-3xl bg-white p-6 dark:bg-dark"
  in:fade={{ delay: 200 }}
  out:fade={{ duration: 200 }}
>
  <Button label="Continue" on:click={() => goto('/welcome/password')} disabled={!$onboarding_state.picture} />
</div>

<!-- TODO: needed here or in component? -->
<style>
  :global(body) {
    /* Fixes a UI problem on iOS where there is a white bar at the bottom when the emoji drawer is open */
    position: unset !important;
  }
</style>
