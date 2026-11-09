import { pageTransitionDirection } from './pageTransitions';

describe('pageTransitionDirection', () => {
  test('goes forward from a list to its detail page and back the other way', () => {
    expect(pageTransitionDirection('/(app)/me', '/credentials/[id]')).toBe('forward');
    expect(pageTransitionDirection('/credentials/[id]', '/(app)/me')).toBe('back');
    expect(pageTransitionDirection('/(app)/activity', '/(app)/activity/connection/[id]')).toBe('forward');
    expect(pageTransitionDirection('/(app)/activity/connection/[id]', '/(app)/activity')).toBe('back');
  });

  test('leaves every other navigation without a transition', () => {
    expect(pageTransitionDirection('/(app)/me', '/(app)/activity')).toBeUndefined();
    expect(pageTransitionDirection('/(app)/activity', '/credentials/[id]')).toBeUndefined();
    expect(pageTransitionDirection(null, '/(app)/me')).toBeUndefined();
    expect(pageTransitionDirection('/(app)/me', '/(app)/me')).toBeUndefined();
  });
});
