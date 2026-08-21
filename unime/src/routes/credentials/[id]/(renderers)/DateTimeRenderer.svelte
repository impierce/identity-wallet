<script lang="ts">
  import { state as appState } from '$lib/stores';
  import { formatIso8601 } from '$lib/utils/date';

  export let key: string;
  export let value: string;
</script>

<!--
@component
This renderer for `credentialSubject` fields renders an [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) timestamp
in a human-readable format, using the locale of the current profile. Timestamps without a time of day are rendered as
a plain date.
-->
<div class="rounded-xl bg-background px-4 py-3 text-[13px]/[24px]">
  <h2 class="font-medium text-text-alt">{key}</h2>
  <p class="overflow-x-auto wrap-break-word">{formatIso8601(value, $appState.profile_settings.locale)}</p>
</div>
