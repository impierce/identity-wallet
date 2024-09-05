import { Sha256 } from '@aws-crypto/sha256-js';
import type { Locale } from '@bindings/profile_settings/Locale';
import { convertFileSrc } from '@tauri-apps/api/core';
import { appDataDir, join } from '@tauri-apps/api/path';
import { exists } from '@tauri-apps/plugin-fs';
import { debug, info, warn } from '@tauri-apps/plugin-log';

/**
 * Get an image asset URL from the UniMe backend.
 *
 * @param id The identifier of the asset (e.g. the credential_id)
 * @param tmp Specify whether to look in the `tmp` folder (e.g. during a offer), default: `false`
 * @returns A local URL to the asset if present (else `null`)
 */
export const getImageAsset = async (id: string, tmp = false): Promise<string | null> => {
  const appDataDirPath = await appDataDir();

  const extensions = ['svg', 'png'];

  if (tmp) {
    for (const extension of extensions) {
      const tmpFilePath = await join(appDataDirPath, `assets/tmp/${id}.${extension}`);
      if (await exists(tmpFilePath)) {
        const assetUrl = convertFileSrc(tmpFilePath);
        info(`assetUrl: ${assetUrl}`);
        return assetUrl;
      }
    }
    warn(`No tmp file found for id: ${id}`);
    return null;
  }

  for (const extension of extensions) {
    const filePath = await join(appDataDirPath, `assets/${id}.${extension}`);
    debug(`filePath: ${filePath}`);
    if (await exists(filePath)) {
      const assetUrl = convertFileSrc(filePath);
      debug(`assetUrl: ${assetUrl}`);
      return assetUrl;
    }
  }

  warn(`No file found for id: ${id}`);
  return null;
};

export const hash = (data: string): string => {
  const hash = new Sha256();
  hash.update(data);
  const result = hash.digestSync();
  return Array.from(result)
    .map((i) => i.toString(16).padStart(2, '0'))
    .join('');
};

export const calculateInitials = (name: string): string => {
  const parts = name.split(' ').filter((n) => n.length > 0);
  if (parts.length === 1) {
    // Take first two letters, if only one name
    return parts.at(0)!.slice(0, 2).toUpperCase();
  } else {
    const first = parts?.at(0)?.charAt(0) ?? '?';
    const last = parts?.at(1)?.charAt(0) ?? '?';
    return `${first}${last}`.toUpperCase();
  }
};

export function formatDate(isoDate: string, locale: Locale, test = false) {
  return new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
    // Timezone in CI is UTC, which would fail tests.
    timeZone: test ? 'Europe/Amsterdam' : undefined,
  }).format(new Date(isoDate));
}

export function formatDateTime(isoDate: string, locale: Locale, test = false) {
  return new Intl.DateTimeFormat(locale, {
    dateStyle: 'medium',
    timeStyle: 'medium',
    // Timezone in CI is UTC, which would fail tests.
    timeZone: test ? 'Europe/Amsterdam' : undefined,
  }).format(new Date(isoDate));
}

export function formatRelativeDateTime(isoDate: string, locale: Locale) {
  // 1 min, 1 hour, 1 day, 1 week, 1 month, 1 year.today
  const thresholds = [60, 3600, 86400, 86400 * 7, 86400 * 30, 86400 * 365, Infinity];
  const units: Intl.RelativeTimeFormatUnit[] = ['second', 'minute', 'hour', 'day', 'week', 'month', 'year'];

  const diffInSeconds = (Date.now() - new Date(isoDate).getTime()) / 1000;

  // Determine the threshold to use.
  const index = thresholds.findIndex((threshold) => threshold > diffInSeconds);
  const divisor = index ? thresholds[index - 1] : 1;

  const relativeDateTime = new Intl.RelativeTimeFormat(locale, {
    numeric: 'auto',
  }).format(-1 * Math.floor(diffInSeconds / divisor), units[index]);

  // Capitalize the first character.
  return relativeDateTime.charAt(0).toUpperCase() + relativeDateTime.slice(1);
}
