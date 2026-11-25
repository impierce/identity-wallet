<script lang="ts">
  import { page } from '$app/state';

  import { Button, SettingsSwitch, TopNavBar } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { SealCheckFillIcon } from '$lib/icons';
  import { useParentRoute } from '$lib/navigation';
  import { state as appState } from '$lib/stores';

  import TrustAnchorDetails from '../TrustAnchorDetails.svelte';

  const navigateUp = useParentRoute('/me/settings/app/trust-anchors');

  const anchor = $derived($appState.trust_anchors.anchors.find((anchor) => anchor.id === page.params.id));
</script>

<TopNavBar on:back={navigateUp} title={anchor?.name ?? 'Trust anchor'} class="sticky top-0 z-10" />

<div class="flex flex-col gap-[15px] bg-silver px-4 py-5 dark:bg-navy">
  {#if anchor}
    <SettingsSwitch
      checked={anchor.enabled}
      onCheckedChange={({ next }) => {
        dispatch({ type: '[Trust Anchors] Toggle', payload: { id: anchor.id } });
        return next;
      }}
    >
      {#snippet icon()}
        <SealCheckFillIcon class="h-5 w-5 text-primary" />
      {/snippet}
      Enabled
    </SettingsSwitch>

    <TrustAnchorDetails {anchor} />

    {#if anchor.source.type === 'added'}
      <Button
        label="Remove trust anchor"
        variant="secondary"
        on:click={async () => {
          await dispatch({ type: '[Trust Anchors] Remove', payload: { id: anchor.id } });
          navigateUp();
        }}
      />
    {:else}
      <p class="text-center text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">
        A curated anchor can be disabled, but not removed.
      </p>
    {/if}
  {:else}
    <p class="py-4 text-center text-[13px]/[24px] font-medium text-slate-500 dark:text-slate-300">
      This trust anchor no longer exists.
    </p>
  {/if}
</div>
