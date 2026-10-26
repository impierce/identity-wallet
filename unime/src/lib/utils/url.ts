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

const DOMAIN_REGEX = /^(?=.{1,253}$)(?:[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?\.)+[a-z]{2,63}$/i;

// Claim names that suggest a bare domain value is meant as a website.
const WEBSITE_KEY_REGEX = /web|url|domain|homepage|link|site/i;

// Punctuation that ends a sentence rather than the URL it follows.
const TRAILING_PUNCTUATION_REGEX = /[.,;:!?)\]'"]+$/;

/** An `https://` URL for a bare hostname (e.g. `iso.org`), or `undefined` when `domain` is not one. */
export function domainUrl(domain: string): string | undefined {
  return DOMAIN_REGEX.test(domain) ? new URL(`https://${domain}`).toString() : undefined;
}

/** Whether `url` can be handed to the system browser. */
export function isBrowserUrl(url: string): boolean {
  try {
    const { protocol } = new URL(url);
    return protocol === 'http:' || protocol === 'https:';
  } catch {
    return false;
  }
}

export interface TextPart {
  text: string;
  url?: string;
}

/**
 * Splits a claim value into text and the links in it. HTTPS URLs are links wherever they appear. A bare domain
 * is only a link when it is the whole value and either starts with `www.` or the claim is named like a website.
 */
export function linkifyClaim(key: string, value: string): TextPart[] {
  const trimmed = value.trim();
  const url = domainUrl(trimmed);
  if (url && (/^www\./i.test(trimmed) || WEBSITE_KEY_REGEX.test(key))) {
    return [{ text: value, url }];
  }

  return value
    .split(HTTPS_URL_PART_REGEX)
    .filter((part) => part.length > 0)
    .flatMap((part) => {
      if (!isUrl(part)) return [{ text: part }];
      const punctuation = part.match(TRAILING_PUNCTUATION_REGEX)?.[0] ?? '';
      const link = part.slice(0, part.length - punctuation.length);
      if (!isUrl(link)) return [{ text: part }];
      return punctuation ? [{ text: link, url: link }, { text: punctuation }] : [{ text: link, url: link }];
    });
}
