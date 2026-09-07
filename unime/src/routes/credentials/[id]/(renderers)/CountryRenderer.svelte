<script lang="ts">
  import Icon from '@iconify/svelte';

  import type { Country } from '$lib/components/forms/countries';

  export let key: string;
  export let country: Country;
</script>

<!--
@component
This renderer for `credentialSubject` fields renders a country as its flag and name, instead of the bare
[ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) code that is stored in the credential.
-->
<div class="flex flex-col rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
  <h2 class="font-medium text-text-alt">{key}</h2>
  <div class="flex grow items-center gap-2">
    <span class="overflow-hidden rounded-sm">
      <Icon class="size-5" icon={`circle-flags:${country.code.toLowerCase()}`} />
    </span>
    <p class="overflow-x-auto wrap-break-word">{country.name}</p>
  </div>
</div>
