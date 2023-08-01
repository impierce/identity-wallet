import { goto } from '$app/navigation';
import { info, warn } from '@tauri-apps/plugin-log';
import { checkPermissions, openAppSettings } from '@tauri-apps/plugin-barcode-scanner';
import { dispatch } from '$lib/dispatcher';

export const checkScanPrerequisites = async (): Promise<boolean> => {
  await checkPermissions()
    .then((res) => {
      info(`app has permissions to access the camera: ${res}`);
      // TODO: ask user to open settings (https://github.com/impierce/identity-wallet/issues/23)
      if (res === 'granted' || res === 'default') {
        return true;
      } else {
        warn('app does not have permissions to access the camera');
        // return false;
      }
    })
    .catch((err) => {
      warn(`error checking permissions: ${err}`);
      // return false;
    });
  return false;
};
