// Monogram and tint for an ecosystem or one of its members, both derived from the name.

import { hashIndex } from '$lib/utils';

// Whole class strings: Tailwind only emits classes it finds literally in the source.
//
// The `eco-*` colours are ours, defined in `app.css`, not Tailwind's ramps. Tailwind's ramps vary
// wildly in chroma between hues -- `fuchsia-600` carries 0.293 against `emerald-500`'s 0.170 -- so
// a palette built from them has one or two tints that shout while the rest sit quietly, and which
// ecosystem shouts is down to a hash. Ours hold lightness and chroma fixed and vary only hue.
const TINTS = [
  { badge: 'bg-eco-indigo-base', banner: 'from-eco-indigo-base to-eco-indigo-soft' },
  { badge: 'bg-eco-blue-base', banner: 'from-eco-blue-base to-eco-blue-soft' },
  { badge: 'bg-eco-ochre-base', banner: 'from-eco-ochre-base to-eco-ochre-soft' },
  { badge: 'bg-eco-green-base', banner: 'from-eco-green-base to-eco-green-soft' },
  { badge: 'bg-eco-terracotta-base', banner: 'from-eco-terracotta-base to-eco-terracotta-soft' },
  { badge: 'bg-eco-plum-base', banner: 'from-eco-plum-base to-eco-plum-soft' },
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
