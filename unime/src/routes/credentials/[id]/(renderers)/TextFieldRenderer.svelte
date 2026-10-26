<script lang="ts">
  import ExternalLink from '$lib/components/ExternalLink.svelte';
  import { linkifyClaim, type TextPart } from '$lib/utils/url';

  import CollapsibleWrapper from '../CollapsibleWrapper.svelte';

  export let key: string;
  export let value: string;

  let textParts: TextPart[] = [];
  $: textParts = linkifyClaim(key, value);
</script>

{#if value.trim().length > 100}
  <CollapsibleWrapper>
    <h2 slot="title" class="font-medium text-text-alt">{key}</h2>
    <p class="overflow-x-auto wrap-break-word">
      {#each textParts as part}
        {#if part.url}
          <ExternalLink href={part.url} class="break-all underline">{part.text}</ExternalLink>
        {:else}
          {part.text}
        {/if}
      {/each}
    </p>
  </CollapsibleWrapper>
{:else if value.trim().length > 0}
  <div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
    <h2 class="font-medium text-text-alt">{key}</h2>
    <p class="overflow-x-auto wrap-break-word">
      {#each textParts as part}
        {#if part.url}
          <ExternalLink href={part.url} class="break-all underline">{part.text}</ExternalLink>
        {:else}
          {part.text}
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
