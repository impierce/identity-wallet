<script lang="ts">
  import Icon from '@iconify/svelte';

  import { state as appState } from '$lib/stores';
  import { findCountry } from '$lib/utils/country';
  import { formatIso8601 } from '$lib/utils/date';

  export let key: string;
  export let value: unknown;

  $: country = findCountry(key, value);
  $: text = country
    ? country.name
    : typeof value === 'string'
      ? formatIso8601(value, $appState.profile_settings.locale)
      : String(value ?? '');
</script>

<!--
@component
Renders a single claim of a credential as one line, with its name and its value next to each other. Values are
formatted like they are by the `ClaimRenderer`, which renders each claim as a separate card instead.

A claim is opted out of `prose` (which this renderer is used inside of): the margins and line heights that `prose`
puts on headings and paragraphs differ, which would push the name and the value of a claim out of line with each
other. Spacing between claims is left to the parent.
-->
<div class="not-prose flex items-center justify-between gap-4 text-[13px]/[24px]">
  <h4 class="shrink-0 font-medium text-text-alt">{key}</h4>
  <div class="flex min-w-0 items-center gap-2">
    {#if country}
      <span class="overflow-hidden rounded-sm">
        <Icon class="size-5" icon={`circle-flags:${country.code.toLowerCase()}`} />
      </span>
    {/if}
    <p class="text-right wrap-break-word">{text}</p>
  </div>
</div>
