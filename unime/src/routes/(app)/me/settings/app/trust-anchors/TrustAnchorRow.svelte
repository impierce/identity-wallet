<script lang="ts">
  import { goto } from '$app/navigation';

  import type { TrustAnchor } from '@bindings/trust_anchors/TrustAnchor';

  import { Switch } from '$lib/components';
  import { dispatch } from '$lib/dispatcher';
  import { CaretRightBoldIcon } from '$lib/icons';
  import { state as appState } from '$lib/stores';

  import { summary } from './trustAnchor';

  let { anchor }: { anchor: TrustAnchor } = $props();

  const locale = $derived($appState.profile_settings.locale);
</script>

<div data-testid="trust-anchor" class="flex w-full items-center gap-3 rounded-xl bg-white p-4 dark:bg-dark">
  <button
    class="flex min-w-0 grow items-center gap-2 text-left"
    onclick={() => goto(`/me/settings/app/trust-anchors/${anchor.id}`)}
  >
    <div class="flex min-w-0 grow flex-col">
      <p class="text-[13px]/[20px] font-medium wrap-break-word text-slate-800 dark:text-white">{anchor.name}</p>
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
