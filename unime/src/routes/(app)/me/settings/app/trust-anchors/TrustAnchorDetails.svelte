<script lang="ts">
  import type { TrustAnchor } from '@bindings/trust_anchors/TrustAnchor';

  import { state as appState } from '$lib/stores';

  import { formatCertificateDate, isExpired } from './trustAnchor';

  let { anchor }: { anchor: TrustAnchor } = $props();

  const locale = $derived($appState.profile_settings.locale);
  const details = $derived(anchor.details);
</script>

{#snippet row(label: string, value: string, mono = false)}
  <div class="flex flex-col gap-1 py-3">
    <p class="text-[12px]/[20px] font-medium text-slate-500 dark:text-slate-300">{label}</p>
    <p
      class="text-[13px]/[20px] font-medium text-slate-800 dark:text-white {mono
        ? 'font-mono break-all'
        : 'wrap-break-word'}"
    >
      {value}
    </p>
  </div>
{/snippet}

<div
  data-testid="trust-anchor-details"
  class="flex flex-col divide-y divide-slate-200 rounded-xl bg-white px-4 dark:divide-slate-600 dark:bg-dark"
>
  {@render row(
    'Source',
    anchor.source.type === 'curated'
      ? 'Curated by Impierce'
      : anchor.source.url
        ? `Added from ${anchor.source.url}`
        : 'Added by you',
  )}
  {@render row('Subject', details.subject)}
  {@render row('Issuer', details.self_issued ? `${details.issuer} (self-issued: a root CA)` : details.issuer)}
  {@render row(
    'Valid',
    `${formatCertificateDate(details.not_before, locale)} to ${formatCertificateDate(details.not_after, locale)}${
      isExpired(anchor) ? ' (not valid now)' : ''
    }`,
  )}
  {@render row(
    'Name constraints',
    details.permitted_dns_names.length === 0 && details.excluded_dns_names.length === 0
      ? 'None: it may certify any DNS name'
      : [
          details.permitted_dns_names.length > 0 ? `Permitted: ${details.permitted_dns_names.join(', ')}` : '',
          details.excluded_dns_names.length > 0 ? `Excluded: ${details.excluded_dns_names.join(', ')}` : '',
        ]
          .filter(Boolean)
          .join(' · '),
  )}
  {@render row('SHA-256 fingerprint', details.sha256_fingerprint, true)}
</div>
