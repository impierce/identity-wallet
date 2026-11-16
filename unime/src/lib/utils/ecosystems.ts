import type { Connection } from '@bindings/connections/Connection';
import type { EcosystemsCheck } from '@bindings/connections/EcosystemsCheck';
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { EcosystemProfile } from '@bindings/user_prompt/EcosystemProfile';

export interface IssuerEcosystems {
  ecosystems: EcosystemProfile[];
  // Absent until the issuer's ecosystems were resolved with the outcome recorded.
  check?: EcosystemsCheck;
}

// Endorsements belong to the issuer, so every credential from it shares those of its connection. Credentials without
// one, such as self-issued ones, have none.
export function issuerEcosystems(credential: DisplayCredential, connections: Connection[]): IssuerEcosystems {
  const connection = credential.connection_id
    ? connections.find((item) => item.id === credential.connection_id)
    : undefined;
  return { ecosystems: connection?.ecosystems ?? [], check: connection?.ecosystems_check };
}
