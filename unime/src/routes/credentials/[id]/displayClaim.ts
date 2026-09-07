import type { DisplayClaim } from '@bindings/credentials/DisplayClaim';

export function getDisplayClaimKey(displayClaim: DisplayClaim): string {
  if (displayClaim.key.trim().length > 0) {
    return displayClaim.key;
  }

  return [...displayClaim.path].reverse().find((pathElement) => pathElement.trim().length > 0) ?? '';
}
