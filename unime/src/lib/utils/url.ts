export const HTTPS_URL_PART_REGEX = /(https:\/\/\S+)/g;

export function isUrl(text: string): boolean {
  try {
    const url = new URL(text);
    return url.protocol === 'https:';
  } catch {
    return false;
  }
}

/**
 * The hostname of `text` (e.g. `iso.org`), or `undefined` when it does not parse as a URL.
 *
 * Backend fields typed `url::Url` serialize as absolute URLs (`https://iso.org/`), but the
 * designs show a bare hostname.
 */
export function hostname(text: string): string | undefined {
  try {
    return new URL(text).hostname;
  } catch {
    return undefined;
  }
}
