import { vi } from 'vitest';

import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';
import { fireEvent, render, screen } from '@testing-library/svelte';

import Welcome from '$src/routes/welcome/+page@.svelte';

beforeEach(() => {
  // init __TAURI_IPC__
  mockIPC((cmd, args) => {});
});

afterEach(() => {
  clearMocks();
});

test('shows welcome label and user prompt label', () => {
  render(Welcome, {});

  const heading = screen.getByTestId('label-welcome');
  expect(heading).toBeInTheDocument();

  const promptUsername = screen.getByTestId('label-prompt-username');
  expect(promptUsername).toBeInTheDocument();
});

test('triggers correct event when button is clicked', async () => {
  const spy = vi.spyOn(window, '__TAURI_IPC__');

  render(Welcome, {});
  const button = screen.getByRole('button');
  expect(button).toBeInTheDocument();

  const input = screen.getByTestId('input-username');
  expect(input).toBeInTheDocument();
  input.setAttribute('value', 'Ferris');

  await fireEvent.click(button);

  expect(spy).toHaveBeenCalledTimes(2); // TODO: skip first call
  expect(spy).toHaveBeenCalledWith({
    action: { type: '[DID] Create new', payload: { name: 'Ferris' } },
    callback: expect.anything(),
    cmd: 'handle_action',
    error: expect.anything()
  });
});

test('input field has focus when rendered', async () => {
  render(Welcome, {});

  const input = screen.getByTestId('input-username');

  // TODO: click the heading after render still focuses the input? why?
  const heading = screen.getByTestId('label-welcome');
  await fireEvent.click(heading);

  expect(input).toHaveFocus();
});
