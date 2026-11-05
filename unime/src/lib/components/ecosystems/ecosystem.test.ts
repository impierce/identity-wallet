import { calculateOrgInitials, tintFor } from './ecosystem';

describe('calculateOrgInitials', () => {
  test('takes one letter per word', () => {
    expect(calculateOrgInitials('Global FinTech Alliance')).toBe('GFA');
    expect(calculateOrgInitials('Open Banking Consortium')).toBe('OBC');
    expect(calculateOrgInitials('Nordic Trust Framework')).toBe('NTF');
  });

  test('skips words that carry no identity', () => {
    expect(calculateOrgInitials('Dutch Organization for Universities')).toBe('DOU');
    expect(calculateOrgInitials('University of Harderwijk')).toBe('UH');
  });

  test('caps at three letters, so the monogram still fits the smallest badge', () => {
    expect(calculateOrgInitials('EU Digital Identity Network')).toBe('EDI');
    expect(calculateOrgInitials('Stichting Nederlandse Organisatie voor Wetenschappelijk Onderzoek')).toBe('SNO');
  });

  test('takes two letters from a single-word name, so the badge is not a lone letter', () => {
    expect(calculateOrgInitials('Optimizor')).toBe('OP');
    // Matches `calculateInitials` in `$lib/utils`, which does the same for one-word people.
    expect(calculateOrgInitials('Nictiz')).toBe('NI');
  });

  test('ignores punctuation between words', () => {
    expect(calculateOrgInitials('Acme, Ltd.')).toBe('AL');
    expect(calculateOrgInitials('Finara-NL')).toBe('FN');
  });

  test('keeps digits, which appear in framework names', () => {
    expect(calculateOrgInitials('7510 Health Network')).toBe('7HN');
  });

  test('falls back to the first character when every word is a stop word', () => {
    expect(calculateOrgInitials('the of and')).toBe('T');
  });

  test('renders something rather than nothing for a name with no letters', () => {
    expect(calculateOrgInitials('!!!')).toBe('!');
    expect(calculateOrgInitials('')).toBe('?');
  });
});

describe('tintFor', () => {
  test('gives the same name the same tint every time, so a mark is stable across screens', () => {
    expect(tintFor('Global FinTech Alliance')).toBe(tintFor('Global FinTech Alliance'));
  });

  test('pairs the pale background with dark ink, so the initials stay legible on it', () => {
    expect(tintFor('Dutch Organization for Universities')).toMatch(/^bg-credentials-[0-7] text-slate-800$/);
  });

  test('spreads a handful of names across more than one tint', () => {
    const names = [
      'Dutch Organization for Universities',
      'Global FinTech Alliance',
      'EU Digital Identity Network',
      'Open Banking Consortium',
      'Nordic Trust Framework',
      'Healthcare Data Alliance',
    ];
    const distinct = new Set(names.map(tintFor));
    expect(distinct.size).toBeGreaterThan(1);
  });

  test('does not fall over on an empty name', () => {
    expect(tintFor('')).toBeTruthy();
  });
});
