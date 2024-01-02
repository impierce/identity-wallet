import { webkit } from 'playwright';

import { expect, test } from '@playwright/test';

import { state } from '$src/stores';

// import { mockIPC } from '@tauri-apps/api/mocks';

test.beforeAll(async () => {
  console.log('beforeAll');
  state.set({
    active_profile: {
      name: 'Wright',
      picture: null,
      theme: 'dark',
      primary_did: '',
    },
    locale: 'en',
    credentials: [],
    current_user_prompt: null,
    dev_mode_enabled: false,
    debug_messages: [],
    user_journey: null,
    connections: [],
    user_data_query: [],
  });
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
  await expect(page.getByTestId('welcome-message')).toBeVisible();
  await expect(page.getByTestId('welcome-message')).toHaveText(/undefined/);
  // await expect(page).toHaveScreenshot();
});

test('save screenshot', async () => {
  const browser = await webkit.launch();
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto('http://localhost:5173/me');
  await expect(page.getByTestId('welcome-message')).toBeVisible();
  await page.screenshot({ path: 'screenshot.png' });
  await browser.close();
});
