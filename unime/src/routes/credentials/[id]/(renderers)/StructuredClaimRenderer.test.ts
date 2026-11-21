import '@testing-library/jest-dom';

import { setLocale } from '$i18n/i18n-svelte';
import { loadAllLocales } from '$i18n/i18n-util.sync';

import { fireEvent, render, screen } from '@testing-library/svelte';

import ClaimRenderer from './ClaimRenderer.svelte';

describe('StructuredClaimRenderer', () => {
  beforeAll(() => {
    loadAllLocales();
    setLocale('en');
    // jsdom has no Web Animations, which Svelte's transitions use.
    Element.prototype.animate ??= () =>
      ({ cancel: () => {}, finished: Promise.resolve(), onfinish: null }) as unknown as Animation;
  });

  beforeEach(() => {
    // The root layout provides the portal the sheet opens in.
    document.body.innerHTML = '<div id="portal"></div>';
  });

  test('opens the claim itself and folds the Objects inside it', async () => {
    render(ClaimRenderer, {
      key: 'Projects',
      value: [
        { Name: 'UniMe', Team: { Lead: 'Ferris' } },
        { Name: 'UniTrust', Team: { Lead: 'Crab' } },
      ],
    });

    expect(screen.getByRole('button', { name: /Projects/ })).toHaveAttribute('aria-expanded', 'true');
    const firstItem = screen.getByRole('button', { name: /Item 1/ });
    expect(firstItem).toHaveAttribute('aria-expanded', 'false');
    expect(firstItem).toHaveTextContent('2 fields');
    expect(screen.queryByText('UniMe')).not.toBeInTheDocument();

    await fireEvent.click(firstItem);

    expect(firstItem).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByText('UniMe')).toBeInTheDocument();
    expect(screen.queryByText('[object Object]')).not.toBeInTheDocument();
  });

  test('shows a List of single values as chips', () => {
    render(ClaimRenderer, { key: 'Languages', value: ['English', 'Dutch'] });

    expect(screen.getByText('Languages')).toBeInTheDocument();
    expect(screen.getByText('English')).toBeInTheDocument();
    expect(screen.getByText('Dutch')).toBeInTheDocument();
  });

  test('shows an array of flat objects as a Table with its entry count', () => {
    render(ClaimRenderer, {
      key: 'Courses',
      value: [
        { Course: 'Rust', Credits: '5' },
        { Course: 'Svelte', Credits: '3' },
      ],
    });

    expect(screen.getByRole('button', { name: /2 entries/ })).toHaveAttribute('aria-expanded', 'true');
    expect(screen.getByText('Credits')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Courses, Entry 2' })).toHaveTextContent('Svelte');
  });

  test('opens a Table row in a sheet with all of its fields', async () => {
    render(ClaimRenderer, {
      key: 'Courses',
      value: [
        { Course: 'Rust', Credits: '5' },
        { Course: 'Svelte', Credits: '' },
      ],
    });

    await fireEvent.click(screen.getByRole('button', { name: 'Courses, Entry 2' }));

    const sheet = await screen.findByRole('dialog');
    expect(sheet).toHaveTextContent('Entry 2');
    expect(sheet).toHaveTextContent('Svelte');
    expect(sheet).toHaveTextContent('—');
  });

  test('renders nothing for an Object without values', () => {
    const { container } = render(ClaimRenderer, { key: 'Address', value: { Street: '' } });

    // Only Svelte's anchors remain.
    expect(container.children).toHaveLength(0);
    expect(container.textContent?.trim()).toBe('');
  });
});
