/**
 * Lets an action through at most once per `intervalMs`. The returned function answers whether the action may run now,
 * and if so, counts it as run.
 */
export function createThrottle(intervalMs: number): (now?: number) => boolean {
  let lastRun: number | undefined;
  return (now = Date.now()) => {
    if (lastRun !== undefined && now - lastRun < intervalMs) return false;
    lastRun = now;
    return true;
  };
}
