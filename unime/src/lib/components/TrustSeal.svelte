<script lang="ts">
  import LL from '$i18n/i18n-svelte';

  import { CheckBoldIcon, CheckCircleFillIcon, WarningCircleFillIcon, WarningRegularIcon } from '$lib/icons';
  // TEMP: swap to `@bindings/user_prompt/TrustVerification` once the backend struct lands.
  import type { TrustVerification } from '$lib/types/trust';

  export let trust: TrustVerification;

  let open = false;
  let root: HTMLElement;

  const verified = () => trust.status === 'Verified';

  function toggle() {
    open = !open;
  }

  function onWindowClick(e: MouseEvent) {
    if (open && root && !root.contains(e.target as Node)) open = false;
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') open = false;
  }
</script>

<svelte:window on:click={onWindowClick} on:keydown={onKeydown} />

<!--
@component
Overlays a trust seal on the issuer's mark. The seal is the affordance, not the message: the
ecosystem name and cache status are shown inline on the page. Tapping the seal opens a short
explainer of what "verified" actually means.

### Props
- trust: `TrustVerification`

### Slots
- default: the issuer's mark (logo or fallback) the seal is pinned to.
-->
<div class="relative inline-flex" bind:this={root}>
  <slot />

  <div class="absolute -right-1.5 -bottom-1.5">
    <button
      type="button"
      class="flex h-8 w-8 items-center justify-center rounded-full ring-[3px] ring-silver transition-transform active:scale-95 dark:ring-navy
        {verified() ? 'bg-emerald-500' : 'bg-rose-500'}"
      aria-expanded={open}
      aria-label={$LL.SCAN.CREDENTIAL_OFFER.TRUST.EXPLAINER.TITLE()}
      on:click|stopPropagation={toggle}
    >
      {#if verified()}
        <CheckBoldIcon class="h-[18px] w-[18px] text-white" stroke-width="3" />
      {:else}
        <WarningRegularIcon class="h-[18px] w-[18px] text-white" />
      {/if}
    </button>

    {#if open}
      <div
        class="absolute top-full left-1/2 z-10 mt-2 w-[228px] -translate-x-1/2 rounded-xl border border-slate-200/80 bg-white px-3.5 py-3 text-left shadow-[0_6px_20px_-6px_rgba(15,23,42,0.22)] dark:border-slate-700 dark:bg-slate-800"
        role="dialog"
      >
        <!-- Caret tying the card back to the seal. -->
        <div
          class="absolute -top-[5px] left-1/2 h-2 w-2 -translate-x-1/2 rotate-45 border-t border-l border-slate-200/80 bg-white dark:border-slate-700 dark:bg-slate-800"
        ></div>

        <!-- The verdict. Self-contained: it always states who vouches for the issuer, because the
             inline line on the page may be showing prior-trust instead. -->
        <div class="flex items-start gap-1.5">
          {#if verified()}
            <CheckCircleFillIcon class="mt-px h-3.5 w-3.5 shrink-0 text-emerald-500" />
          {:else}
            <WarningCircleFillIcon class="mt-px h-3.5 w-3.5 shrink-0 text-rose-500" />
          {/if}
          <p class="text-[12px]/[16px] font-semibold text-slate-800 dark:text-grey">
            {#if verified() && trust.ecosystem_name}
              {$LL.SCAN.CREDENTIAL_OFFER.TRUST.TRUSTED_BY({ network: trust.ecosystem_name })}
            {:else if trust.status === 'Invalid'}
              {$LL.SCAN.CREDENTIAL_OFFER.TRUST.INVALID()}
            {:else}
              {$LL.SCAN.CREDENTIAL_OFFER.TRUST.UNKNOWN()}
            {/if}
          </p>
        </div>

        <!-- Divider: above is what UniMe found, below is what that finding means. -->
        <div class="my-2.5 border-t border-slate-100 dark:border-slate-700"></div>

        <p class="text-[11px]/[15px] font-semibold text-slate-600 dark:text-slate-300">
          {$LL.SCAN.CREDENTIAL_OFFER.TRUST.EXPLAINER.TITLE()}
        </p>
        <p class="mt-0.5 text-[11px]/[16px] text-slate-500 dark:text-slate-400">
          {verified()
            ? $LL.SCAN.CREDENTIAL_OFFER.TRUST.EXPLAINER.VERIFIED()
            : $LL.SCAN.CREDENTIAL_OFFER.TRUST.EXPLAINER.UNVERIFIED()}
        </p>
      </div>
    {/if}
  </div>
</div>
