// TEMPORARY. remove once `identity-wallet/bindings` has been regenerated.
// CC-REMOVE!
import type { HistoryEvent } from '@bindings/history/HistoryEvent';
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

// `issuer_linked_domains` drops its `#[ts(skip)]` on the Rust side; `url-impl` is
// already enabled, so `Vec<Url>` exports as `Array<string>`.
export interface Certification {
  name: string | null;
  logo_uri: string | null;
  issuance_date: string;
  // The issuer's name is read from `.name` here; `.status` drives the domain row's icon.
  issuer_domain_validation: ValidationResult;
  issuer_linked_domains: string[];
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
