<script lang="ts">
  import { onDestroy } from 'svelte';

  import { goto } from '$app/navigation';

  import { Button, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import { refusalText } from '../trustAnchor';
  import TrustAnchorDetails from '../TrustAnchorDetails.svelte';

  const navigateUp = useParentRoute('/me/settings/app/trust-anchors');

  let url = $state('');
  let loading = $state(false);

  const pending = $derived($appState.trust_anchors.pending);

  async function fetchCertificate() {
    loading = true;
    try {
      await dispatch({ type: '[Trust Anchors] Fetch', payload: { url: url.trim() } });
    } finally {
      loading = false;
    }
  }

  // A certificate under review is discarded when the screen is left without accepting it.
  onDestroy(() => {
    if ($appState.trust_anchors.pending) dispatch({ type: '[Trust Anchors] Deny' });
  });
</script>

<TopNavBar on:back={navigateUp} title="Add trust anchor" class="sticky top-0 z-10" />

<div class="flex flex-col gap-[15px] bg-silver px-4 py-5 dark:bg-navy">
  {#if !pending}
    <p class="text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
      Enter the URL of a CA certificate, in PEM or DER. UniMe shows it to you before you decide whether to trust it.
    </p>
    <input
      type="url"
      inputmode="url"
      autocomplete="off"
      autocapitalize="off"
      spellcheck="false"
      class="h-12 w-full rounded-xl border border-slate-200 bg-white px-3 text-[13px]/[24px] text-slate-800 dark:border-slate-600 dark:bg-dark dark:text-white"
      placeholder="https://ca.example/root.crt"
      bind:value={url}
      onkeydown={(event) => {
        if (event.key === 'Enter' && url.trim() && !loading) fetchCertificate();
      }}
    />
    <Button label="Fetch certificate" on:click={fetchCertificate} disabled={!url.trim() || loading} {loading} />
  {:else}
    <p class="text-[16px]/[24px] font-semibold break-words text-slate-800 dark:text-white">{pending.anchor.name}</p>

    {#if pending.refusal}
      <p
        data-testid="trust-anchor-refusal"
        class="rounded-xl bg-rose-50 p-4 text-[13px]/[20px] font-medium text-rose-700 dark:bg-rose-950 dark:text-rose-300"
      >
        {refusalText[pending.refusal]}
      </p>
    {:else}
      <p class="text-[13px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        Trust this certificate only if you know who controls it: every issuer it certifies is verified as the name in
        its certificate.
      </p>
    {/if}

    <TrustAnchorDetails anchor={pending.anchor} />

    {#if pending.refusal}
      <Button label="Close" variant="secondary" on:click={() => dispatch({ type: '[Trust Anchors] Deny' })} />
    {:else}
      <Button
        label="Accept"
        on:click={async () => {
          await dispatch({ type: '[Trust Anchors] Accept' });
          goto('/me/settings/app/trust-anchors/added', { replaceState: true });
        }}
      />
      <Button label="Deny" variant="secondary" on:click={() => dispatch({ type: '[Trust Anchors] Deny' })} />
    {/if}
  {/if}
</div>
