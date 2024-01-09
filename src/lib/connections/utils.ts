import type { Connection } from './types';

export const groupConnectionsAlphabetically = (connections: Connection[]): Map<string, Connection[]> => {
  const map = new Map<string, Connection[]>();
  connections.forEach((connection) => {
    console.log('connection.url', connection.url);
    // let firstLetter: string = connection.url[0].toUpperCase();
    let firstLetter: string = connection.client_name[0].toUpperCase();
    let displayName = connection.client_name;
    try {
      const hostname = new URL(connection.url).hostname;
      // firstLetter = hostname[0].toUpperCase();
      // displayName = hostname;
    } catch (e) {
      console.warn(`could not create new URL from: ${connection.url}`);
    }
    const connections = map.get(firstLetter) || [];
    connections.push({ ...connection, client_name: displayName });
    map.set(firstLetter, connections);
  });
  return map;
};
