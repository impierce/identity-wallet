import { randomFillSync } from 'crypto';

import { beforeAll, expect, test, vi } from 'vitest';

import { invoke } from '@tauri-apps/api/core';
import { mockIPC } from '@tauri-apps/api/mocks';

// jsdom doesn't come with a WebCrypto implementation
beforeAll(() => {
  //   Object.defineProperty(window, 'crypto', {
  //     value: {
  //       // @ts-ignore
  //       getRandomValues: (buffer) => {
  //         return randomFillSync(buffer);
  //       },
  //     },
  //   });

  Object.defineProperty(globalThis, 'window', {
    value: {
      __TAURI_IPC__: undefined,
      // @ts-ignore
      // getRandomValues: (buffer) => {
      //   return randomFillSync(buffer);
      // },
    },
  });
});

test('invoke', async () => {
  mockIPC((cmd, args) => {
    if (cmd === 'add') {
      return (args.a as number) + (args.b as number);
    }
  });

  // we can use the spying tools provided by vitest to track the mocked function
  const spy = vi.spyOn(window, '__TAURI_IPC__');

  expect(invoke('add', { a: 12, b: 15 })).resolves.toBe(27);
  expect(spy).toHaveBeenCalled();
});
