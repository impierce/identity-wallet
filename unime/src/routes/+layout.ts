// Turn on SPA mode.
export const ssr = false;

// We don't really care about prerendering because Tauri always enters the app via `/`.
// But adapter-static is not happy unless we set this to `true`.
export const prerender = true;
