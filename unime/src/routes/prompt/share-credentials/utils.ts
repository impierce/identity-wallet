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
