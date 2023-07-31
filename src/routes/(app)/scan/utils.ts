import { goto } from '$app/navigation';
import { info, warn } from '@tauri-apps/plugin-log';
import { checkPermissions, openAppSettings } from '@tauri-apps/plugin-barcode-scanner';
import { dispatch } from '$lib/dispatcher';

export const checkScanPrerequisites = async () => {
  await checkPermissions()
    .then((res) => {
      info(`app has permissions to access the camera: ${res}`);
      // TODO: ask user to open settings (https://github.com/impierce/identity-wallet/issues/23)
      if (res === 'granted' || res === 'default') {
        goto('/scan');
      } else {
        warn('app does not have permissions to access the camera');
        openAppSettings();
      }
    })
    .catch((err) => {
      warn(`error checking permissions: ${err}`);
      const TEST_SIOP_REQUEST_URL =
        'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.1.234%3A4243%2Fsiop%2Frequest-uri';
      // const TEST_SIOP_REQUEST_URL = 'siopv2://idtoken?client_id=did%3Akey%3Az6MkpuwK1TrrssGe7siCiJU2K5CbSu3mDLU4Y3z45wAepg7J&request_uri=http%3A%2F%2F192.168.178.42%3A4243%2Fsiop%2Frequest-uri';
      dispatch({
        type: '[Authenticate] Read request',
        payload: { request_url: TEST_SIOP_REQUEST_URL }
      });
    });
  console.log('checkScanPrerequisites');
  // goto('/scan');
};
