import { appDataDir, join } from '@tauri-apps/api/path';
import { convertFileSrc } from '@tauri-apps/api/primitives';

import type { Connection } from './types';

export const groupConnectionsAlphabetically = (connections: Connection[]): Map<string, Connection[]> => {
  const map = new Map<string, Connection[]>();
  connections.forEach((connection) => {
    console.log('connection.url', connection.url);
    let firstLetter: string = connection.url[0].toUpperCase();
    let displayName = connection.displayName;
    try {
      const hostname = new URL(connection.url).hostname;
      firstLetter = hostname[0].toUpperCase();
      displayName = hostname;
    } catch (e) {
      console.warn(`could not create new URL from: ${connection.url}`);
    }
    const connections = map.get(firstLetter) || [];
    connections.push({ ...connection, displayName });
    map.set(firstLetter, connections);
  });
  return map;
};

/**
 * Get an image asset URL from the UniMe backend.
 *
 * @param id The identifier of the asset (e.g. the credential_id)
 * @param tmp Specify whether to look in the `tmp` folder (e.g. during a offer), default: `false`
 * @returns
 */
export const getImageAsset = async (id: string, tmp: boolean = false): Promise<string> => {
  const appDataDirPath = await appDataDir();

  // find by id (any file extension)
  const extension = 'png';

  if (tmp) {
    const tmpFilePath = await join(appDataDirPath, `assets/tmp/${id}.${extension}`);
    const assetUrl = convertFileSrc(tmpFilePath);
    console.log({ assetUrl });
    return assetUrl;
  }

  const filePath = await join(appDataDirPath, `assets/${id}.${extension}`);
  const assetUrl = convertFileSrc(filePath);
  console.log({ assetUrl });
  return assetUrl;
};
