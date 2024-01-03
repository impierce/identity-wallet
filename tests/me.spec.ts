import { webkit } from 'playwright';

import { expect, test } from '@playwright/test';
import { mockIPC } from '@tauri-apps/api/mocks';
import { getCurrent } from '@tauri-apps/api/window';

import { testState } from './state';

test.beforeAll(async () => {
  // console.log('beforeAll');
  // setStore({
  //   // state.set({
  //   active_profile: {
  //     name: 'Wright',
  //     picture: null,
  //     theme: 'dark',
  //     primary_did: '',
  //   },
  //   locale: 'en',
  //   credentials: [],
  //   current_user_prompt: null,
  //   dev_mode_enabled: false,
  //   debug_messages: [],
  //   user_journey: null,
  //   connections: [],
  //   user_data_query: [],
  // });
  // console.log({ state });
  //   Object.defineProperty(window, 'crypto', {
  //     value: {
  //       // @ts-ignore
  //       //   getRandomValues: (buffer) => {
  //       //     return randomFillSync(buffer);
  //       //   },
  //     },
  //   });
});

test('me', async ({ page }) => {
  //   const window = await page.evaluate(() => window);

  //   mockIPC((cmd, args) => {
  //     // simulated rust command called "add" that just adds two numbers
  //     if (cmd === 'handle_action') {
  //       console.log('handle_action', args);
  //     }
  //   });

  await page.goto('http://localhost:5173/me');
  // await page.waitForLoadState('domcontentloaded');

  // state.set({
  //   active_profile: {
  //     name: 'Wright',
  //     picture: null,
  //     theme: 'dark',
  //     primary_did: '',
  //   },
  //   locale: 'en',
  //   credentials: [],
  //   current_user_prompt: null,
  //   dev_mode_enabled: false,
  //   debug_messages: [],
  //   user_journey: null,
  //   connections: [],
  //   user_data_query: [],
  // });
  await expect(page.getByTestId('welcome-message')).toBeVisible();

  await page.evaluate(() => {
    // window.state.set(testState);
    window.state.set({
      active_profile: {
        name: 'Playwright',
        picture: '&#x1F527',
        theme: 'system',
        primary_did: '',
      },
      locale: 'de',
      credentials: [],
      current_user_prompt: null,
      dev_mode_enabled: false,
      debug_messages: [],
      user_journey: null,
      connections: [],
      user_data_query: [],
    });
  });

  await expect(page.getByTestId('welcome-message')).toHaveText(/Playwright/);
  // await expect(page).toHaveScreenshot();
});

test.skip('save screenshot', async () => {
  const browser = await webkit.launch();
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto('http://localhost:5173/me');
  await expect(page.getByTestId('welcome-message')).toBeVisible();
  await page.screenshot({ path: 'screenshot.png' });
  await browser.close();
});

test.describe('credential list', () => {
  test('displays one credential', async ({ page }) => {
    await page.goto('http://localhost:5173/me');

    await expect(page.getByTestId('welcome-message')).toBeVisible();

    // Object.defineProperty(globalThis, 'window', {
    //   // value: undefined,
    //   value: { __TAURI_IPC__: {} }
    // });

    await page.evaluate(() => {
      console.log(window);

      // mockIPC((cmd, args) => {
      //   if (cmd === 'plugin:path|resolve_directory') {
      //     return '/path/to/assets';
      //   }
      //   switch (cmd) {
      //     case 'plugin:path|resolve_directory':
      //       return '/path/to/assets';
      //     // case 'plugin:event|listen':
      //     //   return null;
      //     // case 'plugin:assets|get_asset':
      //     //   return 'data:image/png;base64,iVBOR';
      //     default:
      //       break;
      //   }
      // });
      // Object.defineProperty(window, 'crypto', {
      //   value: {
      //     // @ts-ignore
      //     // getRandomValues: (buffer) => {
      //     //   return randomFillSync(buffer);
      //     // },
      //   },
      // });

      // Object.defineProperty(globalThis, 'window', {
      //   value: {
      //     __TAURI_IPC__: undefined,
      //     // @ts-ignore
      //     // getRandomValues: (buffer) => {
      //     //   return randomFillSync(buffer);
      //     // },
      //   },
      // });
    });

    // mockIPC((cmd, args) => {
    //   if (cmd === 'plugin:path|resolve_directory') {
    //     return '/path/to/assets';
    //   }
    //   switch (cmd) {
    //     case 'plugin:path|resolve_directory':
    //       return '/path/to/assets';
    //     // case 'plugin:event|listen':
    //     //   return null;
    //     // case 'plugin:assets|get_asset':
    //     //   return 'data:image/png;base64,iVBOR';
    //     default:
    //       break;
    //   }
    // });

    // TODO: refactor
    await page.evaluate(() => {
      // window.state.set(testState);
      window.state.set({
        active_profile: {
          name: 'Playwright',
          picture: '&#x1F527',
          theme: 'system',
          primary_did: '',
        },
        locale: 'de',
        credentials: [
          {
            id: '0',
            issuer_name: null,
            format: {
              format: 'jwt_vc_json',
            },
            data: {
              '@context': ['https://www.w3.org/2018/credentials/v1', 'https://www.w3.org/2018/credentials/examples/v1'],
              type: ['VerifiableCredential', 'DriverLicenseCredential'],
              issuer: 'http://192.168.1.127:9090/',
              issuanceDate: '2022-08-15T09:30:00Z',
              expirationDate: '2027-08-15T23:59:59Z',
              credentialSubject: {
                id: 'did:key:z6Mkg1XXGUqfkhAKU1kVd1Pmw6UEj1vxiLj1xc91MBz5owNY',
                licenseClass: 'Class C',
                issuedBy: 'California',
                validity: 'Valid',
              },
            },
            metadata: {
              is_favorite: false,
              date_added: '2023-12-21T20:22:10.503376+00:00',
              date_issued: '"2022-08-15T09:30:00Z"',
              display: {
                icon: null,
                color: null,
                name: null,
              },
            },
          },
        ],
        current_user_prompt: null,
        dev_mode_enabled: false,
        debug_messages: [],
        user_journey: null,
        connections: [],
        user_data_query: [],
      });
    });
  });

  test.skip('displays multiple credentials', async ({ page }) => {
    await page.goto('http://localhost:5173/me');

    // TODO: refactor
    await page.evaluate(() => {
      // window.state.set(testState);
      window.state.set({
        active_profile: {
          name: 'Playwright',
          picture: '&#x1F527',
          theme: 'system',
          primary_did: '',
        },
        locale: 'de',
        credentials: [],
        current_user_prompt: null,
        dev_mode_enabled: false,
        debug_messages: [],
        user_journey: null,
        connections: [],
        user_data_query: [],
      });
    });
  });
});
