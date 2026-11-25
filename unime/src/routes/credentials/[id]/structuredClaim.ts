import type { NestedDisplayClaim } from '@bindings/credentials/NestedDisplayClaim';

/**
 * Arranges a claim whose value is an object or an array into the rows the credential details show: Objects, Lists
 * and Tables fold their content away, Lists of single values become chips, and Tables become a grid. It follows the
 * UniMe preview of UniTrust.
 *
 * The issuer's claims metadata (OpenID4VCI 1.0, Appendix B.2) decides where it says something: the labels, the order
 * of fields and columns, the `format` of a value, and whether an array is a Table. Where it says nothing, the rows
 * follow the data: the keys are the labels, as in UniTrust's templates, and an array of flat objects is a Table.
 */

/** What a row is called: a label or key of the credential, or the position of an item in a List. */
export type ClaimTitle = { kind: 'key'; text: string } | { kind: 'item'; position: number };

/** How many rows a fold row hides: the fields of an Object or item, or the entries of a List or Table. */
export interface ClaimCount {
  unit: 'fields' | 'entries';
  count: number;
}

export type ClaimNode =
  | { kind: 'value'; key: string; title: ClaimTitle; value: string; format?: string }
  | { kind: 'chips'; key: string; title: ClaimTitle; values: string[]; format?: string }
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

export interface ClaimColumn {
  title: string;
  format?: string;
}

export interface ClaimTable {
  kind: 'table';
  key: string;
  title: ClaimTitle;
  count: ClaimCount;
  depth: number;
  columns: ClaimColumn[];
  /** The cells of each row in column order, `undefined` where a cell is empty. */
  rows: { key: string; cells: (string | undefined)[] }[];
}

/** What the issuer says about a claim and the claims inside it. */
export interface ClaimHints {
  format?: string;
  /** The descriptions of the claims inside it, with paths from the claim itself, in the issuer's order. */
  nested: NestedDisplayClaim[];
}

type Primitive = string | number | boolean;

/** Where a node sits inside the claim: keys of Objects and positions of items. */
type Position = (string | number)[];

interface Field {
  key: string;
  title: string;
  format?: string;
}

/** Whether a claim value needs the structured view rather than a single text field. */
export function isStructuredClaim(value: unknown): value is Record<string, unknown> | unknown[] {
  return typeof value === 'object' && value !== null;
}

/**
 * The node for the claim `key` with `value`, or `undefined` when nothing in it has a value. Empty values are left
 * out, and so are Objects, Lists, Tables and items that end up without any.
 */
export function buildClaimTree(key: string, value: unknown, hints: ClaimHints = { nested: [] }): ClaimNode | undefined {
  return new TreeBuilder(hints.nested).node({ kind: 'key', text: key }, `/${key}`, [], value, 0, hints.format);
}

class TreeBuilder {
  constructor(private readonly nested: NestedDisplayClaim[]) {}

  node(
    title: ClaimTitle,
    key: string,
    position: Position,
    value: unknown,
    depth: number,
    format: string | undefined,
  ): ClaimNode | undefined {
    if (Array.isArray(value)) return this.collection(title, key, position, value, depth, format);

    if (isRecord(value)) {
      const children = this.fieldsOf(position, Object.keys(value)).flatMap((field) => {
        const child = this.node(
          { kind: 'key', text: field.title },
          `${key}/${field.key}`,
          [...position, field.key],
          value[field.key],
          depth + 1,
          field.format,
        );
        return child ? [child] : [];
      });
      if (children.length === 0) return undefined;

      return { kind: 'group', key, title, count: { unit: 'fields', count: children.length }, depth, children };
    }

    const text = toText(value);
    if (text === undefined) return undefined;
    return { kind: 'value', key, title, value: text, ...(format ? { format } : {}) };
  }

  /** A List or Table: chips, a grid, or a group of numbered items. */
  private collection(
    title: ClaimTitle,
    key: string,
    position: Position,
    items: unknown[],
    depth: number,
    format: string | undefined,
  ): ClaimNode | undefined {
    if (items.every(isPrimitiveOrEmpty)) {
      const values = items.map(toText).filter((text): text is string => text !== undefined);
      if (values.length === 0) return undefined;
      const itemFormat = this.describedAt([...position, 0])?.format;
      return { kind: 'chips', key, title, values, ...(itemFormat ? { format: itemFormat } : {}) };
    }

    if (this.showsAsTable(position, items, format)) {
      const table = this.table(title, key, position, items as Record<string, unknown>[], depth);
      if (table) return table;
    }

    const groups = items.flatMap((item, index): ClaimNode[] => {
      const itemNode = this.node(
        { kind: 'item', position: index + 1 },
        `${key}/${index}`,
        [...position, index],
        item,
        depth + 1,
        this.describedAt([...position, index])?.format,
      );
      return itemNode ? [itemNode] : [];
    });
    if (groups.length === 0) return undefined;

    return { kind: 'group', key, title, count: { unit: 'entries', count: groups.length }, depth, children: groups };
  }

  /**
   * Whether an array shows as a Table. Its items have to be objects of single values. The issuer makes it a Table
   * with `format: "table"`, and a List of Objects by describing its items without that format. Without any word
   * from the issuer, every such array is a Table.
   */
  private showsAsTable(position: Position, items: unknown[], format: string | undefined): boolean {
    const flat = (item: unknown) => isRecord(item) && Object.values(item).every(isPrimitiveOrEmpty);
    if (items.length === 0 || !items.every(flat)) return false;
    if (format === 'table') return true;
    return format === undefined && !this.nested.some((hint) => goesThrough(hint.path, [...position, 0]));
  }

  private table(
    title: ClaimTitle,
    key: string,
    position: Position,
    items: Record<string, unknown>[],
    depth: number,
  ): ClaimTable | undefined {
    // An issuer leaves empty cells out of a row, so the columns are the keys of all rows.
    const keys = [...new Set(items.flatMap((item) => Object.keys(item)))];
    const fields = this.fieldsOf([...position, 0], keys);
    const rows = items
      .map((item, index) => ({
        key: `${key}/${index}`,
        cells: fields.map((field) => toText(item[field.key])),
      }))
      .filter((row) => row.cells.some((cell) => cell !== undefined));
    if (fields.length === 0 || rows.length === 0) return undefined;

    const columns = fields.map(({ title, format }): ClaimColumn => (format ? { title, format } : { title }));
    return { kind: 'table', key, title, count: { unit: 'entries', count: rows.length }, depth, columns, rows };
  }

  /**
   * The fields of the Object at `position` with the keys `keys`: first those the issuer describes, in its order and
   * with its labels, then the others in the order of the data.
   */
  private fieldsOf(position: Position, keys: string[]): Field[] {
    const described: Field[] = [];
    for (const hint of this.nested) {
      const key = hint.path.at(-1);
      if (
        hint.path.length === position.length + 1 &&
        typeof key === 'string' &&
        keys.includes(key) &&
        !described.some((field) => field.key === key) &&
        selects(hint.path.slice(0, -1), position)
      ) {
        described.push({ key, title: hint.label?.trim() || key, ...(hint.format ? { format: hint.format } : {}) });
      }
    }
    const rest = keys
      .filter((key) => !described.some((field) => field.key === key))
      .map((key) => ({ key, title: key }));
    return [...described, ...rest];
  }

  /** The issuer's description of the claim at `position`. */
  private describedAt(position: Position): NestedDisplayClaim | undefined {
    return this.nested.find((hint) => selects(hint.path, position));
  }
}

/** Whether a path selects the claim at `position`. `null` stands for every item of an array (Appendix C.1). */
function selects(path: NestedDisplayClaim['path'], position: Position): boolean {
  return path.length === position.length && path.every((element, index) => sameElement(element, position[index]));
}

/** Whether a path goes through the claim at `position` to a claim inside it. */
function goesThrough(path: NestedDisplayClaim['path'], position: Position): boolean {
  return path.length > position.length && position.every((element, index) => sameElement(path[index], element));
}

function sameElement(element: string | number | null, at: string | number): boolean {
  return element === null ? typeof at === 'number' : element === at;
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
