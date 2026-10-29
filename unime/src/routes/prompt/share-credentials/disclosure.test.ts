import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { CredentialOption } from '@bindings/user_prompt/CredentialOption';
import type { RequestedCredential } from '@bindings/user_prompt/RequestedCredential';

import { claimRows, claimText, defaultDisclosure, humanize, satisfiesRequest } from './disclosure';

const pid: RequestedCredential = {
  query_id: 'pid',
  multiple: false,
  types: ['pid'],
  options: [],
  last_shared: [],
  claims: [['given_name'], ['birth_date'], ['nationality'], ['address']],
  claim_sets: [
    [0, 1, 3],
    [0, 1],
    [0, 1, 2],
  ],
};

const option = (values: unknown[], selective_disclosure = true): CredentialOption => ({
  id: 'credential',
  state: 'valid',
  selective_disclosure,
  claims: values.map((value) => ({ value, concealable: true })),
});

const credential = {
  id: 'credential',
  display_claims: [{ path: ['given_name'], key: 'First name', value: 'Anna' }],
  data: { credentialSubject: { id: 'did:example:anna', name: 'Anna', address: { city: 'Utrecht' } } },
} as unknown as DisplayCredential;

describe('claimRows', () => {
  test('marks the claims of every claim set the credential satisfies as required and the rest as optional', () => {
    const rows = claimRows(pid, option(['Anna', '1990-03-12', 'NL', null]), credential);

    expect(rows.map(({ index, label, required }) => ({ index, label, required }))).toEqual([
      { index: 0, label: 'First name', required: true },
      { index: 1, label: 'Birth date', required: true },
      { index: 2, label: 'Nationality', required: false },
    ]);
  });

  test('marks claims that cannot be left out as required', () => {
    const credentialOption = option(['Anna', '1990-03-12', 'NL', null]);
    credentialOption.claims[2].concealable = false;

    expect(claimRows(pid, credentialOption, credential).every((row) => row.required)).toBe(true);
  });

  test('shows the whole credential as required without selective disclosure', () => {
    const rows = claimRows(pid, option([], false), { ...credential, display_claims: [] });

    expect(rows).toEqual([
      { label: 'Name', value: 'Anna', required: true },
      { label: 'City', value: 'Utrecht', required: true },
    ]);
  });

  test('makes raw claim names readable', () => {
    const rows = claimRows(pid, option(['Anna', '1990-03-12', 'NL', null]), {
      ...credential,
      display_claims: [{ path: ['given_name'], key: 'given_name', value: 'Anna' }],
    } as unknown as DisplayCredential);

    expect(rows[0].label).toBe('Given name');
  });
});

describe('defaultDisclosure', () => {
  test('picks the smallest claim set the credential satisfies', () => {
    expect(defaultDisclosure(pid, option(['Anna', '1990-03-12', 'NL', 'Utrecht']))).toEqual([0, 1]);
  });

  test('picks all claims when there are no claim sets', () => {
    expect(defaultDisclosure({ ...pid, claim_sets: [] }, option(['Anna', '1990-03-12', 'NL', 'Utrecht']))).toEqual([
      0, 1, 2, 3,
    ]);
  });
});

describe('satisfiesRequest', () => {
  test('needs every claim of at least one claim set', () => {
    const credentialOption = option(['Anna', '1990-03-12', 'NL', null]);

    expect(satisfiesRequest(pid, credentialOption, [0, 1, 2])).toBe(true);
    expect(satisfiesRequest(pid, credentialOption, [0, 2])).toBe(false);
    expect(satisfiesRequest(pid, credentialOption, [0, 3])).toBe(false);
  });
});

describe('claimText', () => {
  test('joins arrays and objects into one line', () => {
    expect(claimText(['NL', 'DE'], 'en-US')).toBe('NL, DE');
    expect(claimText({ street: 'Oudegracht 1', city: 'Utrecht' }, 'en-US')).toBe('Oudegracht 1, Utrecht');
    expect(claimText(null, 'en-US')).toBe('');
  });
});

describe('humanize', () => {
  test('turns claim names into words', () => {
    expect(humanize('given_name')).toBe('Given name');
    expect(humanize('dateOfBirth')).toBe('Date of birth');
  });
});
