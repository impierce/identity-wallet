import type { Connection } from '@bindings/connections/Connection';

// TODO: replace with native "Object.groupBy" once available
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/groupBy
function groupBy(connections: Connection[], getKey: (connection: Connection) => string): Map<string, Connection[]> {
  const map = new Map<string, Connection[]>();
  connections.forEach((connection) => {
    const key = getKey(connection);
    const connections = map.get(key);
    if (!connections) {
      map.set(key, [connection]);
    } else {
      connections.push(connection);
    }
  });
  return map;
}

export const groupConnectionsAlphabetically = (connections: Connection[]): Map<string, Connection[]> => {
  const sorted = connections.sort((a, b) => a.name.localeCompare(b.name));
  return groupBy(sorted, (connection: Connection) => connection.name[0].toUpperCase());
};

// Creates a link to the respective IOTA Explorer that searches for the given DID
export const buildIotaExplorerSearchLink = (did: string): string => {
  const explorerMapping: Record<string, string> = {
    'did:iota': 'mainnet',
    'did:iota:smr': 'shimmer',
    'did:iota:rms': 'testnet',
  };

  // Determine network (by removing the "IOTA-Tag")
  const parts = did.split(':');
  parts.pop();
  const network = parts.join(':');

  return `https://explorer.iota.org/${explorerMapping[network]}/search/${did}`;
};
