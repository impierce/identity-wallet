<script lang="ts">
  import { goto } from '$app/navigation';

  import type { TrustAnchor } from '@bindings/trust_anchors/TrustAnchor';

  import { Button, Switch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CaretRightBoldIcon, InfoRegularIcon } from '$lib/icons';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import { summary } from './trustAnchor';

  const navigateUp = useParentRoute('/me/settings/app');

  const anchors = $derived($appState.trust_anchors.anchors);
  const curated = $derived(anchors.filter((anchor) => anchor.source.type === 'curated'));
  const added = $derived(anchors.filter((anchor) => anchor.source.type === 'added'));
  const locale = $derived($appState.profile_settings.locale);
</script>

{#snippet anchorRow(anchor: TrustAnchor)}
  <div data-testid="trust-anchor" class="flex w-full items-center gap-3 rounded-xl bg-white p-4 dark:bg-dark">
    <button
      class="flex min-w-0 grow items-center gap-2 text-left"
      onclick={() => goto(`/me/settings/app/trust-anchors/${anchor.id}`)}
    >
      <div class="flex min-w-0 grow flex-col">
        <p class="text-[13px]/[20px] font-medium break-words text-slate-800 dark:text-white">{anchor.name}</p>
        <p class="text-[12px]/[18px] font-medium text-slate-500 dark:text-slate-300">{summary(anchor, locale)}</p>
      </div>
      <CaretRightBoldIcon class="h-4 w-4 shrink-0 text-slate-500" />
    </button>
    <Switch
      checked={anchor.enabled}
      onCheckedChange={({ next }) => {
        dispatch({ type: '[Trust Anchors] Toggle', payload: { id: anchor.id } });
        return next;
      }}
    >
      <span class="sr-only">{anchor.enabled ? 'Disable' : 'Enable'} {anchor.name}</span>
    </Switch>
  </div>
{/snippet}

<TopNavBar on:back={navigateUp} title="Trust anchors" class="sticky top-0 z-10" />

<div class="flex flex-col gap-[15px] bg-silver px-4 py-5 dark:bg-navy">
  <!-- Developer info -->
  <div class="flex w-full items-center rounded-lg bg-white px-4 py-4 dark:bg-dark">
    <span class="mr-4 h-6 w-6 shrink-0">
      <InfoRegularIcon class="h-6 w-6 text-primary" />
    </span>
    <div class="flex flex-col">
      <p class="text-[13px]/[24px] font-medium text-slate-800 dark:text-grey">Developer info</p>
      <ul class="ml-3 list-disc text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        <li>
          An issuer signing under an X.509 certificate chain is verified when the chain leads to an enabled anchor.
        </li>
        <li>Impierce curates anchors for interoperability testing. They can be enabled and disabled.</li>
        <li>Add your own anchor from the URL of a CA certificate, in PEM or DER.</li>
      </ul>
    </div>
  </div>

  <div class="flex flex-col gap-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Curated by Impierce</p>
    {#each curated as anchor (anchor.id)}
      {@render anchorRow(anchor)}
    {:else}
      <p class="py-4 text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
        No curated anchors in this version.
      </p>
    {/each}
  </div>

  <div class="flex flex-col gap-[10px]">
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Added by you</p>
    {#each added as anchor (anchor.id)}
      {@render anchorRow(anchor)}
    {:else}
      <p class="py-4 text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
        You haven't added any anchors.
      </p>
    {/each}
  </div>

  <Button label="Add trust anchor" on:click={() => goto('/me/settings/app/trust-anchors/add')} />
</div>
