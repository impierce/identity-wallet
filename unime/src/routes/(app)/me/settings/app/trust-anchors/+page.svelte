<script lang="ts">
  import { goto } from '$app/navigation';

  import { Button, TopNavBar } from '$lib/components';
  import { CaretRightBoldIcon, InfoRegularIcon } from '$lib/icons';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import { listSummary } from './trustAnchor';

  const navigateUp = useParentRoute('/me/settings/app');

  const anchors = $derived($appState.trust_anchors.anchors);
  const lists = $derived([
    {
      name: 'Curated by Impierce',
      href: '/me/settings/app/trust-anchors/curated',
      anchors: anchors.filter((anchor) => anchor.source.type === 'curated'),
    },
    {
      name: 'Added by you',
      href: '/me/settings/app/trust-anchors/added',
      anchors: anchors.filter((anchor) => anchor.source.type === 'added'),
    },
  ]);
</script>

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
    <p class="text-[14px]/[22px] font-medium text-slate-500 dark:text-slate-300">Trust lists</p>
    {#each lists as list (list.href)}
      <button
        data-testid="trust-anchor-list"
        class="flex min-h-14 w-full items-center gap-4 rounded-xl bg-white p-4 dark:bg-dark"
        onclick={() => goto(list.href)}
      >
        <p class="grow text-left text-[13px]/[24px] font-medium text-slate-800 dark:text-white">{list.name}</p>
        <p class="text-right text-[13px]/[24px] font-medium text-slate-400 dark:text-slate-300">
          {listSummary(list.anchors)}
        </p>
        <CaretRightBoldIcon class="h-4 w-4 shrink-0 text-slate-500" />
      </button>
    {/each}
  </div>

  <Button label="Add trust anchor" on:click={() => goto('/me/settings/app/trust-anchors/add')} />
</div>
