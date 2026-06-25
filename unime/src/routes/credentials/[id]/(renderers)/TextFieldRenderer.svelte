<script lang="ts">
  import CollapsibleWrapper from '../CollapsibleWrapper.svelte';

  export let key: string;
  export let value: string;

  function hasValue(text: string): boolean {
    return text.trim().length > 0;
  }

  function isLongText(text: string): boolean {
    return text.length > 100;
  }

  const URL_PART_REGEX = /((?:https?:\/\/|www\.)[^\s]+)/g;

  function isUrl(text: string): boolean {
    return /^(?:https?:\/\/|www\.)[^\s]+$/.test(text);
  }

  function toHref(text: string): string {
    return text.startsWith('www.') ? `https://${text}` : text;
  }

  let textParts: string[] = [];
  $: textParts = value.split(URL_PART_REGEX).filter((part) => part.length > 0);
</script>

{#if hasValue(value)}
  {#if isLongText(value)}
    <CollapsibleWrapper>
      <h2 slot="title" class="font-medium text-text-alt">{key}</h2>
      <p class="overflow-x-auto wrap-break-word">
        {#each textParts as part}
          {#if isUrl(part)}
            <a href={toHref(part)} target="_blank" rel="noopener noreferrer" class="break-all text-blue-600 underline"
              >{part}</a
            >
          {:else}
            {part}
          {/if}
        {/each}
      </p>
    </CollapsibleWrapper>
  {:else}
    <div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
      <h2 class="font-medium text-text-alt">{key}</h2>
      <p class="overflow-x-auto wrap-break-word">
        {#each textParts as part}
          {#if isUrl(part)}
            <a href={toHref(part)} target="_blank" rel="noopener noreferrer" class="break-all underline">{part}</a>
          {:else}
            {part}
          {/if}
        {/each}
      </p>
    </div>
  {/if}
{/if}

<style>
  /* Hide scrollbar. */
  p::-webkit-scrollbar {
    display: none;
  }
</style>
