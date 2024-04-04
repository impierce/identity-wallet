import { randomFillSync } from 'node:crypto';

import '@testing-library/jest-dom';

// import imageUrl from 'vite-asset-url:../../../tests/res/student-rep_bg-white.png';

import { invoke } from '@tauri-apps/api/core';
import { mockConvertFileSrc, mockIPC } from '@tauri-apps/api/mocks';
import { render, screen } from '@testing-library/svelte';

import Image from './Image.svelte';

// TODO: mock the assets server that answers calls to:
// asset://localhost/logo-1.png

beforeAll(() => {
  Object.defineProperty(window, 'crypto', {
    value: {
      // @ts-ignore
      getRandomValues: (buffer) => {
        return randomFillSync(buffer);
      },
    },
  });
});

describe('Image', () => {
  // TODO: set up the test to read an actual image from disk
  test.skip('should display the image if it exists on disk', async () => {
    // window.__TAURI_INTERNALS__ = window.__TAURI_INTERNALS__ ?? {};

    mockConvertFileSrc('linux');

    const id = '65323136-6535-3737-6463-386531323361';

    mockIPC((cmd, args) => {
      switch (cmd) {
        // appDataDir()
        case 'plugin:path|resolve_directory':
          return Promise.resolve('_foobar');
        // join()
        case 'plugin:path|join':
          return Promise.resolve(
            `/Users/daniel/Library/Application Support/com.impierce.identity-wallet/assets/${id}.png`,
          );
        // exists()
        case 'plugin:fs|exists':
          return Promise.resolve(true);
        default:
          console.warn(`No mock provided for: ${cmd}`);
          break;
      }
    });

    // mockIPC((cmd, args) => {
    //   switch (cmd) {
    //     // appDataDir()
    //     case 'plugin:path|resolve_directory':
    //       return Promise.resolve('_foobar');
    //     // join()
    //     case 'plugin:path|join':
    //       return Promise.resolve(
    //         `/Users/daniel/Library/Application Support/com.impierce.identity-wallet/assets/${id}.png`,
    //       );
    //     // exists()
    //     case 'plugin:fs|exists':
    //       return Promise.resolve(true);
    //     default:
    //       console.warn(`No mock provided for: ${cmd}`);
    //       break;
    //   }

    // // appDataDir()
    // if (cmd === 'plugin:path|resolve_directory') {
    //   return Promise.resolve('_foobar');
    // }
    // // join()
    // if (cmd === 'plugin:path|join') {
    //   return Promise.resolve(
    //     `/Users/daniel/Library/Application Support/com.impierce.identity-wallet/assets/${id}.png`,
    //   );
    // }
    // // exists()
    // if (cmd === 'plugin:fs|exists') {
    //   return Promise.resolve(true);
    // }
    // });

    const { container } = render(Image, { id });
    // expect(container).toMatchSnapshot();
    // expect(container).toMatchFileSnapshot('./tests/basic.output.html');

    // screen.debug();

    expect(screen.getByTestId('image')).toBeInTheDocument();
  });

  // TODO: enable test, skipped due to bug in testing library, message: "window.__TAURI_INTERNALS__.invoke is not a function"
  test.skip('should display a given icon if it does not exist on disk', () => {
    // window.__TAURI_INTERNALS__ = window.__TAURI_INTERNALS__ ?? {};

    mockConvertFileSrc('linux');

    const id = '0';

    mockIPC((cmd, args) => {
      // appDataDir()
      // if (cmd === 'plugin:path|resolve_directory') {
      //   return Promise.resolve('_foobar');
      // }
      // join()
      // if (cmd === 'plugin:path|join') {
      //   return Promise.resolve(`/invalid/path/to/assets/${id}.png`);
      // }
      // exists();
      if (cmd === 'plugin:fs|exists') {
        return Promise.resolve(false);
      }
    });

    const { container } = render(Image, { id });
    // expect(container).toMatchSnapshot();

    // screen.debug();

    expect(screen.getByTestId('icon')).toBeInTheDocument();
  });
});
