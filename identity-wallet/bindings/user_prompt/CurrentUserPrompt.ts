// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ValidationResult } from "./ValidationResult";

export type CurrentUserPrompt = { "type": "redirect", target: string, } | { "type": "password-required" } | { "type": "accept-connection", client_name: string, logo_uri?: string, redirect_uri: string, previously_connected: boolean, domain_validation: ValidationResult, thuiswinkel_validation: ValidationResult, } | { "type": "credential-offer", issuer_name: string, logo_uri?: string, credential_configurations: Record<string, any>, } | { "type": "share-credentials", client_name: string, logo_uri?: string, options: Array<string>, };
