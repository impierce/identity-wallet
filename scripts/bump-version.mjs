#!/usr/bin/env node
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');

const versionTargets = [
  { path: 'Cargo.toml', occurrences: 1 },
  { path: 'identity-wallet/Cargo.toml', occurrences: 1 },
  { path: 'unime/package.json', occurrences: 1 },
  { path: 'unime/src-tauri/Cargo.toml', occurrences: 1 },
  { path: 'unime/src-tauri/tauri.conf.json', occurrences: 1 },
  { path: 'unime/src-tauri/gen-static/android/app/tauri.properties', occurrences: 1 },
  { path: 'unime/src-tauri/gen-static/apple/unime_iOS/Info.plist', occurrences: 2 },
  { path: 'unime/src/routes/(app)/me/settings/about/+page.svelte', occurrences: 1 },
  { path: 'Cargo.lock', packages: ['identity-wallet', 'unime'] },
];

/** Total version occurrences the targets above account for. */
const locationCount = versionTargets.reduce(
  (total, target) => total + (target.packages?.length ?? target.occurrences),
  0,
);

const escapeForRegExp = (value) => value.replaceAll(/[.*+?^${}()|[\]\\]/g, '\\$&');

const usage = `Usage: pnpm version:bump <new-version> [--dry-run]

Example:
  pnpm version:bump 0.17.0
  pnpm version:bump 0.17.0 --dry-run`;

const arguments_ = process.argv.slice(2);
const dryRunIndex = arguments_.indexOf('--dry-run');
const dryRun = dryRunIndex !== -1;

if (dryRun) {
  arguments_.splice(dryRunIndex, 1);
}

if (arguments_.includes('--help') || arguments_.includes('-h')) {
  console.log(usage);
  process.exit(0);
}

if (arguments_.length !== 1) {
  console.error(usage);
  process.exit(1);
}

const [newVersion] = arguments_;

// Both app stores expect a period-separated release version consisting of numeric components.
if (!/^\d+\.\d+\.\d+$/.test(newVersion)) {
  console.error(`Invalid version "${newVersion}". Expected a version in the form X.Y.Z.`);
  process.exit(1);
}

const rootCargoPath = resolve(repositoryRoot, 'Cargo.toml');
const rootCargo = await readFile(rootCargoPath, 'utf8');
const currentVersionMatch = rootCargo.match(/^\[workspace\.package\]\s*$[\s\S]*?^version\s*=\s*"([^"]+)"\s*$/m);

if (!currentVersionMatch) {
  console.error('Could not determine the current version from [workspace.package] in Cargo.toml.');
  process.exit(1);
}

const currentVersion = currentVersionMatch[1];

if (currentVersion === newVersion) {
  console.error(`The repository is already at version ${newVersion}.`);
  process.exit(1);
}

const updates = [];

const abort = (message) => {
  console.error(message);
  console.error('No files were changed. Update the target list or version locations before retrying.');
  process.exit(1);
};

// Validate every target before writing any file, avoiding a partial version bump.
for (const target of versionTargets) {
  const absolutePath = resolve(repositoryRoot, target.path);
  const contents = await readFile(absolutePath, 'utf8');

  if (target.packages) {
    let updated = contents;

    for (const packageName of target.packages) {
      // Anchored on the `name`/`version` pair so only the workspace member's own entry moves.
      const entry = new RegExp(
        `(^name = "${escapeForRegExp(packageName)}"\nversion = ")${escapeForRegExp(currentVersion)}(")$`,
        'm',
      );

      if (!entry.test(updated)) {
        abort(`${target.path}: no \`${packageName}\` package entry at version ${currentVersion}.`);
      }

      updated = updated.replace(entry, `$1${newVersion}$2`);
    }

    updates.push({ absolutePath, contents: updated, path: target.path });
    continue;
  }

  const actualOccurrences = contents.split(currentVersion).length - 1;

  if (actualOccurrences !== target.occurrences) {
    abort(
      `${target.path}: expected ${target.occurrences} occurrence(s) of ${currentVersion}, found ${actualOccurrences}.`,
    );
  }

  updates.push({
    absolutePath,
    contents: contents.replaceAll(currentVersion, newVersion),
    path: target.path,
  });
}

if (dryRun) {
  console.log(`Dry run: would replace ${currentVersion} with ${newVersion} in ${locationCount} locations:`);
} else {
  for (const update of updates) {
    await writeFile(update.absolutePath, update.contents);
  }

  console.log(`Replaced ${currentVersion} with ${newVersion} in ${locationCount} locations:`);
}

for (const update of updates) {
  console.log(`- ${update.path}`);
}

if (!dryRun) {
  console.log(
    '\nReview the diff. `Cargo.lock` is bumped here too, so the commit is complete without a Cargo run first.',
  );
}
