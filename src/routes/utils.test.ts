import { describe, expect, it } from 'vitest';

import { determineTheme } from './utils';

describe('app', () => {
  it('determines correct theme', () => {
    expect(determineTheme(true, 'system')).toBe('dark');
    expect(determineTheme(true, 'dark')).toBe('dark');
    expect(determineTheme(true, 'light')).toBe('light');
    expect(determineTheme(false, 'system')).toBe('light');
    expect(determineTheme(false, 'dark')).toBe('dark');
    expect(determineTheme(false, 'light')).toBe('light');
  });
});
