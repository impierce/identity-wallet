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
      style: 'bg-green-50 text-green-700 dark:bg-green-950 dark:text-green-300',
    },
    Failure: {
      icon: WarningCircleFillIcon,
      label: $LL.DOMAIN_LINKAGE.PILL_UNTRUSTED(),
      style: 'bg-rose-50 text-rose-700 dark:bg-rose-950 dark:text-rose-300',
    },
    Unknown: {
      icon: WarningCircleFillIcon,
      label: $LL.DOMAIN_LINKAGE.PILL_UNVERIFIED(),
      style: 'bg-amber-50 text-amber-700 dark:bg-amber-950 dark:text-amber-300',
    },
  } satisfies Record<ValidationStatus, unknown>;

  $: pill = appearance[status];
</script>

<!--
@component
A badge showing the result of the domain linkage check, intended to sit centred
under the hostname.

### Props
- status

### Usage
```tsx
<DomainPill status={domain_validation.status} />
```
-->
<span class="flex items-center gap-1 rounded-full px-2 py-[2px] text-[12px]/[20px] font-normal {pill.style}">
  <svelte:component this={pill.icon} class="size-4" />
  {pill.label}
</span>
