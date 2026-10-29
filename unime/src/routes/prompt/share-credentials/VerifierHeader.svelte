<script lang="ts">
  import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

  import { Image } from '$lib/components';
  import { PlugsConnectedFillIcon } from '$lib/icons';
  import { hash } from '$lib/utils';
  import { hostname } from '$lib/utils/url';

  import DomainName from '../accept-connection/DomainName.svelte';
  import DomainPill from '../accept-connection/DomainPill.svelte';

  export let title: string;
  export let logoUri: string | undefined;
  export let connectionUrl: string | undefined;
  export let domainValidation: ValidationStatus | undefined;

  $: domain = connectionUrl ? hostname(connectionUrl) : undefined;
</script>

<!--
@component
The verifier at the top of the share screens: its logo, a title such as "Mira wants to see", and its domain with the
result of the domain linkage check.
-->
<div class="flex w-full items-center gap-3 py-1">
  <div class="grid size-11 shrink-0 place-items-center overflow-hidden rounded-[11px] bg-white dark:bg-dark">
    {#if logoUri}
      <Image id={hash(logoUri)} isTempAsset={true} imgClass="size-full bg-transparent object-contain" />
    {:else}
      <PlugsConnectedFillIcon class="size-6 text-primary" />
    {/if}
  </div>
  <div class="flex min-w-0 grow flex-col gap-1">
    <p class="text-[15px] font-semibold break-words text-slate-800 dark:text-grey">{title}</p>
    {#if domain}
      <div class="flex flex-wrap items-center gap-x-2">
        <DomainName {domain} status={domainValidation} />
        {#if domainValidation}
          <DomainPill status={domainValidation} />
        {/if}
      </div>
    {/if}
  </div>
</div>
