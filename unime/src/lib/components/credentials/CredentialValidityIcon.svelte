<script lang="ts">
  import { CalendarPlusFillIcon, WarningCircleFillIcon, XCircleFillIcon } from '$lib/icons';
  import type { CredentialValidity } from '$lib/utils/credentialValidity';

  export let validity: CredentialValidity;
</script>

<!--
@component
The icon on a credential list entry for a credential that cannot be used right now.

### Props
- validity
-->
{#if validity.state === 'revoked' || validity.state === 'suspended'}
  <XCircleFillIcon class="mr-3 size-6 text-red-600 dark:text-red-500" />
{:else if validity.state === 'expired'}
  <WarningCircleFillIcon class="mr-3 size-6 text-orange-600 dark:text-orange-500" />
{:else if validity.state === 'not_yet_valid'}
  <CalendarPlusFillIcon class="mr-3 size-6 text-ex-grey-1 dark:text-text-alt" />
{/if}
