import type { Connection } from './types';
import { groupConnectionsAlphabetically } from './utils';

describe('connections', () => {
  test('should be grouped alphabetically by url', () => {
    const given: Connection[] = [
      {
        id: '0',
        url: 'https://auth.example.org',
        client_name: 'ACME Corp.',
        verified: false,
        first_interacted: '',
        last_interacted: '',
      },
      {
        id: '1',
        url: 'https://api.ngdil.com',
        client_name: 'NGDIL Demo',
        verified: false,
        first_interacted: '',
        last_interacted: '',
      },
      {
        id: '2',
        url: 'https://auth2.example.org',
        client_name: 'ACME Corporation',
        verified: false,
        first_interacted: '',
        last_interacted: '',
      },
      {
        id: '3',
        url: 'https://ecorp.com',
        client_name: 'ECorp',
        verified: false,
        first_interacted: '',
        last_interacted: '',
      },
    ];

    expect(groupConnectionsAlphabetically(given)).toEqual(
      new Map<string, Array<any>>([
        ['A', [given.at(0), given.at(2)]],
        ['E', [given.at(3)]],
        ['N', [given.at(1)]],
      ]),
    );
  });
});
