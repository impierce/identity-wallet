import type { Connection } from '@bindings/connections/Connection';
import type { DisplayCredential } from '@bindings/credentials/DisplayCredential';
import type { HistoryCredential } from '@bindings/history/HistoryCredential';

/**
 * The name of a credential's issuer as it is now. Credentials keep the name the issuer had at issuance, while the
 * connection is renamed on every later interaction, so the connection's name wins when the credential has one.
 */
export function currentIssuerName(
  credential: Pick<DisplayCredential, 'issuer_name' | 'connection_id' | 'data'>,
  connections: Connection[],
): string {
  const connection = credential.connection_id
    ? connections.find((connection) => connection.id === credential.connection_id)
    : undefined;
  return connection?.name || (credential.issuer_name ?? credential.data?.issuer?.name ?? credential.data?.issuer);
}

/**
 * The name of a connection as it is now. History events keep the name the connection had at the time of the event.
 */
export function currentConnectionName(connectionId: string, fallbackName: string, connections: Connection[]): string {
  return connections.find((connection) => connection.id === connectionId)?.name || fallbackName;
}

/**
 * The issuer name of a credential in a history event as it is now, if the credential is still in the wallet.
 */
export function currentHistoryIssuerName(
  historyCredential: HistoryCredential,
  credentials: DisplayCredential[],
  connections: Connection[],
): string {
  const credential = credentials.find((credential) => credential.id === historyCredential.id);
  return credential ? currentIssuerName(credential, connections) : historyCredential.issuer_name;
}
