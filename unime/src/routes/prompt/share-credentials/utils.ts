import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { OptionState } from '@bindings/user_prompt/OptionState';
import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

export const isBadge = (credential: DisplayCredential) =>
  credential.data?.type?.includes('OpenBadgeCredential') ?? false;

// Expired credentials can be chosen after a warning, unavailable ones (revoked, suspended, not yet valid) cannot.
export const isSelectable = (state: OptionState | undefined) => state === 'valid' || state === 'expired';

/** A readable name for the requested type, e.g. "Open Badge Credential" for `OpenBadgeCredential`. */
export function typeLabel(requested: RequestedCredential): string {
  const type = requested.types.find((t) => t !== 'VerifiableCredential') ?? requested.types[0] ?? requested.query_id;
  const name = type.split(/[/#:]/).pop() || type;
  return name.replace(/([a-z])([A-Z])/g, '$1 $2').replace(/[_-]/g, ' ');
}

const plainText = (value: unknown) => (typeof value === 'string' || typeof value === 'number' ? String(value) : '');

/** The first claim value that differs between the options, which tells otherwise identical credentials apart. */
export function distinguishingClaim(
  credential: DisplayCredential,
  candidates: DisplayCredential[],
): string | undefined {
  const others = candidates.filter((c) => c.id !== credential.id);
  if (others.length === 0) return undefined;
  const claim = credential.display_claims.find((claim) => {
    const value = plainText(claim.value);
    const samePath = (c: DisplayCredential) =>
      c.display_claims.find((other) => other.path.join('/') === claim.path.join('/'));
    return value && value !== credential.display_name && others.some((c) => plainText(samePath(c)?.value) !== value);
  });
  return claim ? plainText(claim.value) : undefined;
}
