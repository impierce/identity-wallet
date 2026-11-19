// Monogram and tint for an ecosystem or one of its members, both derived from the name.

import { hashIndex } from '$lib/utils';

// Whole class strings: Tailwind only emits classes it finds literally in the source.
const TINTS = [
  'bg-credentials-0',
  'bg-credentials-1',
  'bg-credentials-2',
  'bg-credentials-3',
  'bg-credentials-4',
  'bg-credentials-5',
  'bg-credentials-6',
  'bg-credentials-7',
];

// Every tint is pale, so what sits on one needs dark initials. Shipped with the tint because
// `color` inherits: tinting an element inks its children too. No `dark:` -- the tint does not
// change with the theme, so the ink must not either.
const INK = 'text-slate-800';

// Keyed on the name, not list position. The designs cycle by position, but `ecosystems` is
// per-connection, so the same ecosystem sits at a different index under each verifier and would
// change colour between connections. Repeats within one list are the accepted cost.
export function tintFor(name: string): string {
  return `${TINTS[hashIndex(name, TINTS.length)]} ${INK}`;
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
