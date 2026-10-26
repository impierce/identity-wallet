#!/usr/bin/env node
// Runs this checkout as a desktop app that can run next to UniMe instances from other checkouts.
//
// Usage: pnpm dev:instance [--name <name>] [--port <vite port>] [--mcp-port <bridge port>]
//
// Each instance gets its own app identifier (and with it its own data directory and single-instance lock), its own
// Vite port and its own MCP bridge port. The name defaults to the checkout folder name without the
// `identity-wallet-` prefix; the main `identity-wallet` checkout keeps the default identifier and data.
import { spawn } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { createServer } from 'node:net';
import { basename, dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseArgs } from 'node:util';

const { values } = parseArgs({
  options: {
    name: { type: 'string' },
    port: { type: 'string' },
    'mcp-port': { type: 'string' },
  },
});

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const tauriDir = join(root, 'unime', 'src-tauri');
const baseConfig = JSON.parse(readFileSync(join(tauriDir, 'tauri.conf.json'), 'utf8'));

const name = (values.name ?? basename(root).replace(/^identity-wallet-?/, ''))
  .toLowerCase()
  .replace(/[^a-z0-9-]+/g, '-')
  .replace(/^-+|-+$/g, '');

const canListen = (port, host) =>
  new Promise((resolve) => {
    const server = createServer();
    server.once('error', (error) => resolve(error.code === 'EADDRNOTAVAIL' || error.code === 'EAFNOSUPPORT'));
    server.listen({ port, host, exclusive: true }, () => server.close(() => resolve(true)));
  });

const isFree = async (port) => {
  for (const host of ['127.0.0.1', '::1', '0.0.0.0', '::']) {
    if (!(await canListen(port, host))) return false;
  }
  return true;
};

const findFreePort = async (start, exclude = []) => {
  for (let port = start; port < start + 100; port++) {
    if (!exclude.includes(port) && (await isFree(port))) return port;
  }
  throw new Error(`No free port found in ${start}-${start + 99}`);
};

const devPort = values.port ? Number(values.port) : await findFreePort(4173);
const mcpPort = values['mcp-port'] ? Number(values['mcp-port']) : await findFreePort(9223, [devPort]);
const identifier = name ? `${baseConfig.identifier}.${name}` : baseConfig.identifier;

const instanceConfig = {
  identifier,
  app: {
    withGlobalTauri: true,
    windows: baseConfig.app.windows.map((window) => ({
      ...window,
      title: name ? `${window.title} (${name})` : window.title,
    })),
  },
  build: { devUrl: `http://localhost:${devPort}` },
};
const instanceConfigFile = join(tauriDir, 'tauri.instance.conf.json');
writeFileSync(instanceConfigFile, `${JSON.stringify(instanceConfig, null, 2)}\n`);

console.log(`UniMe instance "${name || 'default'}"
  identifier: ${identifier}
  dev server: http://localhost:${devPort}
  MCP bridge: 127.0.0.1:${mcpPort} or the next free port; the bound port is reported once the app is up
`);

const child = spawn('pnpm --filter unime tauri dev --config src-tauri/tauri.instance.conf.json', {
  cwd: root,
  stdio: ['inherit', 'pipe', 'pipe'],
  shell: true,
  env: {
    ...process.env,
    UNIME_DEV_PORT: String(devPort),
    UNIME_MCP_PORT: String(mcpPort),
    CARGO_TERM_COLOR: 'always',
    FORCE_COLOR: '1',
  },
});

const forwardAndWatch = (input, output) => {
  let pending = '';
  input.on('data', (chunk) => {
    output.write(chunk);
    const lines = (pending + chunk.toString()).split('\n');
    pending = lines.pop();
    for (const line of lines) {
      const bound = line.match(/WebSocket server listening on: (\S+)/);
      if (bound) console.log(`\nUniMe instance "${name || 'default'}" MCP bridge bound to ${bound[1]}\n`);
    }
  });
};
forwardAndWatch(child.stdout, process.stdout);
forwardAndWatch(child.stderr, process.stderr);
child.on('close', (code) => {
  process.exitCode = code ?? 1;
});
