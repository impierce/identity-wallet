import '@testing-library/jest-dom';

import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import renderWithRouter from './utils/svelte-navigator/renderWithRouter';
import { vi } from 'vitest';
import App from 'src/App.svelte';
import { tick } from 'svelte';

beforeEach(() => {
  // init __TAURI_IPC__
  mockIPC((cmd, args) => {});
});

afterEach(() => {
  clearMocks();
});

test('fetches app state on mount', async () => {
    const spy = vi.spyOn(window, '__TAURI_IPC__');

    await tick();

    renderWithRouter(App, {}, { withRoute: true });

    await tick();

    const spy_after = vi.spyOn(window, '__TAURI_IPC__');

    expect(spy).toHaveBeenCalledTimes(1);
    expect(spy).toHaveBeenCalledWith({
        cmd: 'tauri',
        callback: expect.anything(),
        error: expect.anything(),
        message: {
            cmd: 'listen',
            event: 'state-changed',
            handler: expect.anything(),
            windowLabel: null
        },
        "__tauriModule": expect.anything()
    });
    
    expect(spy_after).toHaveBeenCalledTimes(1);
    expect(spy_after).toHaveBeenCalledWith({
        action: { type: '[App] Get state' },
        callback: expect.anything(),
        cmd: 'execute_command',
        error: expect.anything()
    });
});
