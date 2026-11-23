#!/usr/bin/env node
// Generates `docs/app-reference.md`: every screen of the UniMe app with its purpose, its title and the screens it
// leads to.
//
// The script reads three things from the code. Each is a plain fact that needs no interpretation:
// - the route tree in `unime/src/routes`: which screens exist,
// - the title that a screen passes to its navigation bar, in the English base translations,
// - the routes that the files of a screen name in a string, as in `goto('/me/settings')` or `href="/me/add"`.
//
// The hand-written part is `scripts/app-reference-screens.json`. It gives every screen a name and one to three
// sentences on its purpose, and sets the reading order. The script fails when a route is missing from that file or
// when the file names a route that no longer exists.
//
// What a screen shows in detail, such as its buttons and when each of them is visible, is left out on purpose. That
// can only be guessed from how a component is written, and the guess breaks when the code changes style.
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
const ONLY_DYNAMIC = /^\u0000+$/;

const read = (file) => readFileSync(file, 'utf8').replace(/\r\n/g, '\n');
const repoPath = (file) => path.relative(ROOT, file).split(path.sep).join('/');
const compare = (a, b) => (a < b ? -1 : a > b ? 1 : 0);
const unique = (items) => [...new Set(items)];
const isFile = (file) => existsSync(file) && statSync(file).isFile();

// ---------------------------------------------------------------------------------------------------------------------
// Translations
// ---------------------------------------------------------------------------------------------------------------------

/** The English base translations as a map from key (`SETTINGS.NAVBAR_TITLE`) to text. */
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

// ---------------------------------------------------------------------------------------------------------------------
// Routes
// ---------------------------------------------------------------------------------------------------------------------

/** The URL of a page: its folder below `routes`, without the `(group)` folders. */
function routeOf(file) {
  const folders = path.relative(ROUTES_DIR, path.dirname(file)).split(path.sep);
  return '/' + folders.filter((folder) => folder && !/^\(.*\)$/.test(folder)).join('/');
}

/** All `+page` components below `routes`, in a platform-independent order. */
function findPages() {
  const pages = [];
  const visit = (directory) => {
    const entries = readdirSync(directory, { withFileTypes: true }).sort((a, b) => compare(a.name, b.name));
    for (const entry of entries) {
      const file = path.join(directory, entry.name);
      if (entry.isDirectory()) visit(file);
      else if (/^\+page(@.*)?\.svelte$/.test(entry.name)) pages.push(file);
    }
  };
  visit(ROUTES_DIR);
  return pages;
}

/**
 * Finds the route that a string from the code names, such as `/me/settings` or `/credentials/${DYNAMIC}`. Returns
 * `undefined` for a string that is no route of the app. A lone `/` is never taken as a link to the start screen: it is
 * far more often a separator, as in `split('/')`.
 */
function matchRoute(candidate, routes) {
  if (!candidate.startsWith('/') || candidate === '/') return undefined;
  const pathname = candidate.split(/[?#]/)[0];
  const segments = pathname
    .split('/')
    .slice(1)
    // A segment without a fixed part, as in `${id}` or `${id}${page.url.search}`, is a parameter. A dynamic part glued
    // to a fixed one, as in `certifications${page.url.search}`, is a query string.
    .map((segment) => (ONLY_DYNAMIC.test(segment) ? DYNAMIC : segment.split(DYNAMIC).join('')));
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

/** Calls `visitor` for every node of a syntax tree. A visitor that returns `false` skips the content of its node. */
function walk(root, visitor) {
  const visit = (node) => {
    if (!node || typeof node !== 'object') return;
    if (Array.isArray(node)) return node.forEach(visit);
    if (typeof node.type === 'string' && visitor(node) === false) return;
    for (const [key, value] of Object.entries(node)) {
      // These keys hold positions and comments, not code.
      if (!['metadata', 'loc', 'name_loc', 'leadingComments', 'trailingComments'].includes(key)) visit(value);
    }
  };
  visit(root);
}

/** Resolves an import to a file below `unime/src`. Packages, generated bindings and translations resolve to nothing. */
function resolveImport(fromFile, specifier) {
  let base;
  if (specifier.startsWith('.')) base = path.resolve(path.dirname(fromFile), specifier);
  else if (specifier === '$lib' || specifier.startsWith('$lib/')) base = path.join(SRC, specifier.slice(1));
  else return undefined;
  return [base, `${base}.ts`, `${base}.svelte`, path.join(base, 'index.ts')].find(isFile);
}

/** For an `index.ts` that re-exports components: a map from the exported name to the component's file. */
function reexportedComponents(file) {
  const components = new Map();
  const sourceFile = ts.createSourceFile(file, read(file), ts.ScriptTarget.Latest, true);
  for (const statement of sourceFile.statements) {
    // `export { default as Button } from './Button.svelte';`
    if (!ts.isExportDeclaration(statement) || !statement.moduleSpecifier || !statement.exportClause?.elements) continue;
    const target = resolveImport(file, statement.moduleSpecifier.text);
    if (!target?.endsWith('.svelte')) continue;
    for (const element of statement.exportClause.elements) {
      if (element.propertyName?.text === 'default') components.set(element.name.text, target);
    }
  }
  return components;
}

/**
 * The titles that a `title` attribute names: fixed texts, and keys of translations as in `title={$LL.A.B()}`. An
 * attribute that picks between titles, as in `title={many ? $LL.A() : $LL.B()}`, gives all of them. Anything else,
 * such as a variable or a text with a value in it, gives no title.
 */
function titlesOf(attribute) {
  // A text with a value in it, as in `title="Wallet {profileName}"`, is a `Text` next to an `ExpressionTag`. Its text
  // alone is not the title. Spaces around a single value, as in `title=" {$LL.A()} "`, still name that value.
  const parts = Array.isArray(attribute.value) ? attribute.value : [];
  const hasText = parts.some((part) => part.type === 'Text' && part.data.trim());
  if (hasText && parts.some((part) => part.type === 'ExpressionTag')) return [];
  const titles = [];
  const addText = (text) => typeof text === 'string' && text.trim() && titles.push({ text: text.trim() });
  const visit = (node) => {
    if (Array.isArray(node)) return node.forEach(visit);
    switch (node?.type) {
      case 'Text':
        return addText(node.data);
      case 'Literal':
        return addText(node.value);
      case 'TemplateLiteral':
        return node.expressions.length === 0 && addText(node.quasis[0].value.cooked);
      case 'ExpressionTag':
        return visit(node.expression);
      // Only the outcomes name a title. A text in the condition, as in `kind === 'personal' ? ... : ...`, does not.
      case 'ConditionalExpression':
        return visit([node.consequent, node.alternate]);
      case 'LogicalExpression':
        return visit([node.left, node.right]);
      case 'CallExpression': {
        const key = [];
        let callee = node.callee;
        while (callee.type === 'MemberExpression' && !callee.computed) {
          key.unshift(callee.property.name);
          callee = callee.object;
        }
        if (callee.type === 'Identifier' && callee.name === '$LL' && key.length > 0)
          titles.push({ key: key.join('.') });
      }
    }
  };
  visit(attribute.value);
  return titles;
}

const components = new Map();
/**
 * What one Svelte component states: the components it imports, the strings in its code that start with `/`, and the
 * titles it gives to a navigation bar. Comments are not part of the syntax tree, so commented-out code is ignored.
 */
function readComponent(file) {
  if (components.has(file)) return components.get(file);
  const ast = parse(read(file), { modern: true, filename: file });
  const component = { imports: [], paths: [], titles: [] };
  walk([ast.module, ast.instance, ast.fragment], (node) => {
    if (node.type === 'Comment') return false;
    if (node.type === 'ImportDeclaration') {
      const target = node.importKind === 'type' ? undefined : resolveImport(file, node.source.value);
      if (target?.endsWith('.svelte')) component.imports.push(target);
      else if (target) {
        const reexported = reexportedComponents(target);
        for (const specifier of node.specifiers) {
          if (reexported.has(specifier.imported?.name)) component.imports.push(reexported.get(specifier.imported.name));
        }
      }
      return false;
    }
    if (node.type === 'Literal' && typeof node.value === 'string') component.paths.push(node.value);
    if (node.type === 'TemplateLiteral') {
      component.paths.push(node.quasis.map((quasi) => quasi.value.cooked).join(DYNAMIC));
    }
    if (node.type === 'Attribute' && node.name === 'href' && Array.isArray(node.value)) {
      component.paths.push(node.value.map((part) => (part.type === 'Text' ? part.data : DYNAMIC)).join(''));
    }
    if (node.type === 'Component' && /NavBar$/.test(node.name)) {
      const title = node.attributes.find((attribute) => attribute.type === 'Attribute' && attribute.name === 'title');
      if (title) component.titles.push(...titlesOf(title));
    }
    return true;
  });
  components.set(file, component);
  return component;
}

/**
 * The components that make up a screen: its page and the components that the page uses, directly or through other
 * components. Shared components from `$lib` count, and so do components from the folder of the screen, from a folder
 * above it and from a folder below it. A component borrowed from the folder of another screen does not: the links it
 * has by default lead to the screens around its own home, not to the ones around this screen.
 */
function componentsOfScreen(page) {
  const home = path.dirname(page);
  const isOwn = (file) => {
    const folder = path.dirname(file);
    if (!folder.startsWith(ROUTES_DIR)) return true;
    return folder === home || folder.startsWith(home + path.sep) || home.startsWith(folder + path.sep);
  };
  const files = [];
  const visit = (file) => {
    if (files.includes(file) || !isOwn(file)) return;
    files.push(file);
    readComponent(file).imports.forEach(visit);
  };
  visit(page);
  return files.map(readComponent);
}

// ---------------------------------------------------------------------------------------------------------------------
// Writing the reference
// ---------------------------------------------------------------------------------------------------------------------

/** Escapes text from the app so that Markdown and MDX show it literally. */
const escapeMarkdown = (text) => text.replace(/[\\`*_{}[\]<>|~]/g, '\\$&');
const code = (text) => `\`${text}\``;

/** Checks the screens file against the route tree. Returns the problems it finds. */
function validate(config, routes) {
  const problems = [];
  const screens = config.groups.flatMap((group) => group.screens);
  const described = screens.map((screen) => screen.route);
  const omitted = config.omittedRoutes.map((entry) => entry.route);
  const file = repoPath(SCREENS_FILE);

  for (const route of routes) {
    if (!described.includes(route) && !omitted.includes(route)) {
      problems.push(
        `The route ${route} is not described. Add it to a group in ${file} with a name and a purpose, or to ` +
          `"omittedRoutes" with a reason.`,
      );
    }
  }
  for (const route of unique([...described, ...omitted])) {
    if (!routes.includes(route)) problems.push(`${file} names the route ${route}, which does not exist. Remove it.`);
    if (described.includes(route) && omitted.includes(route)) {
      problems.push(`The route ${route} is both described and omitted.`);
    }
    if (described.filter((other) => other === route).length > 1) {
      problems.push(`The route ${route} is described more than once. Keep one entry.`);
    }
  }
  for (const entry of config.omittedRoutes) {
    if (!entry.reason?.trim()) problems.push(`The omitted route ${entry.route} has no reason.`);
  }
  // A link to a screen points at its heading, so a name has to be unique and must not match another heading.
  const anchor = (heading) =>
    heading
      .toLowerCase()
      .replace(/[^a-z0-9 -]/g, '')
      .replaceAll(' ', '-');
  const otherHeadings = ['How to read an entry', 'Screens', ...config.groups.map((group) => group.title)];
  for (const screen of screens) {
    if (!/^[A-Za-z0-9][A-Za-z0-9 ]*$/.test(screen.name ?? '')) {
      problems.push(`The name of ${screen.route} must consist of letters, digits and spaces.`);
      continue;
    }
    if (!screen.purpose?.trim()) problems.push(`The route ${screen.route} has no purpose.`);
    if (screens.filter((other) => anchor(other.name ?? '') === anchor(screen.name)).length > 1) {
      problems.push(`The screen name "${screen.name}" is used more than once, apart from its capitals.`);
    }
    const clash = otherHeadings.find((heading) => anchor(heading) === anchor(screen.name));
    if (clash) problems.push(`The screen name "${screen.name}" clashes with the heading "${clash}". Rename one.`);
  }
  return unique(problems);
}

function buildReference() {
  const config = JSON.parse(read(SCREENS_FILE));
  const pageByRoute = new Map(findPages().map((file) => [routeOf(file), file]));
  const routes = [...pageByRoute.keys()].sort(compare);

  const problems = validate(config, routes);
  if (problems.length > 0) {
    console.error(`The app reference cannot be generated:\n${problems.map((problem) => `- ${problem}`).join('\n')}`);
    process.exit(1);
  }

  const translations = loadTranslations();
  const screens = config.groups.flatMap((group) => group.screens);
  const nameByRoute = new Map(screens.map((screen) => [screen.route, screen.name]));
  const link = (route) => `[${nameByRoute.get(route)}](#${nameByRoute.get(route).toLowerCase().replaceAll(' ', '-')})`;

  // What the code of each screen states: its titles and the described screens it names.
  const facts = new Map();
  for (const screen of screens) {
    const parts = componentsOfScreen(pageByRoute.get(screen.route));
    const titles = parts
      .flatMap((part) => part.titles)
      .map((title) => title.text ?? translations.get(title.key))
      .filter(Boolean);
    const opens = parts
      .flatMap((part) => part.paths)
      .map((candidate) => matchRoute(candidate, routes))
      .filter((route) => route && route !== screen.route && nameByRoute.has(route));
    facts.set(screen.route, { titles: unique(titles), opens: unique(opens) });
  }
  const openedFrom = (route) => screens.filter((screen) => facts.get(screen.route).opens.includes(route));

  const out = [
    '# UniMe app reference',
    '',
    'Every screen of the UniMe app: what it is for, its title and the screens it leads to.',
    '',
    `This file is generated by ${code('scripts/generate-app-reference.mjs')} from the routes in ${code('unime/src/routes')}, the Svelte components of each screen and the English translations in ${code('unime/src/i18n/en/index.ts')}. Do not edit it by hand. Change the code, or the names and purposes in ${code('scripts/app-reference-screens.json')}, and run ${code('pnpm docs:app-reference')}.`,
    '',
    '## How to read an entry',
    '',
    `- The line under the name gives the route of the screen and the file that implements it. A part in square brackets, as in ${code('/credentials/[id]')}, is a parameter.`,
    '- The paragraph below it says what the screen is for. It is written by hand.',
    '- **Title** is the English title in the bar at the top of the screen, for a screen that sets one.',
    '- **Opens** lists the screens whose route the code of this screen names, and **Opened from** lists the screens that name this one. A route in the code is nearly always the target of a link or a redirect, but the lists also include a route that the code only compares with. A screen that the backend opens, such as a prompt, says so in its purpose.',
    '- The buttons and texts of a screen are not listed. The file of the screen is the place to look them up.',
    '',
    '## Screens',
    '',
    ...config.groups.map(
      (group) => `- ${group.title}: ${group.screens.map((screen) => link(screen.route)).join(', ')}`,
    ),
    '',
  ];
  for (const group of config.groups) {
    out.push(`## ${group.title}`, '', group.description, '');
    for (const screen of group.screens) {
      const { titles, opens } = facts.get(screen.route);
      const sources = openedFrom(screen.route).map((source) => link(source.route));
      out.push(
        `### ${screen.name}`,
        '',
        `${code(screen.route)} in ${code(repoPath(pageByRoute.get(screen.route)))}`,
        '',
        screen.purpose,
        '',
      );
      if (titles.length > 0)
        out.push(`- **Title:** ${titles.map((title) => `"${escapeMarkdown(title)}"`).join(' or ')}`);
      if (opens.length > 0) out.push(`- **Opens:** ${opens.map(link).join(', ')}`);
      if (sources.length > 0) out.push(`- **Opened from:** ${sources.join(', ')}`);
      if (titles.length + opens.length + sources.length > 0) out.push('');
    }
  }
  return out.join('\n').replace(/\n+$/, '\n');
}

const reference = buildReference();
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
