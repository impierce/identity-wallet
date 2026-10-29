#!/usr/bin/env node
// Ends the dev processes started from this checkout: the Tauri CLI, Vite, esbuild, cargo and rustc, and the app.
//
// Usage: pnpm dev:stop
//        node scripts/dev-stop.mjs --after-exit-of <pid> --started-at <epoch ms>
//
// `pnpm dev:instance` starts the second form as a hidden watchdog. Windows does not end child processes together with
// their parent, so when only the launcher is killed (a closed shell or agent task), the watchdog stops what is left.
// It waits for the launcher, or the `pnpm dev:instance` wrappers around it, to exit and then stops the processes of this
// checkout that were started before that moment, leaving an instance started afterwards alone.
import { execFileSync } from 'node:child_process';
import { basename, dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const isWindows = process.platform === 'win32';

const normalize = (text) => (text ?? '').replaceAll('\\', '/').toLowerCase();
const rootPrefix = `${normalize(root).replace(/\/+$/, '')}/`;

const listProcesses = () => {
  if (isWindows) {
    const script =
      'Get-CimInstance Win32_Process | ForEach-Object { [pscustomobject]@{ pid = $_.ProcessId; ppid = $_.ParentProcessId; ' +
      'name = $_.Name; path = $_.ExecutablePath; command = $_.CommandLine; ' +
      "started = if ($_.CreationDate) { $_.CreationDate.ToUniversalTime().ToString('o') } } } | ConvertTo-Json -Compress";
    const output = execFileSync('powershell.exe', ['-NoProfile', '-NonInteractive', '-Command', script], {
      encoding: 'utf8',
      maxBuffer: 64 * 1024 * 1024,
      windowsHide: true,
    });
    return JSON.parse(output).map((entry) => ({ ...entry, started: entry.started && new Date(entry.started) }));
  }
  // `lstart` is a fixed-width date such as "Thu Oct 29 11:54:52 2026".
  const output = execFileSync('ps', ['-A', '-o', 'pid=', '-o', 'ppid=', '-o', 'lstart=', '-o', 'args='], {
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  return output
    .split('\n')
    .map((line) => line.match(/^\s*(\d+)\s+(\d+)\s+(\w{3} \w{3}\s+\d+ [\d:]+ \d{4})\s+(.*)$/))
    .filter(Boolean)
    .map(([, pid, ppid, started, command]) => {
      const [, weekday, month, day, time, year] = started.match(/(\w+) (\w+)\s+(\d+) ([\d:]+) (\d+)/);
      // `args` does not separate the executable from its arguments, so a path into this checkout may contain spaces.
      const path = normalize(command).startsWith(rootPrefix) ? command : command.split(' ')[0];
      return {
        pid: Number(pid),
        ppid: Number(ppid),
        name: basename(command.split(' ')[0]),
        path,
        command,
        started: new Date(`${weekday} ${month} ${day} ${year} ${time}`),
      };
    });
};

const isRunning = (pid) => {
  try {
    process.kill(pid, 0);
    return true;
  } catch (error) {
    return error.code === 'EPERM';
  }
};

// The processes of this checkout run its binaries (esbuild, the app, build scripts) or are a node, cargo or rustc
// process with a path into this checkout on the command line. Their wrappers (pnpm, cmd, rustup, cargo run) are
// included as descendants or exit by themselves once these are gone.
const isFromCheckout = ({ name, path, command }) =>
  normalize(path).startsWith(rootPrefix) ||
  (/^(node|cargo|rustc)(\.exe)?$/i.test(name) && normalize(command).includes(rootPrefix));

// Without `parentPid`, the processes of this checkout and their descendants; with it, the descendants of that process.
const findProcesses = (processes, { parentPid, startedAfter, startedBefore } = {}) => {
  const byPid = new Map(processes.map((entry) => [entry.pid, entry]));
  const spared = new Set();
  for (let pid = process.pid; byPid.has(pid) && !spared.has(pid); pid = byPid.get(pid).ppid) spared.add(pid);

  const wanted = (candidate) =>
    !spared.has(candidate.pid) && !(candidate.started < startedAfter) && !(candidate.started > startedBefore);
  const isRoot = (candidate) => (parentPid ? candidate.ppid === parentPid : isFromCheckout(candidate));
  const found = new Map(processes.filter((candidate) => wanted(candidate) && isRoot(candidate)).map((p) => [p.pid, p]));
  for (let added = true; added;) {
    added = false;
    for (const candidate of processes) {
      const parent = found.get(candidate.ppid);
      // A started time before the parent's means the parent id was reused.
      if (parent && !found.has(candidate.pid) && wanted(candidate) && !(candidate.started < parent.started)) {
        found.set(candidate.pid, candidate);
        added = true;
      }
    }
  }
  return [...found.values()];
};

const stopProcesses = (targets) => {
  if (targets.length === 0) return;
  if (isWindows) {
    try {
      execFileSync('taskkill', ['/F', ...targets.flatMap(({ pid }) => ['/PID', String(pid)])], {
        stdio: 'ignore',
        windowsHide: true,
      });
    } catch {
      // Some targets already exited together with their parent.
    }
    return;
  }
  for (const { pid } of targets) {
    try {
      process.kill(pid, 'SIGTERM');
    } catch {
      // Already exited.
    }
  }
};

export const stopProcessTree = (parentPid) => stopProcesses(findProcesses(listProcesses(), { parentPid }));

const watchUntilExit = async (launcherPid, launcherStarted) => {
  const byPid = new Map(listProcesses().map((entry) => [entry.pid, entry]));
  // The launcher and the `pnpm dev:instance` wrappers around it (pnpm, cmd, sh), but not the terminal shell. If the
  // launcher is already gone, the chain is just its id, which is no longer running.
  const chain = [launcherPid];
  for (let parent = byPid.get(byPid.get(launcherPid)?.ppid); parent; parent = byPid.get(parent.ppid)) {
    if (chain.includes(parent.pid) || !/dev[:-]instance/.test(parent.command ?? '')) break;
    chain.push(parent.pid);
  }
  while (chain.every(isRunning)) await new Promise((wake) => setTimeout(wake, 1000));
  // The instance's processes may have lost their parents, so they are found by checkout and start time instead.
  stopProcesses(findProcesses(listProcesses(), { startedAfter: launcherStarted, startedBefore: new Date() }));
};

if (process.argv[1] && normalize(resolve(process.argv[1])) === normalize(fileURLToPath(import.meta.url))) {
  const { values } = parseArgs({ options: { 'after-exit-of': { type: 'string' }, 'started-at': { type: 'string' } } });
  if (values['after-exit-of']) {
    await watchUntilExit(Number(values['after-exit-of']), new Date(Number(values['started-at'] ?? Date.now())));
  } else {
    const stopped = findProcesses(listProcesses());
    stopProcesses(stopped);
    for (const { pid, name } of stopped) console.log(`Stopped ${name} (${pid})`);
    console.log(stopped.length ? `Stopped ${stopped.length} processes of ${root}` : `Nothing running from ${root}`);
  }
}
