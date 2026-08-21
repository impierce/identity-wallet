<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  export let title: string;
  // Omit to render the header without a "Show more" link.
  export let href: string | undefined = undefined;
</script>

<!--
@component
Section title with an optional "Show more" link to a full list.

### Props
- title
- href (optional)
-->
<div class="flex w-full items-center justify-between px-1 pb-2">
  <p class="text-[13px]/[24px] font-semibold text-slate-800 dark:text-grey">{title}</p>
  {#if href}
    <a {href} class="text-[13px]/[24px] font-normal text-primary">
      {$LL.SCAN.CONNECTION_REQUEST.SHOW_MORE()}
    </a>
  {/if}
</div>
