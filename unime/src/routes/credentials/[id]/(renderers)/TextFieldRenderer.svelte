<script lang="ts">
  import { HTTPS_URL_PART_REGEX, isUrl } from '../../../../lib/utils/url';
  import CollapsibleWrapper from '../CollapsibleWrapper.svelte';

  export let key: string;
  export let value: string;

  let textParts: string[] = [];
  $: textParts = value.split(HTTPS_URL_PART_REGEX).filter((part) => part.length > 0);
</script>

{#if value.trim().length > 100}
  <CollapsibleWrapper>
    <h2 slot="title" class="font-medium text-text-alt">{key}</h2>
    <p class="overflow-x-auto wrap-break-word">
      {#each textParts as part}
        {#if isUrl(part)}
          <a href={part} target="_blank" rel="noopener noreferrer" class="break-all underline">{part}</a>
        {:else}
          {part}
        {/if}
      {/each}
    </p>
  </CollapsibleWrapper>
{:else if value.trim().length > 0}
  <div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    <h2 class="font-medium text-text-alt">{key}</h2>
    <p class="overflow-x-auto wrap-break-word">
      {#each textParts as part}
        {#if isUrl(part)}
          <a href={part} target="_blank" rel="noopener noreferrer" class="break-all underline">{part}</a>
        {:else}
          {part}
        {/if}
      {/each}
    </p>
  </div>
{/if}

<style>
  /* Hide scrollbar. */
  p::-webkit-scrollbar {
    display: none;
  }
</style>
