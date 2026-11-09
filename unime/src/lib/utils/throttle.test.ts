import { createThrottle } from './throttle';

describe('createThrottle', () => {
  test('lets the first call through, then none until the interval has passed', () => {
    const mayRun = createThrottle(1000);

    expect(mayRun(0)).toBe(true);
    expect(mayRun(1)).toBe(false);
    expect(mayRun(999)).toBe(false);
    expect(mayRun(1000)).toBe(true);
    expect(mayRun(1500)).toBe(false);
  });
});
