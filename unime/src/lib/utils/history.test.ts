import type { EventType } from '@bindings/history/EventType';
import type { HistoryCredential } from '@bindings/history/HistoryCredential';
import type { HistoryEvent } from '@bindings/history/HistoryEvent';

import { countInteractions, interactionDirection } from './history';

const credential = (title: string): HistoryCredential => ({
  title,
  issuer_name: 'BestDex',
  id: title.toLowerCase().replace(/\s+/g, '-'),
});

const event = (event_type: EventType, credentials: HistoryCredential[] = []): HistoryEvent => ({
  connection_id: 'did:web:bestdex.com',
  connection_name: 'BestDex',
  event_type,
  date: '2023-07-28T09:30:00Z',
  credentials,
});

describe('interactionDirection', () => {
  test('classifies credentials we received as incoming', () => {
    expect(interactionDirection('CredentialsAdded')).toBe('incoming');
  });

  test('classifies credentials we shared as outgoing', () => {
    expect(interactionDirection('CredentialsShared')).toBe('outgoing');
  });

  test('gives establishing the connection no direction, since no data moved', () => {
    expect(interactionDirection('ConnectionAdded')).toBe('none');
  });
});

describe('countInteractions', () => {
  test('counts no interactions', () => {
    expect(countInteractions([])).toEqual({ total: 0, shared: 0, received: 0 });
  });

  test('counts each direction, with the connection event only in the total', () => {
    const counts = countInteractions([
      event('ConnectionAdded'),
      event('CredentialsAdded', [credential('Diploma')]),
      event('CredentialsShared', [credential('Diploma')]),
      event('CredentialsShared', [credential('Diploma')]),
    ]);

    expect(counts).toEqual({ total: 4, shared: 2, received: 1 });
  });

  test('counts an exchange carrying several credentials as one interaction', () => {
    const counts = countInteractions([
      event('CredentialsShared', [credential('Diploma'), credential('Passport'), credential('Drivers License')]),
    ]);

    // Not 3: the tiles count exchanges, so `shared` can never exceed `total`.
    expect(counts).toEqual({ total: 1, shared: 1, received: 0 });
  });

  test('keeps shared and received within the total', () => {
    const interactions = [
      event('ConnectionAdded'),
      event('CredentialsShared', [credential('Diploma')]),
      event('CredentialsAdded', [credential('Passport')]),
    ];

    const { total, shared, received } = countInteractions(interactions);

    expect(shared + received).toBeLessThanOrEqual(total);
  });
});
