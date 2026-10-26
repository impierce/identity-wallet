#!/usr/bin/env node
// Creates a separate checkout (git worktree) for a branch, next to this repository, ready for `pnpm dev:instance`.
//
// Usage: pnpm checkout:branch <branch> [--from <base>] [--dir <path>]
//
// An existing local or remote branch is checked out as is; otherwise a new branch is created from `--from`
// (default: origin/dev). The default directory is `../identity-wallet-<branch>` with `/` replaced by `-`.
import { execFileSync } from 'node:child_process';
import { copyFileSync, existsSync } from 'node:fs';
import { basename, dirname, join, resolve } from 'node:path';
import { parseArgs } from 'node:util';

const { values, positionals } = parseArgs({
  allowPositionals: true,
  options: {
    from: { type: 'string', default: 'origin/dev' },
    dir: { type: 'string' },
  },
});

const branch = positionals[0];
if (!branch) {
  console.error('Usage: pnpm checkout:branch <branch> [--from <base>] [--dir <path>]');
  process.exit(1);
}

const isWindows = process.platform === 'win32';
const run = (cmd, args, cwd) => execFileSync(cmd, args, { cwd, stdio: 'inherit', shell: isWindows && cmd === 'pnpm' });
const git = (args, cwd = root) => execFileSync('git', args, { cwd, encoding: 'utf8' }).trim();
const gitSucceeds = (args) => {
  try {
    git(args);
    return true;
  } catch {
    return false;
  }
};

const root = execFileSync('git', ['rev-parse', '--show-toplevel'], { encoding: 'utf8' }).trim();
const dir = resolve(values.dir ?? join(dirname(root), `${basename(root)}-${branch.replaceAll('/', '-')}`));
if (existsSync(dir)) {
  console.error(`${dir} already exists.`);
  process.exit(1);
}

run('git', ['fetch', 'origin'], root);
if (gitSucceeds(['rev-parse', '--verify', '--quiet', `refs/heads/${branch}`])) {
  run('git', ['worktree', 'add', dir, branch], root);
} else if (gitSucceeds(['rev-parse', '--verify', '--quiet', `refs/remotes/origin/${branch}`])) {
  run('git', ['worktree', 'add', '--track', '-b', branch, dir, `origin/${branch}`], root);
} else {
  run('git', ['worktree', 'add', '--no-track', '-b', branch, dir, values.from], root);
}

for (const envFile of ['unime/.env', 'identity-wallet/.env']) {
  const source = existsSync(join(root, envFile)) ? join(root, envFile) : join(root, `${envFile}.example`);
  copyFileSync(source, join(dir, envFile));
}

run('pnpm', ['install'], dir);
run('pnpm', ['--filter', 'unime', 'tauri', 'icon', 'src-tauri/app-icon.png'], dir);
run('git', ['checkout', '--', 'unime/src-tauri/icons'], dir);

console.log(`\nCheckout ready: ${dir}\nStart it with: cd "${dir}" && pnpm dev:instance`);
