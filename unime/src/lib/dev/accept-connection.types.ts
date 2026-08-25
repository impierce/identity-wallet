// TEMPORARY.Delete this  once
// `LinkedVerifiableCredentialData` carries a credential and `ValidationResult` carries a `url`.
// CC-REMOVE!
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { CurrentUserPrompt } from '@bindings/user_prompt/CurrentUserPrompt';
import type { ValidationResult } from '@bindings/user_prompt/ValidationResult';

/**
 * A `ValidationResult` with the `url` the certification cards render the issuer domain from.
 * The Rust struct does not carry it yet, so it is declared here rather than generated.
 */
export interface IssuerDomainValidation extends ValidationResult {
  url: string;
}

/**
 * What `LinkedVerifiableCredentialData` is expected to become. The generated type is still the
 * old `{ name, logo_uri, issuance_date }` shape, so the certification pages run against this.
 */
export interface Certification {
  credential: DisplayCredential;
  issuer_domain_validations: IssuerDomainValidation[];
}

/** The generated `accept-connection` variant, pulled out of the `CurrentUserPrompt` union. */
type BackendPrompt = Extract<CurrentUserPrompt, { type: 'accept-connection' }>;

/**
 * The prompt as the pages consume it: generated for every field the backend already ships, with
 * `linked_verifiable_presentations` still overridden. Drop the override and the `Omit`, and this
 * collapses to `BackendPrompt`.
 */
export interface AcceptConnectionPrompt extends Omit<BackendPrompt, 'linked_verifiable_presentations'> {
  linked_verifiable_presentations?: Certification[];
}
