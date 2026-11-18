#!/usr/bin/env node
// Generates `docs/app-reference.md`: every screen of the UniMe app with its purpose, its controls, the screens it opens
// and the backend actions it dispatches.
//
// The reference is derived from three sources, so it follows the code instead of being maintained by hand:
// - the route tree in `unime/src/routes` (which screens exist),
// - the Svelte components of each screen (controls, navigation and dispatched actions),
// - the English base translations in `unime/src/i18n/en/index.ts` (the labels of the controls).
//
// The only hand-written input is `scripts/app-reference-screens.json`, which gives every screen a name and one or two
// sentences on its purpose, and sets the reading order. The script fails when a route is missing from that file or
// when the file names a route that no longer exists.
//
// Usage:
//   node scripts/generate-app-reference.mjs            write docs/app-reference.md
//   node scripts/generate-app-reference.mjs --check    fail if docs/app-reference.md is out of date (used by CI)
//
// The output is deterministic: it contains no dates, versions or commit hashes and does not depend on the platform.
import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const SRC = path.join(ROOT, 'unime', 'src');
const ROUTES_DIR = path.join(SRC, 'routes');
const TRANSLATIONS_FILE = path.join(SRC, 'i18n', 'en', 'index.ts');
const SCREENS_FILE = path.join(ROOT, 'scripts', 'app-reference-screens.json');
const OUTPUT_FILE = path.join(ROOT, 'docs', 'app-reference.md');

// Svelte and TypeScript are dependencies of the `unime` workspace, not of the repository root.
const requireFromUnime = createRequire(path.join(ROOT, 'unime', 'package.json'));
const { parse } = requireFromUnime('svelte/compiler');
const ts = requireFromUnime('typescript');

// Stands in for a `${...}` expression in a template literal such as `/credentials/${credential.id}`.
const DYNAMIC = '\u0000';
// Serialized action types follow the `[Feature] Action name` convention (see AGENTS.md).
const ACTION_TYPE = /^\[[^\]]+\] \S/;
// Keys of AST nodes that hold no source code of their own.
const SKIPPED_NODE_KEYS = new Set(['metadata', 'loc', 'name_loc', 'leadingComments', 'trailingComments', 'comments']);

const read = (file) => readFileSync(file, 'utf8').replace(/\r\n/g, '\n');
const repoPath = (file) => path.relative(ROOT, file).split(path.sep).join('/');
const compare = (a, b) => (a < b ? -1 : a > b ? 1 : 0);
const unique = (items) => [...new Set(items)];
const collapse = (text) => text.replace(/\s+/g, ' ').trim();
const isFile = (file) => existsSync(file) && statSync(file).isFile();
const within = (ranges, position) => ranges.some(([start, end]) => position >= start && position < end);

/** Problems with the input, reported together at the end. */
const problems = [];

// ---------------------------------------------------------------------------------------------------------------------
// Translations
// ---------------------------------------------------------------------------------------------------------------------

/** The English base translations as a map from key (`SCAN.CREDENTIAL_OFFER.ACCEPT`) to text. */
function loadTranslations() {
  const sourceFile = ts.createSourceFile(TRANSLATIONS_FILE, read(TRANSLATIONS_FILE), ts.ScriptTarget.Latest, true);
  const strings = new Map();
  const visit = (object, prefix) => {
    for (const property of object.properties) {
      if (!ts.isPropertyAssignment(property)) continue;
      const key = [...prefix, property.name.text];
      if (ts.isObjectLiteralExpression(property.initializer)) visit(property.initializer, key);
      else if (ts.isStringLiteralLike(property.initializer)) strings.set(key.join('.'), property.initializer.text);
    }
  };
  ts.forEachChild(sourceFile, (statement) => {
    if (!ts.isVariableStatement(statement)) return;
    for (const declaration of statement.declarationList.declarations) {
      let value = declaration.initializer;
      while (value && (ts.isSatisfiesExpression(value) || ts.isAsExpression(value))) value = value.expression;
      if (value && ts.isObjectLiteralExpression(value)) visit(value, []);
    }
  });
  if (strings.size === 0) throw new Error(`No translations found in ${repoPath(TRANSLATIONS_FILE)}.`);
  return strings;
}

/** Turns typesafe-i18n syntax into readable text: `{name:string}` becomes `{name}`, `{{field|fields}}` `field/fields`. */
function displayText(text) {
  return collapse(
    text
      .replace(/\{\{([^}]*)\}\}/g, (_match, forms) => forms.split('|').join('/'))
      .replace(/\{(\w+)(?::[^}]*)?\}/g, '{$1}'),
  );
}

// ---------------------------------------------------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------------------------------------------------

/** The URL of a route file: its folder below `routes`, without the `(group)` folders. */
function routeOf(file) {
  const folders = path.relative(ROUTES_DIR, path.dirname(file)).split(path.sep);
  return '/' + folders.filter((folder) => folder && !/^\(.*\)$/.test(folder)).join('/');
}

/** All `+page` and `+layout` components below `routes`, in a platform-independent order. */
function findRouteFiles() {
  const pages = [];
  const layouts = [];
  const visit = (directory) => {
    const entries = readdirSync(directory, { withFileTypes: true }).sort((a, b) => compare(a.name, b.name));
    for (const entry of entries) {
      const file = path.join(directory, entry.name);
      if (entry.isDirectory()) visit(file);
      else if (/^\+page(@.*)?\.svelte$/.test(entry.name)) pages.push(file);
      else if (/^\+layout(@.*)?\.svelte$/.test(entry.name)) layouts.push(file);
    }
  };
  visit(ROUTES_DIR);
  return { pages, layouts };
}

/**
 * Finds the route that a string from the code points to, such as `/me/settings` or `/credentials/${DYNAMIC}`. Returns
 * `undefined` for anything that is not a route of the app.
 */
function matchRoute(candidate, routes) {
  // A lone `/` is far more often a separator, as in `split('/')`, than a link to the start screen.
  if (!candidate.startsWith('/') || candidate === '/') return undefined;
  const pathname = candidate.split(/[?#]/)[0];
  const segments = pathname
    .split('/')
    .slice(1)
    // A dynamic part glued to a fixed one, as in `certifications${page.url.search}`, is a query string.
    .map((segment) => (segment === DYNAMIC ? DYNAMIC : segment.split(DYNAMIC).join('')));
  const matches = routes.filter((route) => {
    const expected = route.split('/').slice(1);
    if (expected.length !== segments.length) return false;
    return expected.every((part, index) => (part.startsWith('[') ? segments[index] !== '' : segments[index] === part));
  });
  // A fixed segment wins over a parameter: `/x/new` is `/x/new`, not `/x/[id]`.
  const fixedParts = (route) => route.split('/').filter((part) => part && !part.startsWith('[')).length;
  return matches.sort((a, b) => fixedParts(b) - fixedParts(a) || compare(a, b))[0];
}

// ---------------------------------------------------------------------------------------------------------------------
// Reading a Svelte component
// ---------------------------------------------------------------------------------------------------------------------

/** The translation key of an expression such as `$LL.SCAN.TITLE_1`, or `undefined` if it is something else. */
function translationKey(node) {
  const parts = [];
  let current = node;
  while (current.type === 'MemberExpression' && !current.computed && current.property.type === 'Identifier') {
    parts.unshift(current.property.name);
    current = current.object;
  }
  const isTranslations = current.type === 'Identifier' && (current.name === '$LL' || current.name === 'LL');
  return isTranslations && parts.length > 0 && parts.every((part) => /^[A-Z][A-Z0-9_]*$/.test(part))
    ? parts.join('.')
    : undefined;
}

/** Calls `visitor` for every node below `root`. A visitor that returns `false` skips the content of its node. */
function walk(root, visitor) {
  const visit = (node) => {
    if (!node || typeof node !== 'object') return;
    if (Array.isArray(node)) return node.forEach(visit);
    if (typeof node.type === 'string' && visitor(node) === false) return;
    for (const [key, value] of Object.entries(node)) {
      if (!SKIPPED_NODE_KEYS.has(key)) visit(value);
    }
  };
  visit(root);
}

/**
 * Collects what a piece of a Svelte component refers to: translation keys, strings that may be routes and the types of
 * dispatched actions, each with its position in the file. Comments are not part of the syntax tree, so commented-out
 * code is ignored.
 */
function collectReferences(root) {
  const found = { keys: [], paths: [], actions: [] };
  walk(root, (node) => {
    if (node.type === 'Comment') return false;
    if (node.type === 'MemberExpression') {
      const key = translationKey(node);
      if (key) {
        found.keys.push({ value: key, start: node.start });
        return false;
      }
    }
    if (node.type === 'Literal' && typeof node.value === 'string') {
      if (node.value.startsWith('/')) found.paths.push({ value: node.value, start: node.start });
      else if (ACTION_TYPE.test(node.value)) found.actions.push({ value: node.value, start: node.start });
    }
    if (node.type === 'TemplateLiteral') {
      const value = node.quasis.map((quasi) => quasi.value.cooked).join(DYNAMIC);
      if (value.startsWith('/')) found.paths.push({ value, start: node.start });
    }
    if (node.type === 'Attribute' && node.name === 'href' && Array.isArray(node.value)) {
      const value = node.value.map((part) => (part.type === 'Text' ? part.data : DYNAMIC)).join('');
      if (value.startsWith('/')) found.paths.push({ value, start: node.start });
    }
    return true;
  });
  return found;
}

/** The imports of a script block: `[{ specifier, bindings: [{ imported, local }] }]`, without type imports. */
function findImports(script) {
  const imports = [];
  for (const statement of script?.content?.body ?? []) {
    if (statement.type !== 'ImportDeclaration' || statement.importKind === 'type') continue;
    const bindings = statement.specifiers
      .filter((specifier) => specifier.importKind !== 'type')
      .map((specifier) => ({
        imported: specifier.type === 'ImportSpecifier' ? specifier.imported.name : 'default',
        local: specifier.local.name,
      }));
    if (bindings.length > 0) imports.push({ specifier: statement.source.value, bindings });
  }
  return imports;
}

/** The text parts of an attribute value or of child nodes: literal text and translation keys. */
function textParts(nodes) {
  if (nodes === true || nodes === undefined || nodes === null) return [];
  return (Array.isArray(nodes) ? nodes : [nodes]).flatMap((node) => {
    if (node.type === 'Text') {
      const text = collapse(node.data);
      return /\p{L}/u.test(text) ? [{ text }] : [];
    }
    if (node.type === 'ExpressionTag') {
      const expression = node.expression;
      if (expression.type === 'Literal' && typeof expression.value === 'string') {
        const text = collapse(expression.value);
        return /\p{L}/u.test(text) ? [{ text }] : [];
      }
      return collectReferences(expression).keys.map((key) => ({ key: key.value }));
    }
    return [];
  });
}

const attributeOf = (node, name) =>
  node.attributes?.find((attribute) => attribute.type === 'Attribute' && attribute.name === name);

/** The click handler of an element, in the Svelte 4 (`on:click`) or Svelte 5 (`onclick`) spelling. */
const clickHandlerOf = (node) =>
  node.attributes?.find(
    (attribute) =>
      (attribute.type === 'OnDirective' && attribute.name === 'click') ||
      (attribute.type === 'Attribute' && attribute.name === 'onclick'),
  );

const childFragments = (node) =>
  Object.entries(node)
    .filter(([key, value]) => !SKIPPED_NODE_KEYS.has(key) && value?.type === 'Fragment')
    .map(([, value]) => value);

const rangeOf = (fragment) =>
  fragment?.nodes?.length ? [fragment.nodes[0].start, fragment.nodes.at(-1).end] : undefined;

/** Attributes and directives, such as `class="..."` and `on:click`: their values are no content of the element. */
const isAttribute = (node) => /Attribute$|Directive$/.test(node.type);

/** Whether an element is a control the user operates, and of which kind. */
function controlKind(node) {
  if (node.type !== 'RegularElement' && node.type !== 'Component') return undefined;
  const isComponent = node.type === 'Component';
  if (isComponent ? attributeOf(node, 'placeholder') : ['input', 'textarea', 'select'].includes(node.name)) {
    return 'field';
  }
  if (isComponent && /(Switch|Checkbox)$/.test(node.name)) return 'switch';
  if (attributeOf(node, 'href') || (!isComponent && node.name === 'a')) return 'link';
  if (clickHandlerOf(node) || (isComponent ? node.name === 'Button' : node.name === 'button')) return 'button';
  return undefined;
}

/** Whether an element has a control somewhere in its content, such as a card with a delete button. */
function containsControl(node) {
  let found = false;
  walk(childFragments(node), (current) => {
    if (controlKind(current)) found = true;
    return !found;
  });
  return found;
}

/**
 * Everything in a control that can serve as its label: texts, translation keys and the names of icons. Controls inside
 * the control are left out, because they are listed on their own.
 */
function describeContent(node) {
  const parts = [];
  const icons = [];
  let dynamic = false;
  walk(childFragments(node), (current) => {
    if (current.type === 'Comment' || isAttribute(current) || controlKind(current)) return false;
    if (current.type === 'Text') parts.push(...textParts(current));
    if (current.type === 'ExpressionTag') {
      const found = textParts(current);
      if (found.length === 0) dynamic = true;
      parts.push(...found);
      return false;
    }
    if (current.type === 'Component') {
      if (/Icon$/.test(current.name)) icons.push(current.name.replace(/Icon$/, ''));
      for (const prop of ['label', 'title']) parts.push(...textParts(attributeOf(current, prop)?.value));
    }
    return true;
  });
  return { parts, icons, dynamic };
}

const parsed = new Map();
/** The syntax tree of a component and the files of the components it imports, by the name used in the template. */
function parseComponent(file) {
  if (!parsed.has(file)) {
    const ast = parse(read(file), { modern: true, filename: file });
    const imports = [...findImports(ast.module), ...findImports(ast.instance)];
    const componentFiles = new Map();
    const modules = [];
    for (const { specifier, bindings } of imports) {
      const target = resolveImport(file, specifier);
      if (!target) continue;
      if (target.endsWith('.svelte')) bindings.forEach((binding) => componentFiles.set(binding.local, target));
      else if (barrelExports(target).size > 0) {
        for (const binding of bindings) {
          const exported = barrelExports(target).get(binding.imported);
          if (exported) componentFiles.set(binding.local, exported);
        }
      } else {
        modules.push({
          file: target,
          names: bindings.map((binding) => binding.imported),
        });
      }
    }
    parsed.set(file, { ast, componentFiles, modules });
  }
  return parsed.get(file);
}

const analyzed = new Map();
/**
 * Reads one Svelte component: the title bar and the controls (buttons, links, fields and switches) of its template,
 * plus every route and action type that the component refers to anywhere.
 *
 * How a component is used decides what it shows, so two things about the use are taken into account:
 * - `off` lists the props that the using component sets to `false`. Content inside `{#if prop}` is then left out: the
 *   credential details, for example, hide their menu with `actions={false}` while a credential is being shared.
 * - `passed` lists all props that the using component sets. The default value of such a prop is left out: a summary
 *   card that links to `/prompt/...` by default links elsewhere when it gets an `href`.
 */
function analyzeComponent(file, off = [], passed = []) {
  const id = [file, off.join(','), passed.join(',')].join('\n');
  if (analyzed.has(id)) return analyzed.get(id);

  const { ast, componentFiles, modules } = parseComponent(file);
  const component = { titles: [], controls: [], uses: [], modules };
  // Parts of the template that only show in developer mode, and parts that the use of the component rules out.
  const developerRanges = [];
  const hiddenRanges = [];

  // Default values of props that the using component sets: `export let x = ...` and `let { x = ... } = $props()`.
  for (const statement of ast.instance?.content?.body ?? []) {
    const declaration = statement.type === 'ExportNamedDeclaration' ? statement.declaration : statement;
    if (declaration?.type !== 'VariableDeclaration') continue;
    for (const { id: target, init } of declaration.declarations) {
      if (statement.type === 'ExportNamedDeclaration' && target.type === 'Identifier' && init) {
        if (passed.includes(target.name)) hiddenRanges.push([init.start, init.end]);
      }
      if (target.type === 'ObjectPattern' && init?.type === 'CallExpression' && init.callee.name === '$props') {
        for (const property of target.properties) {
          if (property.type !== 'Property' || property.value.type !== 'AssignmentPattern') continue;
          if (passed.includes(property.key.name))
            hiddenRanges.push([property.value.right.start, property.value.right.end]);
        }
      }
    }
  }
  // The same for a fallback further on: `href ?? '/prompt/...'`.
  walk([ast.instance, ast.fragment], (node) => {
    const isFallback = node.type === 'LogicalExpression' && node.operator === '??' && node.left.type === 'Identifier';
    if (isFallback && passed.includes(node.left.name)) hiddenRanges.push([node.right.start, node.right.end]);
  });

  const visitAll = (nodes) => nodes.forEach(visit);
  const visit = (node) => {
    switch (node.type) {
      case 'Comment':
      case 'Text':
      case 'ExpressionTag':
        return;
      case 'IfBlock': {
        const test = node.test;
        const isOff = (expression) => expression.type === 'Identifier' && off.includes(expression.name);
        const hidden = isOff(test)
          ? node.consequent
          : test.type === 'UnaryExpression' && test.operator === '!' && isOff(test.argument)
            ? node.alternate
            : undefined;
        if (hidden !== undefined || isOff(test)) {
          if (rangeOf(hidden)) hiddenRanges.push(rangeOf(hidden));
          for (const fragment of [node.consequent, node.alternate]) {
            if (fragment && fragment !== hidden) visitAll(fragment.nodes);
          }
          return;
        }
        const names = [];
        walk(test, (current) => {
          if (current.type === 'Identifier') names.push(current.name);
        });
        if (names.includes('dev_mode') || names.includes('show_dev_mode_setting')) {
          // `dev_mode === 'Off'` shows its content outside developer mode, and its `else` branch inside it.
          const negated = test.type === 'BinaryExpression' && test.operator.startsWith('==');
          const range = rangeOf(negated ? node.alternate : node.consequent);
          if (range) developerRanges.push(range);
        }
        break;
      }
      case 'RegularElement':
      case 'Component':
        if (visitElement(node)) return;
        break;
    }
    for (const fragment of childFragments(node)) visitAll(fragment.nodes);
  };

  /** Records an element that is a title bar or a control. Returns `true` if its content needs no further reading. */
  const visitElement = (node) => {
    const isComponent = node.type === 'Component';
    if (isComponent && /NavBar$/.test(node.name) && attributeOf(node, 'title')) {
      component.titles.push({ parts: textParts(attributeOf(node, 'title').value), start: node.start });
      return true;
    }

    const kind = controlKind(node);
    const references = collectReferences([clickHandlerOf(node), attributeOf(node, 'href')]);
    const isContainer = kind !== undefined && containsControl(node);
    // A clickable area around other controls, such as the backdrop of a sheet, counts as a control only when the code
    // states what a click on it does.
    if (!kind || (isContainer && references.paths.length + references.actions.length === 0)) return false;

    const content = describeContent(node);
    const labelProps = ['label', 'placeholder', 'title', 'aria-label'].filter((name) => attributeOf(node, name));
    const ownLabel = labelProps
      .map((name) => textParts(attributeOf(node, name).value))
      .find((parts) => parts.length > 0);
    component.controls.push({
      kind,
      parts: ownLabel ?? content.parts,
      icons: content.icons,
      // Part of the label comes from a variable, such as the name of a credential.
      dynamic: ownLabel ? false : content.dynamic || labelProps.length > 0,
      paths: references.paths.map((reference) => reference.value),
      actions: references.actions.map((reference) => reference.value),
      start: node.start,
    });
    return !isContainer;
  };

  visitAll(ast.fragment.nodes);

  // The components this one uses, each with the props it sets and the props it sets to `false`.
  const writtenAsTag = new Set();
  walk(ast.fragment, (node) => {
    if (node.type === 'Comment') return false;
    if (node.type !== 'Component' || !componentFiles.has(node.name)) return true;
    writtenAsTag.add(node.name);
    if (within(hiddenRanges, node.start)) return false;
    const props = (node.attributes ?? []).filter((attribute) => attribute.type === 'Attribute');
    const isFalse = (attribute) => {
      if (attribute.value === true || Array.isArray(attribute.value)) return false;
      const expression = attribute.value.expression;
      return (
        (expression.type === 'Literal' && expression.value === false) ||
        (expression.type === 'Identifier' && off.includes(expression.name))
      );
    };
    component.uses.push({
      file: componentFiles.get(node.name),
      off: props
        .filter(isFalse)
        .map((attribute) => attribute.name)
        .sort(compare),
      passed: props.map((attribute) => attribute.name).sort(compare),
    });
    return true;
  });
  // A component that is imported but not written as a tag is rendered dynamically, for example by `<svelte:component>`.
  for (const [name, target] of componentFiles) {
    if (!writtenAsTag.has(name)) component.uses.push({ file: target, off: [], passed: [] });
  }

  const references = collectReferences([ast.module, ast.instance, ast.fragment]);
  const visible = (item) => !within(hiddenRanges, item.start);
  const withScope = (item) => ({ ...item, developerOnly: within(developerRanges, item.start) });
  for (const name of ['titles', 'controls']) component[name] = component[name].map(withScope);
  for (const name of ['paths', 'actions']) component[name] = references[name].filter(visible).map(withScope);

  analyzed.set(id, component);
  return component;
}

/**
 * Reads the functions that a component imports from a TypeScript module, such as `cancelUserFlow` from
 * `$lib/navigation`, including the functions of the same module that they call.
 */
function analyzeModule(file, names) {
  const sourceFile = ts.createSourceFile(file, read(file), ts.ScriptTarget.Latest, true);
  const declarations = new Map();
  for (const statement of sourceFile.statements) {
    if (ts.isFunctionDeclaration(statement) && statement.name) declarations.set(statement.name.text, statement);
    if (ts.isVariableStatement(statement)) {
      for (const declaration of statement.declarationList.declarations) {
        if (ts.isIdentifier(declaration.name)) declarations.set(declaration.name.text, declaration);
      }
    }
  }

  const found = { paths: [], actions: [] };
  const done = new Set();
  const queue = names.filter((name) => declarations.has(name));
  const visit = (node) => {
    if (ts.isStringLiteralLike(node)) {
      if (node.text.startsWith('/')) found.paths.push({ value: node.text });
      else if (ACTION_TYPE.test(node.text)) found.actions.push({ value: node.text });
    }
    if (ts.isTemplateExpression(node)) {
      const value = [node.head.text, ...node.templateSpans.map((span) => span.literal.text)].join(DYNAMIC);
      if (value.startsWith('/')) found.paths.push({ value });
    }
    if (ts.isIdentifier(node) && declarations.has(node.text) && !done.has(node.text)) queue.push(node.text);
    ts.forEachChild(node, visit);
  };
  while (queue.length > 0) {
    const name = queue.shift();
    if (done.has(name)) continue;
    done.add(name);
    ts.forEachChild(declarations.get(name), visit);
  }
  return found;
}

/** Resolves an import to a file below `unime/src`. Packages, generated bindings and translations resolve to nothing. */
function resolveImport(fromFile, specifier) {
  let base;
  if (specifier.startsWith('.')) base = path.resolve(path.dirname(fromFile), specifier);
  else if (specifier === '$lib' || specifier.startsWith('$lib/')) base = path.join(SRC, specifier.slice(1));
  else return undefined;
  return [base, `${base}.ts`, `${base}.svelte`, path.join(base, 'index.ts')].find(isFile);
}

const barrels = new Map();
/** For an `index.ts` that re-exports components: a map from the exported name to the component's file. */
function barrelExports(file) {
  if (!barrels.has(file)) {
    const exports = new Map();
    for (const match of read(file).matchAll(/export \{ default as (\w+) \} from '([^']+)';/g)) {
      const target = resolveImport(file, match[2]);
      if (target) exports.set(match[1], target);
    }
    barrels.set(file, exports);
  }
  return barrels.get(file);
}

/**
 * Everything a screen or layout shows and does: its own component followed by the components it uses, directly or
 * through other components, and the functions it imports from TypeScript modules.
 */
function analyzeScreen(entryFile) {
  const parts = [];
  const seen = new Set();
  const visit = (file, off, passed) => {
    const id = [file, off.join(','), passed.join(',')].join('\n');
    if (seen.has(id)) return;
    seen.add(id);
    const component = analyzeComponent(file, off, passed);
    parts.push(component, ...component.modules.map((module) => analyzeModule(module.file, module.names)));
    for (const use of component.uses) visit(use.file, use.off, use.passed);
  };
  visit(entryFile, [], []);
  const merge = (name) => parts.flatMap((part) => part[name] ?? []);
  return {
    titles: merge('titles'),
    controls: merge('controls'),
    paths: merge('paths'),
    actions: merge('actions'),
  };
}

// ---------------------------------------------------------------------------------------------------------------------
// Writing the reference
// ---------------------------------------------------------------------------------------------------------------------

/** Escapes text from the app so that Markdown and MDX show it literally. */
const escapeMarkdown = (text) => text.replace(/[\\`*_{}[\]<>|~]/g, '\\$&');
const quote = (text) => `"${escapeMarkdown(text)}"`;
const code = (text) => `\`${text}\``;
const developerNote = (item) => (item.developerOnly ? ' (developer mode only)' : '');

function buildReference() {
  const translations = loadTranslations();
  const config = JSON.parse(read(SCREENS_FILE));
  const { pages, layouts } = findRouteFiles();

  const omittedRoutes = new Map(config.omittedRoutes.map((entry) => [entry.route, entry.reason]));

  const pageByRoute = new Map(pages.map((file) => [routeOf(file), file]));
  const routes = [...pageByRoute.keys()].sort(compare);
  const described = config.groups.flatMap((group) => group.screens);
  const nameByRoute = new Map(described.map((screen) => [screen.route, screen.name]));
  // The folder of a layout below `routes`, with its `(group)` folders: `/(app)` or `/prompt`.
  const layoutId = (file) => `/${path.relative(ROUTES_DIR, path.dirname(file)).split(path.sep).join('/')}`;
  const layoutNotes = new Map(config.layouts.map((entry) => [entry.layout, entry.note]));

  // The screens file and the route tree have to agree.
  for (const route of routes) {
    if (!nameByRoute.has(route) && !omittedRoutes.has(route)) {
      problems.push(
        `The route ${route} (${repoPath(pageByRoute.get(route))}) is not described. Add it to a group in ` +
          `${repoPath(SCREENS_FILE)} with a name and a purpose, or to "omittedRoutes" with a reason.`,
      );
    }
  }
  for (const route of [...nameByRoute.keys(), ...omittedRoutes.keys()]) {
    if (!pageByRoute.has(route)) {
      problems.push(`${repoPath(SCREENS_FILE)} describes the route ${route}, which does not exist. Remove the entry.`);
    }
  }
  for (const screen of described) {
    if (omittedRoutes.has(screen.route)) problems.push(`The route ${screen.route} is both described and omitted.`);
    if (!/^[A-Za-z0-9][A-Za-z0-9 ]*$/.test(screen.name ?? '')) {
      problems.push(`The name of ${screen.route} must consist of letters, digits and spaces.`);
    }
    if (!screen.purpose?.trim()) problems.push(`The route ${screen.route} has no purpose.`);
  }
  for (const name of unique(described.map((screen) => screen.name))) {
    if (described.filter((screen) => screen.name === name).length > 1) {
      problems.push(`The screen name "${name}" is used more than once.`);
    }
  }
  // Links to a screen point at its heading, so no other heading may produce the same anchor.
  const headingAnchor = (heading) =>
    heading
      .toLowerCase()
      .replace(/[^a-z0-9 -]/g, '')
      .replaceAll(' ', '-');
  const otherHeadings = ['How to read an entry', 'Screens', 'Layouts', ...config.groups.map((group) => group.title)];
  for (const screen of described) {
    const clash = otherHeadings.find((heading) => headingAnchor(heading) === headingAnchor(screen.name ?? ''));
    if (clash) problems.push(`The screen name "${screen.name}" clashes with the heading "${clash}". Rename one.`);
  }
  for (const layout of layoutNotes.keys()) {
    if (!layouts.some((file) => layoutId(file) === layout)) {
      problems.push(`"layouts" names the layout ${layout}, which does not exist. Remove the entry.`);
    }
  }
  if (problems.length > 0) return undefined;

  /** The texts of a translation key. A key that names a group, as in `$LL.SETTINGS.THEME[theme]()`, gives all of it. */
  const textsOfKey = (key) =>
    [...translations.keys()]
      .filter((known) => known === key || known.startsWith(`${key}.`))
      .map((known) => displayText(translations.get(known)));
  const textsOfParts = (parts) => parts.flatMap((part) => (part.key ? textsOfKey(part.key) : [part.text]));

  const anchorOf = (name) => name.toLowerCase().replaceAll(' ', '-');
  const screenLink = (route) => `[${nameByRoute.get(route)}](#${anchorOf(nameByRoute.get(route))})`;
  /** The described routes that a list of strings from the code points to. */
  const targetsOf = (paths, ownRoute) =>
    unique(paths.map((candidate) => matchRoute(candidate, routes))).filter(
      (route) => route && route !== ownRoute && nameByRoute.has(route),
    );
  /** Whether a string from the code points to a route that is left out of the reference. */
  const pointsToOmittedRoute = (paths) => paths.some((candidate) => omittedRoutes.has(matchRoute(candidate, routes)));

  /** The label of a control, and whether it is a fixed text (`named`) or a description of the control. */
  const labelOf = (control) => {
    const texts = unique(textsOfParts(control.parts));
    if (texts.length > 0) {
      const label = texts.map(quote).join(' / ');
      return { label: control.dynamic ? `${label} and text from the data` : label, named: true };
    }
    if (control.dynamic) return { label: 'a label taken from the data', named: false };
    if (control.icons.length > 0) return { label: `the ${unique(control.icons).join(', ')} icon`, named: false };
    return undefined;
  };

  /** The lines that describe what a screen or a layout shows and does. */
  const describe = (analysis, ownRoute) => {
    const lines = [];

    const titles = unique(analysis.titles.flatMap((title) => textsOfParts(title.parts)));
    if (titles.length > 0) lines.push(`- **Title bar:** ${titles.map(quote).join(' or ')}`);

    const controls = [];
    for (const control of analysis.controls) {
      if (pointsToOmittedRoute(control.paths)) continue;
      const description = labelOf(control);
      const kind = { button: 'Button', link: 'Link', field: 'Field', switch: 'Switch' }[control.kind];
      const effects = [
        ...targetsOf(control.paths, ownRoute).map((route) => `opens ${screenLink(route)}`),
        ...unique(control.actions).map((action) => `dispatches ${code(action)}`),
      ];
      // A control without a fixed label says something only when the code states what it does. This also leaves out
      // the inner elements of shared components such as `Button`, whose label is a prop.
      if (!description || (!description.named && effects.length === 0)) continue;
      controls.push(
        `  - ${kind}: ${description.label}${effects.length > 0 ? `, ${effects.join(', ')}` : ''}${developerNote(control)}`,
      );
    }
    if (controls.length > 0) lines.push('- **Controls:**', ...unique(controls));

    const opens = targetsOf(
      analysis.paths.map((item) => item.value),
      ownRoute,
    );
    if (opens.length > 0) lines.push(`- **Opens:** ${opens.map(screenLink).join(', ')}`);

    const actions = unique(analysis.actions.map((item) => item.value));
    if (actions.length > 0) lines.push(`- **Backend actions:** ${actions.map(code).join(', ')}`);
    return { lines, opens };
  };

  const descriptions = new Map(
    described.map((screen) => [screen.route, describe(analyzeScreen(pageByRoute.get(screen.route)), screen.route)]),
  );
  const layoutSections = layouts.map((file) => ({
    file,
    id: layoutId(file),
    ...describe(analyzeScreen(file), undefined),
  }));

  /** The screens and layouts that open a route. */
  const openedFrom = (route) => [
    ...described
      .filter((screen) => descriptions.get(screen.route).opens.includes(route))
      .map((screen) => screenLink(screen.route)),
    ...layoutSections.filter((layout) => layout.opens.includes(route)).map((layout) => `the layout ${code(layout.id)}`),
  ];

  const out = [];
  out.push(
    '# UniMe app reference',
    '',
    'Every screen of the UniMe app: what it is for, its controls with their English labels, the screens it opens and the actions it sends to the backend.',
    '',
    `This file is generated by ${code('scripts/generate-app-reference.mjs')} from the routes in ${code('unime/src/routes')}, the Svelte components of each screen and the English translations in ${code('unime/src/i18n/en/index.ts')}. Do not edit it by hand. Change the code, or the names and purposes in ${code('scripts/app-reference-screens.json')}, and run ${code('pnpm docs:app-reference')}.`,
    '',
    '## How to read an entry',
    '',
    `- The line under the name gives the route of the screen and the file that implements it. A part in square brackets, as in ${code('/credentials/[id]')}, is a parameter.`,
    '- **Title bar** is the title in the bar at the top of the screen.',
    '- **Controls** are the buttons, links, fields and switches, with the screen each one opens and the backend action it dispatches where the code states that directly. A control whose handler is a function shows no effect here. The lines **Opens** and **Backend actions** list everything the screen can open or dispatch.',
    `- In a label, ${code('{name}')} marks a value that the app fills in, and ${code('one/other')} marks a singular and a plural form.`,
    '- **Opened from** lists the screens with a control or a redirect that leads here. Screens that the backend opens, such as the prompts, say so in their purpose.',
    `- Backend actions are handled by the reducers in ${code('identity-wallet/src/state')}.`,
    '- Controls that only show in developer mode are marked.',
    '- An entry lists every control that the components of a screen can show. Which of them show at a given moment depends on the state of the wallet.',
    '',
    '## Screens',
    '',
  );
  for (const group of config.groups) {
    out.push(`- ${group.title}: ${group.screens.map((screen) => screenLink(screen.route)).join(', ')}`);
  }
  out.push('');

  for (const group of config.groups) {
    out.push(`## ${group.title}`, '', group.description, '');
    for (const screen of group.screens) {
      const sources = openedFrom(screen.route);
      out.push(
        `### ${screen.name}`,
        '',
        `${code(screen.route)} in ${code(repoPath(pageByRoute.get(screen.route)))}`,
        '',
        screen.purpose,
        '',
        ...descriptions.get(screen.route).lines,
        ...(sources.length > 0 ? [`- **Opened from:** ${sources.join(', ')}`] : []),
        '',
      );
    }
  }

  const layoutsWithContent = layoutSections.filter((layout) => layout.lines.length > 0 || layoutNotes.has(layout.id));
  if (layoutsWithContent.length > 0) {
    out.push(
      '## Layouts',
      '',
      `A layout wraps every screen in its folder below ${code('unime/src/routes')}. What a layout shows and does applies to all of those screens, so it is listed here once. The layout ${code('/')} wraps every screen of the app.`,
      '',
    );
    for (const layout of layoutsWithContent) {
      out.push(
        `### Layout ${code(layout.id)}`,
        '',
        code(repoPath(layout.file)),
        '',
        ...(layoutNotes.has(layout.id) ? [layoutNotes.get(layout.id), ''] : []),
        ...layout.lines,
        '',
      );
    }
  }

  return out.join('\n').replace(/\n+$/, '\n');
}

const reference = buildReference();
if (problems.length > 0) {
  console.error(
    `The app reference cannot be generated:\n${unique(problems)
      .map((problem) => `- ${problem}`)
      .join('\n')}`,
  );
  process.exit(1);
}

if (process.argv.includes('--check')) {
  const current = existsSync(OUTPUT_FILE) ? read(OUTPUT_FILE) : '';
  if (current !== reference) {
    console.error(`${repoPath(OUTPUT_FILE)} is out of date. Run \`pnpm docs:app-reference\` and commit the result.`);
    process.exit(1);
  }
  console.log(`${repoPath(OUTPUT_FILE)} is up to date.`);
} else {
  writeFileSync(OUTPUT_FILE, reference);
  console.log(`Wrote ${repoPath(OUTPUT_FILE)}.`);
}
