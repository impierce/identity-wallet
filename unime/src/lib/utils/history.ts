import type { EventType } from '@bindings/history/EventType';
import type { HistoryEvent } from '@bindings/history/HistoryEvent';

/**
 * Which way data moved during an interaction.
 *
 * `ConnectionAdded` only establishes the connection — the backend always pushes it with an empty
 * `credentials` array — so it has no direction.
 */
export type InteractionDirection = 'incoming' | 'outgoing' | 'none';

/**
 * The direction of a single history event.
 *
 * The declared return type keeps this exhaustive: if `EventType` ever gains a variant, this stops
 * compiling ("function lacks ending return statement") rather than silently classifying the new
 * variant as `none`.
 */
export function interactionDirection(eventType: EventType): InteractionDirection {
  switch (eventType) {
    case 'CredentialsAdded':
      return 'incoming';
    case 'CredentialsShared':
      return 'outgoing';
    case 'ConnectionAdded':
      return 'none';
  }
}

export interface InteractionCounts {
  /** Every interaction, `ConnectionAdded` included. */
  total: number;
  /** Interactions in which we sent credentials to the other party. */
  shared: number;
  /** Interactions in which we received credentials from the other party. */
  received: number;
}

/**
 * Counts interactions per direction, for the summary tiles on the connection request prompt.
 *
 * These are counts of *events*, not of credentials. A single exchange can carry several credentials
 * (the backend pushes one event with the whole set), so counting credentials could make `shared` and
 * `received` exceed `total`. Counting events preserves `shared + received <= total`, with the
 * difference being the `ConnectionAdded` events.
 */
export function countInteractions(interactions: HistoryEvent[]): InteractionCounts {
  let shared = 0;
  let received = 0;

  for (const interaction of interactions) {
    switch (interactionDirection(interaction.event_type)) {
      case 'outgoing':
        shared += 1;
        break;
      case 'incoming':
        received += 1;
        break;
      case 'none':
        break;
    }
  }

  return { total: interactions.length, shared, received };
}
