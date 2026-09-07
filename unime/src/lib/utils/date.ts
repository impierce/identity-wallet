import type { Locale } from '@bindings/profile_settings/Locale';

/** A calendar date without a time component, e.g. `2024-07-22`. */
const ISO_8601_DATE_REGEX = /^(\d{4}-\d{2}-\d{2})$/;

/**
 * A date with a time component, e.g. `2024-07-22T09:31:45Z`.
 * Seconds, fractional seconds and the timezone designator are optional.
 */
const ISO_8601_DATE_TIME_REGEX = /^(\d{4}-\d{2}-\d{2})T(\d{2}:\d{2}(?::\d{2}(?:\.\d+)?)?)(?:Z|[+-]\d{2}:?\d{2})?$/;

/** Time components that represent the very start of a day, e.g. `00:00`, `00:00:00` or `00:00:00.000`. */
const MIDNIGHT_REGEX = /^00:00(?::00(?:\.0+)?)?$/;

export interface Iso8601Timestamp {
  /** The calendar date as written in the credential, e.g. `2024-07-22`. */
  date: string;
  /**
   * Whether a meaningful time of day is part of the timestamp. Timestamps at midnight are treated as "date only",
   * since issuers commonly use them to express a plain date.
   */
  hasTime: boolean;
}

/**
 * Parses a timestamp compliant with [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) (as used by the `date` and
 * `date-time` formats of JSON Schema). Returns `null` for any value that is not such a timestamp.
 */
export function parseIso8601(value: unknown): Iso8601Timestamp | null {
  if (typeof value !== 'string') {
    return null;
  }

  const trimmed = value.trim();
  const match = ISO_8601_DATE_REGEX.exec(trimmed) ?? ISO_8601_DATE_TIME_REGEX.exec(trimmed);

  if (!match || Number.isNaN(new Date(trimmed).getTime())) {
    return null;
  }

  const [, date, time] = match;

  // `Date` silently rolls over days that do not exist (e.g. `1985-02-30` becomes `1985-03-02`), so the calendar date
  // is only accepted when it survives a round trip.
  if (!new Date(`${date}T00:00:00Z`).toISOString().startsWith(date)) {
    return null;
  }

  return { date, hasTime: time !== undefined && !MIDNIGHT_REGEX.test(time) };
}

/**
 * Formats an ISO 8601 timestamp for the given locale, omitting the time of day for "date only" timestamps.
 * Values that are not ISO 8601 timestamps are returned unchanged.
 *
 * Dates without a time of day are formatted in UTC, so that the date the issuer wrote into the credential is the date
 * the holder gets to see, independent of the timezone the device happens to be in.
 *
 * @param timeZone Overrides the timezone of timestamps that carry a time of day. Only intended for tests, since the
 * timezone in CI is UTC.
 */
export function formatIso8601(value: string, locale: Locale, timeZone?: string): string {
  const timestamp = parseIso8601(value);

  if (!timestamp) {
    return value;
  }

  if (!timestamp.hasTime) {
    return new Intl.DateTimeFormat(locale, { dateStyle: 'long', timeZone: 'UTC' }).format(new Date(timestamp.date));
  }

  return new Intl.DateTimeFormat(locale, { dateStyle: 'long', timeStyle: 'short', timeZone }).format(new Date(value));
}
