import type { TrustAnchor } from '@bindings/trust_anchors/TrustAnchor';
import type { TrustAnchorRefusal } from '@bindings/trust_anchors/TrustAnchorRefusal';

/** A date of a certificate's validity period, given in seconds since the Unix epoch. */
export function formatCertificateDate(seconds: number, locale: string): string {
  return new Intl.DateTimeFormat(locale, { dateStyle: 'medium', timeZone: 'UTC' }).format(new Date(seconds * 1000));
}

/** Whether the certificate of `anchor` is outside its validity period at `now` (milliseconds since the Unix epoch). */
export function isExpired(anchor: TrustAnchor, now: number = Date.now()): boolean {
  return now / 1000 > anchor.details.not_after || now / 1000 < anchor.details.not_before;
}

/** One line on `anchor` for a list: when it expires, and for a curated one whether that is its default. */
export function summary(anchor: TrustAnchor, locale: string, now: number = Date.now()): string {
  const validity = isExpired(anchor, now)
    ? 'Expired or not yet valid'
    : `Valid until ${formatCertificateDate(anchor.details.not_after, locale)}`;
  if (anchor.source.type === 'curated' && anchor.enabled !== anchor.source.enabled_by_default) {
    return `${validity} · ${anchor.source.enabled_by_default ? 'on' : 'off'} by default`;
  }
  return validity;
}

export const refusalText: Record<TrustAnchorRefusal, string> = {
  not_ca: "This is not a CA certificate, so it can't be a trust anchor: its key signs credentials, not certificates.",
  expired: 'This certificate has expired, so no certificate chain validates against it.',
  not_yet_valid: "This certificate isn't valid yet, so no certificate chain validates against it.",
  already_trusted: 'This certificate is already one of your trust anchors.',
};
