import '@testing-library/jest-dom';

import { tick } from 'svelte';

import { fireEvent, render, screen } from '@testing-library/svelte';

import PinInputFixture from './PinInputFixture.test.svelte';

const boxes = () => screen.getAllByRole<HTMLInputElement>('textbox');
const values = () => boxes().map((box) => box.value);

// Text entered into a box at once, as by autofill, a keyboard suggestion or a test driver.
async function enter(box: HTMLInputElement, text: string) {
  box.focus();
  await fireEvent.input(box, { target: { value: box.value + text } });
  await tick();
}

async function paste(box: HTMLInputElement, text: string) {
  box.focus();
  await fireEvent.paste(box, { clipboardData: { getData: () => text } });
  await tick();
}

describe('fillPinInput', () => {
  test('spreads a code entered into the first box over all boxes', async () => {
    render(PinInputFixture);

    await enter(boxes()[0], '123456');

    expect(values()).toEqual(['1', '2', '3', '4', '5', '6']);
    expect(screen.getByRole('button', { name: 'Accept' })).toBeEnabled();
    expect(boxes()[5]).toHaveFocus();
  });

  test('fills the boxes from the focused one on and focuses the box after them', async () => {
    render(PinInputFixture);
    await enter(boxes()[0], '1');

    await paste(boxes()[1], '23');

    expect(values()).toEqual(['1', '2', '3', '', '', '']);
    expect(boxes()[3]).toHaveFocus();
    expect(screen.getByRole('button', { name: 'Accept' })).toBeDisabled();
  });

  test('keeps only digits for a numeric code', async () => {
    render(PinInputFixture);

    await paste(boxes()[0], '123 456\n');

    expect(values()).toEqual(['1', '2', '3', '4', '5', '6']);
    expect(screen.getByRole('button', { name: 'Accept' })).toBeEnabled();
  });

  test('leaves no rejected text in a box', async () => {
    render(PinInputFixture);
    await enter(boxes()[0], '1');

    await enter(boxes()[1], 'abc');

    expect(values()).toEqual(['1', '', '', '', '', '']);
    expect(screen.getByRole('button', { name: 'Accept' })).toBeDisabled();
  });

  test('drops what does not fit', async () => {
    render(PinInputFixture, { maxLength: 4 });

    await paste(boxes()[0], '123456');

    expect(values()).toEqual(['1', '2', '3', '4']);
  });

  test('takes letters for a text code', async () => {
    render(PinInputFixture, { type: 'text', maxLength: 4 });

    await paste(boxes()[0], 'ab12');

    expect(values()).toEqual(['a', 'b', '1', '2']);
    expect(screen.getByRole('button', { name: 'Accept' })).toBeEnabled();
  });
});
