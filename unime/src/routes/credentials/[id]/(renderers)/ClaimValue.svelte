<script lang="ts">
  import Icon from '@iconify/svelte';

  import ExternalLink from '$lib/components/ExternalLink.svelte';
  import { state as appState } from '$lib/stores';
  import { findCountry } from '$lib/utils/country';
  import { formatIso8601, parseIso8601 } from '$lib/utils/date';
  import { linkifyClaim } from '$lib/utils/url';

  export let key: string;
  export let value: string;

  $: country = findCountry(key, value);
</script>

<!--
@component
The value of a single field inside an Object, List or Table, formatted like the card renderers format a top-level
claim: images, dates in the profile's locale, countries with their flag, and links.
-->
{#if value.startsWith('data:image/')}
  <img src={value} alt={key} class="max-h-32 rounded-lg" />
{:else if parseIso8601(value)}
  {formatIso8601(value, $appState.profile_settings.locale)}
{:else if country}
  <span class="inline-flex items-center gap-2">
    <span class="overflow-hidden rounded-sm">
      <Icon class="size-5" icon={`circle-flags:${country.code.toLowerCase()}`} />
    </span>
    {country.name}
  </span>
{:else}
  {#each linkifyClaim(key, value) as part}
    {#if part.url}
      <ExternalLink href={part.url} class="break-all text-secondary underline">{part.text}</ExternalLink>
    {:else}
      {part.text}
    {/if}
  {/each}
{/if}
