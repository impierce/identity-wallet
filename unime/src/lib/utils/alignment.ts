/**
 * An `Alignment` (Open Badges 3.0 B.1.2) of an achievement or of a result: a reference to a skill, a competency or
 * an occupation that the achievement lines up with.
 *
 * All fields are optional here because only `targetName` and `targetUrl` are required by the specification, and
 * because the credential data an alignment is read from is not validated against a schema.
 */
export interface Alignment {
  targetName?: string;
  targetUrl?: string;
  targetDescription?: string;
  targetCode?: string;
  targetFramework?: string;
  targetType?: string | string[];
}

/** The kind of entry an official skill refers to within its framework. */
export type SkillKind = 'skill' | 'occupation';

/** An alignment that references an entry of a skills framework, with the fields that identify it resolved. */
export interface OfficialSkill {
  name: string;
  framework: string;
  /** The code of the entry within its framework, unless the framework identifies its entries by an opaque code. */
  code: string | null;
  kind: SkillKind | null;
  url: string | null;
  description: string | null;
}

/**
 * Frameworks are free to pick the codes of their entries. ESCO uses a readable notation for the levels of its
 * hierarchy (e.g. `S1.4.1`), but a UUID for the concepts themselves. A UUID tells the holder of the credential
 * nothing, so it is not shown alongside the skill.
 */
const OPAQUE_CODE_REGEX = /^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$/i;

/**
 * `Alignment TargetType` (OBv3 B.1.29) is an extensible enumeration whose members are bound to a framework, e.g.
 * `ceasn:Competency` or `ext:EscoSkill` (the `ext:` prefix is reserved for custom frameworks). Only the entry that
 * an alignment points at is of interest here, not the framework that prefixes it.
 */
function findSkillKind(targetType: Alignment['targetType']): SkillKind | null {
  const targetTypes = Array.isArray(targetType) ? targetType : [targetType];

  for (const type of targetTypes) {
    if (typeof type !== 'string') continue;

    const entry = type.split(':').pop()!.toLowerCase();

    if (entry.endsWith('occupation')) return 'occupation';
    if (entry.endsWith('skill') || entry.endsWith('competency')) return 'skill';
  }

  return null;
}

function trimmed(value: unknown): string | null {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : null;
}

/**
 * Resolves the framework entry an alignment references, or `null` if it does not reference one.
 *
 * An alignment is considered "official" if it names both the framework it belongs to and the code that identifies it
 * within that framework, e.g. a skill picked from ESCO. Alignments that are written by hand during template creation
 * only carry a name, a URL and a description, and cannot be traced back to a framework.
 */
export function findOfficialSkill(alignment: Alignment | null | undefined): OfficialSkill | null {
  if (!alignment) return null;

  const name = trimmed(alignment.targetName);
  const framework = trimmed(alignment.targetFramework);
  const code = trimmed(alignment.targetCode);

  if (!name || !framework || !code) return null;

  return {
    name,
    framework,
    code: OPAQUE_CODE_REGEX.test(code) ? null : code,
    kind: findSkillKind(alignment.targetType),
    url: trimmed(alignment.targetUrl),
    description: trimmed(alignment.targetDescription),
  };
}

/**
 * Reads the alignments of an achievement or of a result as a list.
 *
 * The specification defines both as an array, but a single alignment that is not wrapped in one is common enough in
 * credentials in the wild that it is accepted here instead of being dropped.
 */
export function toAlignments(value: unknown): Alignment[] {
  if (Array.isArray(value)) {
    return value.filter((alignment): alignment is Alignment => typeof alignment === 'object' && alignment !== null);
  }

  return typeof value === 'object' && value !== null ? [value as Alignment] : [];
}
