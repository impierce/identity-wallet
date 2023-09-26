import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';

import '@testing-library/jest-dom';

import { render, screen } from '@testing-library/svelte';

import Profile from '../src/routes/(app)/me/+page.svelte';

beforeEach(() => {
  // init __TAURI_IPC__
  mockIPC((cmd, args) => {});
});

afterEach(() => {
  clearMocks();
});

it('shows primary did', () => {
  render(Profile, {});
  const did = screen.getByTestId('primary-did');
  expect(did).toBeInTheDocument();
});
