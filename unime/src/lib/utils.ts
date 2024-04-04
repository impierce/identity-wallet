import { convertFileSrc } from '@tauri-apps/api/core';
import { appDataDir, join } from '@tauri-apps/api/path';
import { BaseDirectory, exists } from '@tauri-apps/plugin-fs';
import { debug, info, warn } from '@tauri-apps/plugin-log';

/**
 * Get an image asset URL from the UniMe backend.
 *
 * @param id The identifier of the asset (e.g. the credential_id)
 * @param tmp Specify whether to look in the `tmp` folder (e.g. during a offer), default: `false`
 * @returns A local URL to the asset if present (else `null`)
 */
export const getImageAsset = async (id: string, tmp: boolean = false): Promise<string | null> => {
  const appDataDirPath = await appDataDir();

  const extensions = ['svg', 'png'];

  if (tmp) {
    for (let extension of extensions) {
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

  for (let extension of extensions) {
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
