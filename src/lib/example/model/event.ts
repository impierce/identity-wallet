export interface Event {
  id: number;
  timestamp: string;
  type: EventType;
  data: any;
}

export enum EventType {
  INITIAL_CONNECTION = 'initial_connection',
  CREDENTIAL_OFFER = 'credential_offer',
  LOGIN = 'login'
}
