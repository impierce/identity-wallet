import '@testing-library/jest-dom';

import { vi } from 'vitest';
import renderWithRouter from './utils/svelte-navigator/renderWithRouter';
import { fireEvent, screen } from '@testing-library/svelte';
import Welcome from 'src/routes/Welcome.svelte';
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';

beforeEach(() => {
  // init __TAURI_IPC__
  mockIPC((cmd, args) => {});
});

afterEach(() => {
  clearMocks();
});

test('shows welcome label and user prompt label', () => {
  renderWithRouter(Welcome, {}, { withRoute: true });

  const heading = screen.getByTestId('label-welcome');
  expect(heading).toBeInTheDocument();

  const promptUsername = screen.getByTestId('label-prompt-username');
  expect(promptUsername).toBeInTheDocument();
});

test('triggers correct event when button is clicked', async () => {
  const spy = vi.spyOn(window, '__TAURI_IPC__');

  renderWithRouter(Welcome, {}, { withRoute: true });
  const button = screen.getByRole('button');
  expect(button).toBeInTheDocument();

  const input = screen.getByTestId('input-username');
  expect(input).toBeInTheDocument();
  input.setAttribute('value', 'Ferris');

  await fireEvent.click(button);

  expect(spy).toHaveBeenCalledTimes(1);
  expect(spy).toHaveBeenCalledWith({
    action: { type: '[DID] Create new', payload: 'Ferris' },
    callback: expect.anything(),
    cmd: 'handle_action',
    error: expect.anything()
  });
});

test('input field has focus when rendered', async () => {
  renderWithRouter(Welcome, {}, { withRoute: true });

  const input = screen.getByTestId('input-username');

  // TODO: click the heading after render still focuses the input? why?
  const heading = screen.getByTestId('label-welcome');
  await fireEvent.click(heading);

  expect(input).toHaveFocus();
});
