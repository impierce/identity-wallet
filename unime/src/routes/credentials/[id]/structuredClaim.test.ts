import { describe, expect, it } from 'vitest';

import { buildClaimTree, isStructuredClaim } from './structuredClaim';

describe('isStructuredClaim', () => {
  it('is true for objects and arrays only', () => {
    expect(isStructuredClaim({ Street: 'Main' })).toBe(true);
    expect(isStructuredClaim(['en'])).toBe(true);
    expect(isStructuredClaim('text')).toBe(false);
    expect(isStructuredClaim(42)).toBe(false);
    expect(isStructuredClaim(null)).toBe(false);
  });
});

describe('buildClaimTree', () => {
  it('shows an Object as a group of its fields, open at the top level', () => {
    const tree = buildClaimTree('Address', { Street: 'Main Street 1', City: 'Utrecht', Note: '' });

    expect(tree).toEqual({
      kind: 'group',
      key: '/Address',
      title: { kind: 'key', text: 'Address' },
      count: { unit: 'fields', count: 2 },
      depth: 0,
      children: [
        { kind: 'value', key: '/Address/Street', title: { kind: 'key', text: 'Street' }, value: 'Main Street 1' },
        { kind: 'value', key: '/Address/City', title: { kind: 'key', text: 'City' }, value: 'Utrecht' },
      ],
    });
  });

  it('shows a List of single values as chips, leaving out empty ones', () => {
    expect(buildClaimTree('Languages', ['English', '', 'Dutch', 3, true])).toEqual({
      kind: 'chips',
      key: '/Languages',
      title: { kind: 'key', text: 'Languages' },
      values: ['English', 'Dutch', '3', 'true'],
    });
  });

  it('shows an array of flat objects as a Table, with the keys of all rows as columns', () => {
    const tree = buildClaimTree('Courses', [
      { Name: 'Rust', Credits: '5' },
      { Name: 'Svelte', Grade: 'A' },
      { Name: '', Credits: '' },
    ]);

    expect(tree).toEqual({
      kind: 'table',
      key: '/Courses',
      title: { kind: 'key', text: 'Courses' },
      count: { unit: 'entries', count: 2 },
      depth: 0,
      columns: ['Name', 'Credits', 'Grade'],
      rows: [
        { key: '/Courses/0', cells: ['Rust', '5', undefined] },
        { key: '/Courses/1', cells: ['Svelte', undefined, 'A'] },
      ],
    });
  });

  it('shows a List of nested objects as numbered items, one level deeper', () => {
    const tree = buildClaimTree('Projects', [
      { Name: 'UniMe', Team: { Lead: 'Ferris' } },
      { Name: '', Team: { Lead: '' } },
      { Name: 'UniTrust', Team: { Lead: 'Crab' } },
    ]);

    expect(tree?.kind).toBe('group');
    if (tree?.kind !== 'group') return;
    expect(tree.count).toEqual({ unit: 'entries', count: 2 });
    expect(tree.children.map((child) => child.title)).toEqual([
      { kind: 'item', position: 1 },
      { kind: 'item', position: 3 },
    ]);

    const [first] = tree.children;
    expect(first).toMatchObject({ kind: 'group', depth: 1, count: { unit: 'fields', count: 2 } });
    if (first.kind !== 'group') return;
    expect(first.children[1]).toMatchObject({ kind: 'group', key: '/Projects/0/Team', depth: 2 });
  });

  it('nests Objects, Lists and Tables in an Object', () => {
    const tree = buildClaimTree('Profile', {
      Skills: ['Rust'],
      Jobs: [{ Role: 'Engineer', Year: 2024 }],
    });

    expect(tree?.kind).toBe('group');
    if (tree?.kind !== 'group') return;
    expect(tree.children.map((child) => child.kind)).toEqual(['chips', 'table']);
    expect(tree.children[1]).toMatchObject({ depth: 1, rows: [{ cells: ['Engineer', '2024'] }] });
  });

  it('shows the single values of a mixed List as numbered values', () => {
    const tree = buildClaimTree('Mixed', ['first', { Name: 'second' }, ['third']]);

    expect(tree?.kind).toBe('group');
    if (tree?.kind !== 'group') return;
    expect(tree.children.map((child) => [child.kind, child.title])).toEqual([
      ['value', { kind: 'item', position: 1 }],
      ['group', { kind: 'item', position: 2 }],
      ['chips', { kind: 'item', position: 3 }],
    ]);
  });

  it('leaves out an Object, List or Table without any value', () => {
    expect(buildClaimTree('Empty', {})).toBeUndefined();
    expect(buildClaimTree('Empty', { Street: '', City: null })).toBeUndefined();
    expect(buildClaimTree('Empty', [])).toBeUndefined();
    expect(buildClaimTree('Empty', ['', null])).toBeUndefined();
    expect(buildClaimTree('Empty', [{ Name: '' }, { Name: ' ' }])).toBeUndefined();
    expect(buildClaimTree('Empty', [{ Team: { Lead: '' } }])).toBeUndefined();
  });
});
