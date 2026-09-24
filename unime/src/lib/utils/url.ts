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

/**
 * User-facing hostname for a connection. Older stored connections can lack a URL scheme,
 * and the infrastructure-specific `identity.` prefix is not useful in the UI.
 */
export function connectionHostname(text: string): string {
  const parsedHostname = hostname(text) ?? hostname(`https://${text}`);
  return (parsedHostname ?? text).replace(/^identity\./i, '');
}

/** A browser-safe HTTP(S) URL for a stored connection URL, including legacy values without a scheme. */
export function connectionBrowserUrl(text: string): string | undefined {
  if (/^[a-z][a-z\d+.-]*:/i.test(text) && !/^https?:/i.test(text)) return undefined;

  const candidates = [text, `https://${text}`];

  for (const candidate of candidates) {
    try {
      const url = new URL(candidate);
      if (url.protocol === 'http:' || url.protocol === 'https:') return url.toString();
    } catch {
      // Try the next representation.
    }
  }

  return undefined;
}
