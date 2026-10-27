<script lang="ts">
  import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

  import { ExternalLink } from '$lib/components';
  import { ArrowSquareOutBoldIcon } from '$lib/icons';
  import { domainUrl } from '$lib/utils/url';

  export let domain: string;
  export let status: ValidationStatus | undefined;

  let className = '';
  export { className as class };

  // Only a domain whose linkage was verified is worth sending the user to.
  $: href = status === 'Success' ? domainUrl(domain) : undefined;
</script>

<!--
@component
A hostname, intended to sit above the `DomainPill`. A verified domain opens its website in the system browser.

### Props
- domain
- status
- class
-->
{#if href}
  <ExternalLink
    {href}
    class="inline-flex items-center gap-1 text-[13px]/[20px] font-normal break-all text-secondary {className}"
  >
    {domain}
    <ArrowSquareOutBoldIcon class="size-4 shrink-0" />
  </ExternalLink>
{:else}
  <p class="text-[13px]/[20px] font-normal break-all text-text-alt {className}">{domain}</p>
{/if}
