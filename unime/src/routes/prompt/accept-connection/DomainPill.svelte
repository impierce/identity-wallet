<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

  import { SealCheckFillIcon, WarningCircleFillIcon } from '$lib/icons';

  export let status: ValidationStatus;

  // `Failure` means the domain linkage was checked and did not hold.
  // `Unknown` means no proof could be found at all.
  $: appearance = {
    Success: {
      icon: SealCheckFillIcon,
      label: $LL.DOMAIN_LINKAGE.PILL_VERIFIED(),
      style: 'text-secondary',
    },
    Failure: {
      icon: WarningCircleFillIcon,
      label: $LL.DOMAIN_LINKAGE.PILL_UNTRUSTED(),
      style: 'text-rose-700 dark:text-rose-300',
    },
    Unknown: {
      icon: WarningCircleFillIcon,
      label: $LL.DOMAIN_LINKAGE.PILL_UNVERIFIED(),
      style: 'text-amber-700 dark:text-amber-300',
    },
  } satisfies Record<ValidationStatus, unknown>;

  $: pill = appearance[status];
</script>

<!--
@component
An icon and label showing the result of the domain linkage check, intended to sit
below the hostname.

### Props
- status

### Usage
```tsx
<DomainPill status={domain_validation.status} />
```
-->
<span class="flex items-center gap-1 text-[12px]/[20px] font-normal {pill.style}">
  <svelte:component this={pill.icon} class="size-4" />
  {pill.label}
</span>
