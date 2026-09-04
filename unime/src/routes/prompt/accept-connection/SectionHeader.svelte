<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  export let title: string;
  // Renders the trailing text as a link to a full list. Omit for no link at all.
  export let href: string | undefined = undefined;
  // Renders the trailing text as a button that fires `action` instead, for expanding in
  // place. Takes precedence over `href`.
  export let action: string | undefined = undefined;

  const dispatch = createEventDispatcher<{ action: void }>();
</script>

<!--
@component
Section title with an optional trailing "Show more" — either a link to a full list (`href`)
or an in-place toggle (`action`).

### Props
- title
- href (optional)
- action (optional)

### Events
- action
-->
<div class="flex w-full items-center justify-between px-1 pb-2">
  <p class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">{title}</p>
  {#if action}
    <button type="button" class="text-[13px]/[24px] font-normal text-primary" on:click={() => dispatch('action')}>
      {action}
    </button>
  {:else if href}
    <a {href} class="text-[13px]/[24px] font-normal text-primary">
      {$LL.SCAN.CONNECTION_REQUEST.SHOW_MORE()}
    </a>
  {/if}
</div>
