<script lang="ts">
  import { findCountry } from '$lib/utils/country';
  import { parseIso8601 } from '$lib/utils/date';

  import CountryRenderer from './CountryRenderer.svelte';
  import DataUrlImageRenderer from './DataUrlImageRenderer.svelte';
  import DateTimeRenderer from './DateTimeRenderer.svelte';
  import TextFieldRenderer from './TextFieldRenderer.svelte';

  export let key: string;
  export let value: unknown;

  function isDataUrl(value: unknown): value is string {
    return typeof value === 'string' && value.startsWith('data:image/');
  }

  $: country = findCountry(key, value);
</script>

<!--
@component
Renders a single claim of a credential by picking the renderer that fits the value best. Claims that no specialized
renderer applies to are rendered as plain text.
-->
{#if isDataUrl(value)}
  <DataUrlImageRenderer {key} dataUrl={value} />
{:else if parseIso8601(value)}
  <DateTimeRenderer {key} value={String(value)} />
{:else if country}
  <CountryRenderer {key} {country} />
{:else}
  <TextFieldRenderer {key} value={String(value ?? '')} />
{/if}
