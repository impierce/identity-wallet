import { checkPermissions, type PermissionState } from '@tauri-apps/plugin-barcode-scanner';
import { info, warn } from '@tauri-apps/plugin-log';

// TODO: remove (unused)
export const checkScanPrerequisites = async (): Promise<boolean> => {
  return await checkPermissions()
    .then((permission: PermissionState) => {
      info(`app has permissions to access the camera: ${res}`);
      if (permission === 'prompt') {
        // TODO: ask user to open settings (https://github.com/impierce/identity-wallet/issues/23)
        warn('TODO: ask the user');
        return false;
      }
      if (permission === 'granted') {
        return true;
      } else {
        warn('app does not have permissions to access the camera');
        return false;
      }
    })
    .catch((err) => {
      warn(`error checking permissions: ${err}`);
      return false;
    });
};
