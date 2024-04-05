import type { Connection } from '@bindings/Connection';

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
