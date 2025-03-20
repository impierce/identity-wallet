import { Sha256 } from '@aws-crypto/sha256-js';
import type { Locale } from '@bindings/profile_settings/Locale';
import { convertFileSrc } from '@tauri-apps/api/core';
import { appDataDir, join } from '@tauri-apps/api/path';
import { BiometryType } from '@tauri-apps/plugin-biometric';
import { platform } from '@tauri-apps/plugin-os';
import { exists } from '@tauri-apps/plugin-fs';
import { debug, info, warn } from '@tauri-apps/plugin-log';
import LL from '$i18n/i18n-svelte';
import { get } from 'svelte/store';

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
  const date = new Date(isoDate);
  const now = new Date();

  const diffInSeconds = (date.getTime() - now.getTime()) / 1000;

  // Thresholds in seconds: 1 min, 1 hour, 1 day, 1 week, 1 month, 1 year.
  const thresholds = [60, 3600, 86400, 86400 * 7, 86400 * 30, 86400 * 365, Infinity];

  const units: Intl.RelativeTimeFormatUnit[] = ['second', 'minute', 'hour', 'day', 'week', 'month', 'year'];

  // Determine the threshold to use.
  const index = thresholds.findIndex((threshold) => Math.abs(diffInSeconds) < threshold);
  const divisor = index ? thresholds[index - 1] : 1;

  const relativeFormatter = new Intl.RelativeTimeFormat(locale, {
    numeric: 'auto',
  });

  // Use Math.round for more accurate relative time.
  const relativeDateTime = relativeFormatter.format(Math.round(diffInSeconds / divisor), units[index]);

  // Capitalize the first character.
  return relativeDateTime.charAt(0).toUpperCase() + relativeDateTime.slice(1);
}

/**
 * Determines the name of the biometrics type based on the platform.
 */
export function localizedBiometricsTypeString(type: BiometryType): string {
  let biometryTypeString = get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.GENERIC(); // default
  // On iOS, we distinguish between Face ID and Touch ID.
  if (platform() === 'ios') {
    const localizedType = (() => {
      switch (type) {
        case BiometryType.TouchID:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.IOS.TOUCH_ID();
        case BiometryType.FaceID:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.IOS.FACE_ID();
        default:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.GENERIC();
      }
    })();
    biometryTypeString = localizedType;
    // On Android, we distinguish between fingerprint and face unlock.
  } else if (platform() === 'android') {
    const localizedType = (() => {
      switch (type) {
        case BiometryType.TouchID:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.ANDROID.TOUCH_ID();
        case BiometryType.FaceID:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.ANDROID.FACE_ID();
        default:
          return get(LL).SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.GENERIC();
      }
    })();
    biometryTypeString = localizedType;
  }
  return biometryTypeString;

  // {#if biometricsStatus.biometryType === BiometryType.FaceID}
  //         {#if platform() === 'ios'}
  //           {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({ type: $LL.SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.IOS.FACE_ID() })}
  //         {:else if platform() === 'android'}
  //           {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({
  //             type: $LL.SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.ANDROID.FACE_ID(),
  //           })}
  //         {:else}
  //           <!-- TODO: handle unsupported platform? -->
  //         {/if}
  //       {:else if biometricsStatus.biometryType === BiometryType.TouchID}
  //         {#if platform() === 'ios'}
  //           {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({ type: $LL.SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.IOS.TOUCH_ID() })}
  //         {:else if platform() === 'android'}
  //           {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({
  //             type: $LL.SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.ANDROID.TOUCH_ID(),
  //           })}
  //         {:else}
  //           <!-- TODO: handle unsupported platform? -->
  //         {/if}
  //       {:else}
  //         <!-- TODO: handle unsupported platform -->
  //         {$LL.SETTINGS.APP.SECURITY.SWITCH_LABEL({ type: $LL.SETTINGS.APP.SECURITY.BIOMETRIC_TYPE.GENERIC() })}
  //       {/if}
}
