import { calculateInitials } from './utils';

describe('app', () => {
  test('calculates initials correctly', () => {
    expect(calculateInitials('Ferris Rustacean')).toBe('FR');
    expect(calculateInitials('Ferris')).toBe('FE');
    expect(calculateInitials('F')).toBe('F');
    expect(calculateInitials('Ferris   Rustacean')).toBe('FR');
    expect(calculateInitials('Ferris rustacean')).toBe('FR');
    expect(calculateInitials('')).toBe('??');
  });
});
