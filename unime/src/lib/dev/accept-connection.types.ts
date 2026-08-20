// TEMPORARY. Delete once identity-wallet/bindings is regenerated with the new
// AcceptConnection variant.
// CC-REMOVE!
import type { HistoryEvent } from '@bindings/history/HistoryEvent';
import type { LinkedVerifiableCredentialData } from '@bindings/user_prompt/LinkedVerifiableCredentialData';
import type { ValidationResult } from '@bindings/user_prompt/ValidationResult';

export interface Member {
  logo_uri: string | null;
  name: string;
  description: string | null;
  domain: string;
}

export interface EcosystemProfile {
  logo_uri: string | null;
  name: string;
  description: string | null;
  ecosystem_leader: Member;
  member_count: number;
  members: Member[];
}

export interface ConnectionData {
  first_interacted_at: string;
  last_interacted_at: string;
  interactions: HistoryEvent[];
}

export interface AcceptConnectionPrompt {
  type: 'accept-connection';
  client_name: string;
  logo_uri?: string;
  redirect_uri: string;
  connection_data: ConnectionData | null;
  domain_validation: ValidationResult;
  linked_verifiable_presentations: LinkedVerifiableCredentialData[];
  ecosystems: EcosystemProfile[];
}
