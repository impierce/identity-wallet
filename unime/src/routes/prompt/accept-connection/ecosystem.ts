// Monogram and tint for an ecosystem or one of its members, both derived from the name.

import { hashIndex } from '$lib/utils';

// Whole class strings: Tailwind only emits classes it finds literally in the source.
const TINTS = [
  { badge: 'bg-violet-600', banner: 'from-violet-600 to-violet-500' },
  { badge: 'bg-blue-600', banner: 'from-blue-600 to-blue-500' },
  { badge: 'bg-amber-500', banner: 'from-amber-500 to-amber-400' },
  { badge: 'bg-emerald-500', banner: 'from-emerald-500 to-emerald-400' },
  { badge: 'bg-red-500', banner: 'from-red-500 to-red-400' },
  { badge: 'bg-fuchsia-600', banner: 'from-fuchsia-600 to-fuchsia-500' },
] as const;

type Tint = (typeof TINTS)[number];

// Keyed on the name, not list position. The designs cycle by position, but `ecosystems` is
// per-connection, so the same ecosystem sits at a different index under each verifier and would
// change colour between connections. Repeats within one list are the accepted cost.
export function tintFor(name: string): Tint {
  return TINTS[hashIndex(name, TINTS.length)];
}

// "Dutch Organization for Universities" is DOU, not DOFU. Not localised: names come from the
// ecosystem, not the wallet's locale.
const STOP_WORDS = new Set(['of', 'for', 'the', 'and', 'a', 'an', 'in', 'on', 'von', 'de', 'van']);

// "Global FinTech Alliance" -> "GFA". One letter per significant word, max three; single-word
// names take two. Not `calculateInitials`, which caps at two and renders "University of
// Harderwijk" as "UO".
export function calculateOrgInitials(name: string): string {
  const words = (name.match(/[\p{L}\p{N}]+/gu) ?? []).filter((word) => !STOP_WORDS.has(word.toLowerCase()));

  // All stop words, or no letters at all.
  if (words.length === 0) {
    return (name.trim()[0] ?? '?').toUpperCase();
  }

  if (words.length === 1) {
    return words[0].slice(0, 2).toUpperCase();
  }

  return words
    .slice(0, 3)
    .map((word) => word[0])
    .join('')
    .toUpperCase();
}
