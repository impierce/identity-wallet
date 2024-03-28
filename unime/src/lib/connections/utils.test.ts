import type { Connection } from '@bindings/Connection';

import { groupConnectionsAlphabetically } from './utils';

const connection: Connection = {
  id: '0',
  url: '',
  name: '',
  verified: false,
  first_interacted: '',
  last_interacted: '',
};

describe('connections', () => {
  test('should be grouped alphabetically by name', () => {
    const given: Connection[] = [
      { ...connection, id: '0', url: 'https://auth2.example.org', name: 'acme Corporation' },
      { ...connection, id: '1', url: 'https://api.ngdil.com', name: 'NGDIL Demo' },
      { ...connection, id: '2', url: 'https://auth.example.org', name: 'ACME Corp.' },
      { ...connection, id: '3', url: 'https://ecorp.com', name: 'ECorp' },
    ];

    const expected = new Map<string, Array<Connection>>([
      ['A', [given.at(2)!!, given.at(0)!!]],
      ['E', [given.at(3)!!]],
      ['N', [given.at(1)!!]],
    ]);

    expect(groupConnectionsAlphabetically(given)).toEqual(expected);
  });
});
