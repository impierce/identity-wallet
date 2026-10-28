import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { Locale } from '@bindings/profile_settings/Locale';
import type { CredentialOption } from '@bindings/user_prompt/CredentialOption';
import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

import { formatIso8601 } from '$lib/utils/date';

/** A field of a credential on the share screen. `index` points into `RequestedCredential.claims`, if requested. */
export interface ClaimRow {
  index?: number;
  label: string;
  value: unknown;
  required: boolean;
}

type ClaimPath = RequestedCredential['claims'][number];

const samePath = (a: ClaimPath, b: ClaimPath) => JSON.stringify(a) === JSON.stringify(b);

/** A readable name for a claim name, e.g. "Given name" for `given_name` or "Date of birth" for `dateOfBirth`. */
export function humanize(name: string): string {
  const words = name
    .replace(/([a-z])([A-Z])/g, '$1 $2')
    .replace(/[_-]+/g, ' ')
    .trim()
    .toLowerCase();
  return words.charAt(0).toUpperCase() + words.slice(1);
}

function claimLabel(path: ClaimPath, key?: string): string {
  const name = [...path].reverse().find((element): element is string => typeof element === 'string');
  // Without display metadata from the issuer, the key is the raw claim name.
  if (key && key !== name) return key;
  return name ? humanize(name) : '';
}

/** The value of a claim as one line of text. */
export function claimText(value: unknown, locale: Locale): string {
  if (typeof value === 'string') return formatIso8601(value, locale);
  if (typeof value === 'number' || typeof value === 'boolean') return String(value);
  if (Array.isArray(value)) return value.map((item) => claimText(item, locale)).join(', ');
  if (value && typeof value === 'object') {
    return Object.values(value)
      .map((item) => claimText(item, locale))
      .filter(Boolean)
      .join(', ');
  }
  return '';
}

// The leaf values of the credential subject, for credentials that hold no display claims.
function subjectRows(value: unknown, path: string[] = []): ClaimRow[] {
  if (Array.isArray(value) && value.every((item) => typeof item !== 'object' || item === null)) {
    return [{ label: humanize(path.at(-1) ?? ''), value, required: true }];
  }
  if (value && typeof value === 'object') {
    return Object.entries(value).flatMap(([key, child]) => subjectRows(child, [...path, key]));
  }
  if (path.at(-1) === 'id' || value === null || value === undefined) return [];
  return [{ label: humanize(path.at(-1) ?? ''), value, required: true }];
}

// The claim sets (by index) that the credential can satisfy. Without claim sets, all requested claims are one set.
function satisfiableSets(requested: RequestedCredential, option: CredentialOption): number[][] {
  const sets = requested.claim_sets.length > 0 ? requested.claim_sets : [requested.claims.map((_, index) => index)];
  return sets.filter((set) => set.every((index) => option.claims[index]?.value !== null));
}

// Claims that are in every set the credential can satisfy, or cannot be left out, are always shared.
function requiredClaims(requested: RequestedCredential, option: CredentialOption): Set<number> {
  const sets = satisfiableSets(requested, option);
  const offered = new Set(sets.flat());
  return new Set(
    [...offered].filter((index) => sets.every((set) => set.includes(index)) || !option.claims[index]?.concealable),
  );
}

/**
 * The fields shown for a credential. With selective disclosure these are the requested claims the holder can share;
 * otherwise the whole credential is shared, so all of its fields are shown as required.
 */
export function claimRows(
  requested: RequestedCredential,
  option: CredentialOption,
  credential: DisplayCredential,
): ClaimRow[] {
  if (!option.selective_disclosure) {
    if (credential.display_claims.length > 0) {
      return credential.display_claims.map((claim) => ({
        label: claimLabel(claim.path as ClaimPath, claim.key),
        value: claim.value,
        required: true,
      }));
    }
    return subjectRows(credential.data?.credentialSubject);
  }
  const required = requiredClaims(requested, option);
  const offered = new Set(satisfiableSets(requested, option).flat());
  return requested.claims
    .map((path, index) => ({
      index,
      label: claimLabel(path, credential.display_claims.find((claim) => samePath(claim.path as ClaimPath, path))?.key),
      value: option.claims[index]?.value,
      required: required.has(index),
    }))
    .filter((row) => offered.has(row.index));
}

/** The claims shared by default: the smallest claim set the credential can satisfy, the earliest one on a tie. */
export function defaultDisclosure(requested: RequestedCredential, option: CredentialOption): number[] {
  if (!option.selective_disclosure) return [];
  const sets = satisfiableSets(requested, option);
  const smallest = sets.reduce<number[] | undefined>(
    (best, set) => (best === undefined || set.length < best.length ? set : best),
    undefined,
  );
  return [...new Set([...(smallest ?? []), ...requiredClaims(requested, option)])].sort((a, b) => a - b);
}

/** Whether sharing the claims at `disclosed` gives the verifier one of the combinations it asked for. */
export function satisfiesRequest(
  requested: RequestedCredential,
  option: CredentialOption,
  disclosed: number[],
): boolean {
  if (!option.selective_disclosure) return true;
  return satisfiableSets(requested, option).some((set) => set.every((index) => disclosed.includes(index)));
}

// Whether `path` is the field a claim path points to, or lies inside it.
const covers = (claimPath: ClaimPath, path: string[]) =>
  claimPath.length <= path.length &&
  claimPath.every((element, index) => (element === null ? /^\d+$/.test(path[index]) : String(element) === path[index]));

/**
 * Fields that are not requested but are shared anyway with the claims at `disclosed`, because the credential discloses
 * them together.
 */
export function sharedAlongside(
  requested: RequestedCredential,
  option: CredentialOption,
  credential: DisplayCredential,
  disclosed: number[],
): ClaimRow[] {
  if (!option.selective_disclosure) return [];
  const shared = requested.claims
    .map((_, index) => index)
    .filter((index) => disclosed.includes(index) || requiredClaims(requested, option).has(index));
  const fields = new Map<string, { path: string[]; value: unknown }>();
  for (const index of shared) {
    for (const field of option.claims[index]?.shared_alongside ?? []) fields.set(JSON.stringify(field.path), field);
  }
  return [...fields.values()]
    .filter(({ path }) => !shared.some((index) => covers(requested.claims[index], path)))
    .map(({ path, value }) => ({
      label: claimLabel(path, credential.display_claims.find((claim) => samePath(claim.path as ClaimPath, path))?.key),
      value,
      required: true,
    }));
}
