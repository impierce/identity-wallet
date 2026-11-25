<script lang="ts">
  import Icon from '@iconify/svelte';

  import ExternalLink from '$lib/components/ExternalLink.svelte';
  import { state as appState } from '$lib/stores';
  import { countryByCode, findCountry } from '$lib/utils/country';
  import { formatIso8601, parseIso8601 } from '$lib/utils/date';
  import { linkifyClaim, uriParts } from '$lib/utils/url';

  export let key: string;
  export let value: string;
  /** The issuer's `format` for the value, which takes precedence over what the value looks like. */
  export let format: string | undefined = undefined;
  /** Whether links are links; inside a button, they show as text. */
  export let links = true;

  const KNOWN_FORMATS = ['country', 'date', 'date-time', 'uri'];

  $: known = format !== undefined && KNOWN_FORMATS.includes(format);
  $: country = format === 'country' ? countryByCode(value) : known ? null : findCountry(key, value);
  $: date = format === 'date' || format === 'date-time' || !known ? parseIso8601(value) : null;
  $: parts = format === 'uri' ? uriParts(value) : known ? [{ text: value }] : linkifyClaim(key, value);
</script>

<!--
@component
The value of a single field inside an Object, List or Table, formatted like the card renderers format a top-level
claim: images, dates in the profile's locale, countries with their flag, and links. A `format` from the issuer's claims
metadata (`country`, `date`, `date-time` or `uri`) decides; without one, the value's looks do.
-->
{#if !known && value.startsWith('data:image/')}
  <img src={value} alt={key} class="max-h-32 rounded-lg" />
{:else if date}
  {formatIso8601(value, $appState.profile_settings.locale)}
{:else if country}
  <span class="inline-flex items-center gap-2">
    <span class="overflow-hidden rounded-sm">
      <Icon class="size-5" icon={`circle-flags:${country.code.toLowerCase()}`} />
    </span>
    {country.name}
  </span>
{:else}
  {#each parts as part}
    {#if part.url && links}
      <ExternalLink href={part.url} class="break-all text-secondary underline">{part.text}</ExternalLink>
    {:else}
      {part.text}
    {/if}
  {/each}
{/if}
