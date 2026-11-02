<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';

  import EcosystemAvatar from '$lib/components/ecosystems/EcosystemAvatar.svelte';
  import {
    ArrowCounterClockwiseBoldIcon,
    CaretDownBoldIcon,
    CaretUpBoldIcon,
    SealCheckFillIcon,
    WarningCircleRegularIcon,
  } from '$lib/icons';
  import { state as appState } from '$lib/stores';
  import { formatDateWithWrittenMonth } from '$lib/utils';
  import { issuerEcosystems } from '$lib/utils/ecosystems';
  import { hostname } from '$lib/utils/url';

  export let credential: DisplayCredential;
  // Set while the status and ecosystems are fetched again: what is shown may be outdated.
  export let refreshing = false;

  const dispatchEvent = createEventDispatcher<{ refresh: void }>();

  $: ({ ecosystems, check } = issuerEcosystems(credential, $appState.connections));
  $: unconfirmed = check?.resolved === false;

  // The trust chains on display, by ecosystem position.
  let expanded = new Set<number>();
  function toggle(index: number) {
    expanded = expanded.has(index)
      ? new Set([...expanded].filter((item) => item !== index))
      : new Set([...expanded, index]);
  }

  function warning(): string {
    if (ecosystems.length === 0) {
      return $LL.CREDENTIAL.ECOSYSTEMS.UNCHECKED();
    }
    return check?.last_resolved
      ? $LL.CREDENTIAL.ECOSYSTEMS.UNCONFIRMED_SINCE({
          date: formatDateWithWrittenMonth(check.last_resolved, $appState.profile_settings.locale),
        })
      : $LL.CREDENTIAL.ECOSYSTEMS.UNCONFIRMED();
  }
</script>

<!--
@component
The ecosystems endorsing a credential's issuer, each expandable into the trust chain from the issuer up to the
ecosystem leader. Credentials whose issuer is endorsed by none show nothing here, unless the check failed. Also holds
the control to fetch the credential's status and ecosystems again, which turns into a progress indicator meanwhile.

### Props
- credential
- refreshing: whether a refresh is running

### Events
- refresh: the user asked to fetch the credential's data again
-->
<div class="flex flex-col gap-2">
  <div class="flex items-center justify-between gap-2">
    {#if ecosystems.length > 0}
      <div class="flex items-center">
        <SealCheckFillIcon class="mr-2 text-primary" />
        <p class="text-[13px]/[24px] font-medium text-slate-500 dark:text-white">
          {$LL.CREDENTIAL.ECOSYSTEMS.TITLE()}
        </p>
      </div>
    {:else}
      <div></div>
    {/if}
    <button
      class="flex items-center gap-1.5 rounded-full px-2 py-1 text-[12px]/[16px] font-medium text-text-alt disabled:opacity-80"
      disabled={refreshing}
      aria-busy={refreshing}
      on:click={() => dispatchEvent('refresh')}
      data-testid="credential-refresh"
    >
      {#if refreshing}
        <div
          class="size-3.5 rounded-full border-2 border-primary/25 border-t-primary motion-safe:animate-spin"
          aria-hidden="true"
        ></div>
        {$LL.CREDENTIAL.REFRESHING()}
      {:else}
        <ArrowCounterClockwiseBoldIcon class="size-3.5" />
        {$LL.CREDENTIAL.REFRESH()}
      {/if}
    </button>
  </div>

  {#if unconfirmed}
    <div
      class="flex items-start gap-2 rounded-xl bg-orange-50 p-3 text-[12px]/[18px] text-orange-700 dark:bg-background dark:text-orange-500"
      role="status"
      data-testid="ecosystems-unconfirmed"
    >
      <WarningCircleRegularIcon class="mt-px size-4 shrink-0" />
      <p>{warning()}</p>
    </div>
  {/if}

  {#if ecosystems.length > 0}
    <div class="space-y-2 {refreshing ? 'opacity-70' : ''}">
      {#each ecosystems as ecosystem, index (index)}
        <div class="rounded-xl bg-background">
          <button
            class="flex w-full items-center p-3 text-left"
            aria-expanded={expanded.has(index)}
            aria-label={$LL.CREDENTIAL.ECOSYSTEMS.SHOW_TRUST_CHAIN({ name: ecosystem.name })}
            on:click={() => toggle(index)}
          >
            <EcosystemAvatar
              name={ecosystem.name}
              logoUri={ecosystem.logo_uri}
              isTempAsset="fallback"
              class="mr-3 size-10 rounded-lg"
              textClass="text-[13px]/[18px]"
            />
            <div class="flex min-w-0 grow flex-col">
              <p class="truncate text-[13px]/[20px] font-medium text-slate-800 dark:text-grey">{ecosystem.name}</p>
              {#if ecosystem.ecosystem_leader.name !== ecosystem.name}
                <p class="truncate text-[12px]/[18px] text-slate-500 dark:text-slate-300">
                  {ecosystem.ecosystem_leader.name}
                </p>
              {/if}
            </div>
            {#if expanded.has(index)}
              <CaretUpBoldIcon class="ml-2 size-4 shrink-0 text-text-alt" />
            {:else}
              <CaretDownBoldIcon class="ml-2 size-4 shrink-0 text-text-alt" />
            {/if}
          </button>

          {#if expanded.has(index)}
            <div class="px-3 pb-3" data-testid="trust-chain">
              {#if ecosystem.trust_chain.length === 0}
                <p class="text-[12px]/[18px] text-text-alt">{$LL.CREDENTIAL.ECOSYSTEMS.TRUST_CHAIN_PENDING()}</p>
              {:else}
                <ol>
                  {#each ecosystem.trust_chain as member, position (position)}
                    {@const last = position === ecosystem.trust_chain.length - 1}
                    <li class="flex gap-3">
                      <!-- The line connects each entity to the one endorsing it. -->
                      <div class="flex flex-col items-center">
                        <EcosystemAvatar
                          name={member.name}
                          logoUri={member.logo_uri}
                          isTempAsset="fallback"
                          class="size-8 rounded-full"
                          textClass="text-[10px]/[12px]"
                        />
                        {#if !last}
                          <div class="my-1 min-h-3 w-0.5 grow bg-grey dark:bg-blue"></div>
                        {/if}
                      </div>
                      <div class="flex min-w-0 flex-col pb-3">
                        <p class="truncate text-[13px]/[18px] font-medium text-slate-800 dark:text-grey">
                          {member.name}
                        </p>
                        <p class="truncate text-[12px]/[16px] text-slate-500 dark:text-slate-300">
                          {#if position === 0}
                            {$LL.CREDENTIAL.ECOSYSTEMS.ISSUER()}
                          {:else if last}
                            {$LL.CREDENTIAL.ECOSYSTEMS.ECOSYSTEM_LEADER()}
                          {/if}
                          {#if member.identifier}
                            {#if position === 0 || last}·{/if}
                            {hostname(member.identifier) ?? member.identifier}
                          {/if}
                        </p>
                      </div>
                    </li>
                  {/each}
                </ol>
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>
