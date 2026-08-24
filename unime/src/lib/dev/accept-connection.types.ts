// TEMPORARY. remove once `identity-wallet/bindings` has been regenerated.
// CC-REMOVE!
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { HistoryEvent } from '@bindings/history/HistoryEvent';
import type { ValidationStatus } from '@bindings/user_prompt/ValidationStatus';

export interface ValidationResult {
  status: ValidationStatus;
  url: string;
  name?: string;
  logo_uri?: string;
  issuance_date?: string;
  message?: string;
}

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

// Mirrors `LinkedVerifiableCredentialData`.
export interface Certification {
  credential: DisplayCredential;
  issuer_domain_validations: ValidationResult[];
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
  linked_verifiable_presentations: Certification[];
  ecosystems: EcosystemProfile[];
}
