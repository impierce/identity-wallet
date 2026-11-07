/**
 * Measures how often the WebView runs `requestAnimationFrame` callbacks over `durationMs`, in frames per second.
 *
 * Scrolling and CSS animations of `transform` and `opacity` are composited natively and may run at a higher rate than
 * this, so it is a lower bound for what the user sees. Used in development builds to check the display's frame rate.
 */
export function measureFrameRate(durationMs = 1000): Promise<number> {
  return new Promise((resolve) => {
    let frames = 0;
    let start: number | undefined;
    const tick = (now: number) => {
      start ??= now;
      frames += 1;
      if (now - start < durationMs) {
        requestAnimationFrame(tick);
      } else {
        resolve(Math.round(((frames - 1) * 1000) / (now - start)));
      }
    };
    requestAnimationFrame(tick);
  });
}
