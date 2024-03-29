import type { Connection } from '@bindings/Connection';

// TODO: replace with native "Object.groupBy" once available
// https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/groupBy
function groupBy(list: Array<any>, keyGetter) {
  const map = new Map();
  list.forEach((item) => {
    const key = keyGetter(item);
    const collection = map.get(key);
    if (!collection) {
      map.set(key, [item]);
    } else {
      collection.push(item);
    }
  });
  return map;
}

export const groupConnectionsAlphabetically = (connections: Connection[]): Map<string, Connection[]> => {
  const sorted = connections.sort((a, b) => a.name.localeCompare(b.name));
  return groupBy(sorted, (connection: Connection) => connection.name[0].toUpperCase());
};
