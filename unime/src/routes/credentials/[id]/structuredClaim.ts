/**
 * Arranges a claim whose value is an object or an array into the rows the credential details show: Objects, Lists
 * and Tables fold their content away, Lists of single values become chips, and arrays of flat objects with the same
 * keys become Tables. It follows the UniMe preview of UniTrust, whose templates write the field titles as the keys.
 */

/** What a row is called: a key of the credential, or the position of an item in a List. */
export type ClaimTitle = { kind: 'key'; text: string } | { kind: 'item'; position: number };

/** How many rows a fold row hides: the fields of an Object or item, or the entries of a List or Table. */
export interface ClaimCount {
  unit: 'fields' | 'entries';
  count: number;
}

export type ClaimNode =
  | { kind: 'value'; key: string; title: ClaimTitle; value: string }
  | { kind: 'chips'; key: string; title: ClaimTitle; values: string[] }
  | ClaimGroup
  | ClaimTable;

/** An Object, a List of Objects or one of its items: a row that folds its children away. */
export interface ClaimGroup {
  kind: 'group';
  key: string;
  title: ClaimTitle;
  count: ClaimCount;
  /** How many fold rows sit above it; only the top level opens expanded. */
  depth: number;
  children: ClaimNode[];
}

export interface ClaimTable {
  kind: 'table';
  key: string;
  title: ClaimTitle;
  count: ClaimCount;
  depth: number;
  columns: string[];
  /** The cells of each row in column order, `undefined` where a cell is empty. */
  rows: { key: string; cells: (string | undefined)[] }[];
}

type Primitive = string | number | boolean;

/** Whether a claim value needs the structured view rather than a single text field. */
export function isStructuredClaim(value: unknown): value is Record<string, unknown> | unknown[] {
  return typeof value === 'object' && value !== null;
}

/**
 * The node for the claim `key` with `value`, or `undefined` when nothing in it has a value. Empty values are left
 * out, and so are Objects, Lists, Tables and items that end up without any.
 */
export function buildClaimTree(key: string, value: unknown): ClaimNode | undefined {
  return toNode({ kind: 'key', text: key }, `/${key}`, value, 0);
}

function toNode(title: ClaimTitle, key: string, value: unknown, depth: number): ClaimNode | undefined {
  if (Array.isArray(value)) return toCollection(title, key, value, depth);

  if (isRecord(value)) {
    const children = Object.entries(value).flatMap(([childKey, childValue]) => {
      const child = toNode({ kind: 'key', text: childKey }, `${key}/${childKey}`, childValue, depth + 1);
      return child ? [child] : [];
    });
    if (children.length === 0) return undefined;

    return { kind: 'group', key, title, count: { unit: 'fields', count: children.length }, depth, children };
  }

  const text = toText(value);
  return text === undefined ? undefined : { kind: 'value', key, title, value: text };
}

/** A List or Table: chips, a grid, or a group of numbered items. */
function toCollection(title: ClaimTitle, key: string, items: unknown[], depth: number): ClaimNode | undefined {
  if (items.every(isPrimitiveOrEmpty)) {
    const values = items.map(toText).filter((text): text is string => text !== undefined);
    return values.length > 0 ? { kind: 'chips', key, title, values } : undefined;
  }

  const columns = tableColumns(items);
  if (columns) return toTable(title, key, items as Record<string, unknown>[], columns, depth);

  const groups = items.flatMap((item, position): ClaimNode[] => {
    const itemNode = toNode({ kind: 'item', position: position + 1 }, `${key}/${position}`, item, depth + 1);
    return itemNode ? [itemNode] : [];
  });
  if (groups.length === 0) return undefined;

  return { kind: 'group', key, title, count: { unit: 'entries', count: groups.length }, depth, children: groups };
}

/**
 * The columns of a Table: the keys of its rows in the order they first appear, when every item is an object of
 * single values. A row may lack a key, since an issuer leaves empty cells out. `undefined` for anything else, which
 * then shows as a List of items.
 */
function tableColumns(items: unknown[]): string[] | undefined {
  const flat = (item: unknown): item is Record<string, unknown> =>
    isRecord(item) && Object.values(item).every(isPrimitiveOrEmpty);
  if (items.length === 0 || !items.every(flat)) return undefined;

  const columns = [...new Set(items.flatMap((item) => Object.keys(item as Record<string, unknown>)))];
  return columns.length > 0 ? columns : undefined;
}

function toTable(
  title: ClaimTitle,
  key: string,
  items: Record<string, unknown>[],
  columns: string[],
  depth: number,
): ClaimTable | undefined {
  const rows = items
    .map((item, position) => ({
      key: `${key}/${position}`,
      cells: columns.map((column) => toText(item[column])),
    }))
    .filter((row) => row.cells.some((cell) => cell !== undefined));
  if (rows.length === 0) return undefined;

  return { kind: 'table', key, title, count: { unit: 'entries', count: rows.length }, depth, columns, rows };
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function isPrimitiveOrEmpty(value: unknown): value is Primitive | null | undefined {
  return value === null || value === undefined || ['string', 'number', 'boolean'].includes(typeof value);
}

/** The text of a single value, `undefined` when it is empty or not a single value. */
function toText(value: unknown): string | undefined {
  if (!isPrimitiveOrEmpty(value) || value === null || value === undefined) return undefined;

  const text = String(value);
  return text.trim().length > 0 ? text : undefined;
}
