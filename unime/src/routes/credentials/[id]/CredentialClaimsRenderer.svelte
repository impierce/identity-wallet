<script lang="ts">
  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import AddressRenderer from './AddressRenderer.svelte';
  import DefaultRenderer from './DefaultRenderer.svelte';
  import ELMRenderer from './ELMRenderer.svelte';
  import OpenBadgeRenderer from './OpenBadgeRenderer.svelte';
  import PidRenderer from './PidRenderer.svelte';

  export let credential: DisplayCredential;

  $: credentialTypes = credential.data?.type as string[] | undefined;
</script>

<!--
@component
Selects the claims renderer that matches a credential's declared type. Keeping this decision in
one place ensures wallet credentials and linked certifications render structured claims identically.
-->
{#if credentialTypes?.includes('OpenBadgeCredential') || credentialTypes?.includes('AchievementCredential')}
  <OpenBadgeRenderer {credential} />
{:else if credentialTypes?.includes('EuropeanDigitalCredential')}
  <ELMRenderer {credential} />
{:else if credentialTypes?.includes('ResidenceCredential')}
  <AddressRenderer {credential} />
{:else if credentialTypes?.includes('NaturalPersonCredential')}
  <PidRenderer {credential} />
{:else}
  <DefaultRenderer {credential} />
{/if}
