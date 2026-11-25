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

/** One line on `anchor` for a list: when its certificate expires. */
export function summary(anchor: TrustAnchor, locale: string, now: number = Date.now()): string {
  return isExpired(anchor, now)
    ? 'Expired or not yet valid'
    : `Valid until ${formatCertificateDate(anchor.details.not_after, locale)}`;
}

/** One line on a list of anchors for the list of lists: how many there are, and how many of them are enabled. */
export function listSummary(anchors: TrustAnchor[]): string {
  const enabled = anchors.filter((anchor) => anchor.enabled).length;
  if (anchors.length === 0) return 'Empty list';
  if (anchors.length === 1) return enabled === 1 ? '1 anchor, enabled' : '1 anchor, disabled';
  if (enabled === anchors.length) return `${anchors.length} anchors, all enabled`;
  if (enabled === 0) return `${anchors.length} anchors, none enabled`;
  return `${enabled} of ${anchors.length} anchors enabled`;
}

export const refusalText: Record<TrustAnchorRefusal, string> = {
  not_ca: "This is not a CA certificate, so it can't be a trust anchor: its key signs credentials, not certificates.",
  expired: 'This certificate has expired, so no certificate chain validates against it.',
  not_yet_valid: "This certificate isn't valid yet, so no certificate chain validates against it.",
  already_trusted: 'This certificate is already one of your trust anchors.',
};
