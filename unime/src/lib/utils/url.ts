export const HTTPS_URL_PART_REGEX = /(https:\/\/\S+)/g;

export function isUrl(text: string): boolean {
  try {
    const url = new URL(text);
    return url.protocol === 'https:';
  } catch {
    return false;
  }
}
