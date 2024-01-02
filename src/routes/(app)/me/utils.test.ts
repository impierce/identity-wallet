import { describe, expect, it } from 'vitest';

import { calculate_initials } from './utils';

describe('Me', () => {
  it('calculates initials correctly', () => {
    expect(calculate_initials('Ferris Rustacean')).toBe('FR');
    expect(calculate_initials('Ferris')).toBe('FE');
    expect(calculate_initials('F')).toBe('F');
    expect(calculate_initials('Ferris   Rustacean')).toBe('FR');
  });
});
