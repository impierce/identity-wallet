import { readFileSync } from 'node:fs';

import { toMatchImageSnapshot } from 'jest-image-snapshot';

import { render } from '@testing-library/svelte';

import Button from './Button.svelte';

// declare module 'vitest' {
//   interface Assertion<T> {
//     toMatchImageSnapshot(): T;
//   }
// }

expect.extend({ toMatchImageSnapshot });

describe('Button', () => {
  test('matches image snapshot', () => {
    const { container } = render(Button);
    expect(container.innerHTML).toMatchImageSnapshot();
    // expect(container.innerHTML).toMatchSnapshot();

    // expect(readFileSync('./src/lib/components/atoms/stubs/impierce_white.png')).toMatchImageSnapshot();
  });
});
