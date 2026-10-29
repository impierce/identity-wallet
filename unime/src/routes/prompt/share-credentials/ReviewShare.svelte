<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  import LL from '$i18n/i18n-svelte';

  import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
  import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

  import { Button, TopNavBar } from '$lib/components';
  import { CheckRegularIcon, EyeSlashRegularIcon, IdentificationBadgeRegularIcon, SealCheckFillIcon } from '$lib/icons';

  import VerifierHeader from './VerifierHeader.svelte';

  export let clientName: string;
  export let logoUri: string | undefined;
  export let connectionUrl: string | undefined;
  export let domainValidation: ValidationStatus | undefined;
  // The fields that will be shared, per credential.
  export let shared: { credential: DisplayCredential; fields: { label: string; value: string }[] }[];
  // Switched-off fields and credentials that were not chosen, each with what it belongs to.
  export let notShared: { label: string; detail: string }[];
  export let loading = false;

  const dispatch = createEventDispatcher<{ back: void; share: void; decline: void }>();

  $: fieldCount = shared.reduce((count, group) => count + group.fields.length, 0);
</script>

<!--
@component
The last check before sharing: exactly which fields go to the verifier, per credential, and what stays behind.
-->
<div class="safe-area-height flex flex-col items-stretch overflow-y-auto bg-silver dark:bg-navy">
  <TopNavBar
    title={$LL.SCAN.SHARE_CREDENTIALS.REVIEW.NAVBAR_TITLE()}
    on:back={() => dispatch('back')}
    disabled={loading}
    class="sticky top-0 z-10"
  />

  <div class="flex grow flex-col gap-3 px-4 pt-3 pb-6">
    <VerifierHeader
      title={$LL.SCAN.SHARE_CREDENTIALS.REVIEW.SHARING_WITH({ name: clientName })}
      {logoUri}
      {connectionUrl}
      {domainValidation}
    />

    <div class="flex flex-col rounded-xl border border-grey bg-white pb-2.5 dark:border-slate-600 dark:bg-dark">
      <div class="flex items-center gap-1.5 px-[14px] pt-[14px] pb-0.5">
        <SealCheckFillIcon class="size-4 text-primary" />
        <p class="text-[13px] font-semibold text-slate-800 dark:text-grey">
          {$LL.SCAN.SHARE_CREDENTIALS.REVIEW.WILL_BE_SHARED({ count: fieldCount })}
        </p>
      </div>
      {#each shared as { credential, fields }, position (`${credential.id}-${position}`)}
        <div class="flex items-center gap-2 px-[14px] pt-3 pb-1 text-slate-500 dark:text-slate-300">
          <IdentificationBadgeRegularIcon class="size-4 shrink-0" />
          <p class="truncate text-[12px]/[20px] font-medium">
            {credential.display_name}{credential.issuer_name ? ` · ${credential.issuer_name}` : ''}
          </p>
        </div>
        {#each fields as field}
          <div class="flex items-center gap-2.5 px-[14px] py-1.5">
            <CheckRegularIcon class="size-[14px] shrink-0 text-green-700 dark:text-green-400" />
            <p class="min-w-0 grow text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{field.label}</p>
            <p class="min-w-0 text-right text-[13px]/[24px] font-medium break-words text-slate-800 dark:text-grey">
              {field.value}
            </p>
          </div>
        {/each}
      {/each}
    </div>

    {#if notShared.length > 0}
      <div class="flex flex-col rounded-xl border border-grey bg-white pb-2.5 dark:border-slate-600 dark:bg-dark">
        <div class="flex items-center gap-1.5 px-[14px] pt-[14px] pb-0.5">
          <EyeSlashRegularIcon class="size-4 text-slate-800 dark:text-grey" />
          <p class="text-[13px] font-semibold text-slate-800 dark:text-grey">
            {$LL.SCAN.SHARE_CREDENTIALS.REVIEW.NOT_SHARED()}
          </p>
        </div>
        {#each notShared as item}
          <div class="flex items-center gap-2.5 px-[14px] py-1.5 text-slate-500 dark:text-slate-300">
            <EyeSlashRegularIcon class="size-[14px] shrink-0" />
            <p class="min-w-0 grow truncate text-[13px]/[24px] font-medium">{item.label}</p>
            <p class="shrink-0 text-[12px]/[20px] font-medium text-slate-400">{item.detail}</p>
          </div>
        {/each}
      </div>
    {/if}
  </div>

  <div class="sticky bottom-0 left-0 flex flex-col space-y-[10px] rounded-t-2xl bg-white p-6 dark:bg-dark">
    <Button
      label={$LL.SCAN.SHARE_CREDENTIALS.REVIEW.SHARE({ name: clientName })}
      on:click={() => dispatch('share')}
      {loading}
    />
    <Button
      label={$LL.SCAN.SHARE_CREDENTIALS.DECLINE()}
      variant="secondary"
      on:click={() => dispatch('decline')}
      disabled={loading}
    />
  </div>
</div>

<style>
  .safe-area-height {
    height: calc(100vh - var(--safe-area-inset-top) - var(--safe-area-inset-bottom));
  }
</style>
