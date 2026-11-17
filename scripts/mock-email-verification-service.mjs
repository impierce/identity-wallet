#!/usr/bin/env node
// A mock of the email verification service for testing UniMe's email verification page locally. It answers the
// requests UniMe sends, sends no email and keeps its cases in memory until the process ends.
//
// Usage: pnpm mock:email-verification [--port 5180] [--lifetime 60] [--skew 0]
//
// Point a UniMe build at it with `EMAIL_VERIFICATION_SERVICE_HOST=http://localhost:5180` in the environment (or in
// `identity-wallet/.env`), then start the app.
//
// Control API (JSON in and out):
//   GET  /__mock              the configuration and all cases
//   POST /__mock/config       change the configuration, e.g. { "lifetimeSecs": 300, "clockSkewSecs": -90 }:
//                               lifetimeSecs    lifetime of a new case
//                               clockSkewSecs   how far this mock's clock is ahead (+) or behind (-) the real one
//                               code            the code that redeems a case (default `1234`)
//                               delayMs         delay before every response
//                               offer           what a redeemed code returns (default: an offer URI that UniMe cannot
//                                               resolve)
//   POST /__mock/fail         answer the next request(s) to a route with an error instead:
//                               { "route": "create" | "redeem" | "healthz", "status": 500, "body": { "error": "…" },
//                                 "times": 1 }
//   POST /__mock/cases/:id    change when a case expires: { "expiresAt": "<ISO date>" } or { "expiresInSecs": 5 }
//                             (relative to this mock's clock)
//   POST /__mock/reset        drop all cases and failures and restore the configuration given on the command line
import { randomUUID } from 'node:crypto';
import { createServer } from 'node:http';
import { parseArgs } from 'node:util';

const { values: args } = parseArgs({
  options: {
    port: { type: 'string', default: '5180' },
    lifetime: { type: 'string', default: '60' },
    skew: { type: 'string', default: '0' },
  },
});

const defaults = () => ({
  lifetimeSecs: Number(args.lifetime),
  clockSkewSecs: Number(args.skew),
  code: '1234',
  delayMs: 0,
  offer: null,
});

let config = defaults();
/** @type {Map<string, { id: string, expiresAt: Date }>} */
let cases = new Map();
/** @type {Map<string, { status: number, body?: unknown, times: number }>} */
let failures = new Map();

// This mock's clock, which runs `clockSkewSecs` ahead of (or behind) the real one.
const now = () => new Date(Date.now() + config.clockSkewSecs * 1000);

const log = (message) => console.log(`[mock-email-verification] ${new Date().toISOString().slice(11, 19)} ${message}`);

const send = (res, status, body) => {
  if (body === undefined) {
    res.writeHead(status).end();
  } else if (typeof body === 'string') {
    res.writeHead(status, { 'Content-Type': 'text/plain; charset=utf-8' }).end(body);
  } else {
    res.writeHead(status, { 'Content-Type': 'application/json' }).end(JSON.stringify(body));
  }
};

const readJson = async (req) => {
  let text = '';
  for await (const chunk of req) text += chunk;
  try {
    const value = JSON.parse(text || '{}');
    return value && typeof value === 'object' && !Array.isArray(value) ? value : undefined;
  } catch {
    return undefined;
  }
};

const takeFailure = (route) => {
  const failure = failures.get(route);
  if (!failure) return undefined;
  if (--failure.times <= 0) failures.delete(route);
  return failure;
};

const expiresInSecs = (verification) => Math.round((verification.expiresAt.getTime() - now().getTime()) / 1000);

const describe = (verification) => ({ ...verification, expiresInSecs: expiresInSecs(verification) });

async function handleService(req, res, path) {
  const route = path === '/healthz' ? 'healthz' : path === '/api/verify' ? 'create' : 'redeem';

  if (config.delayMs > 0) await new Promise((resolve) => setTimeout(resolve, config.delayMs));

  const failure = takeFailure(route);
  if (failure) {
    log(`${req.method} ${path} → ${failure.status} (injected failure)`);
    return send(res, failure.status, failure.body);
  }

  if (route === 'healthz') return send(res, 200, 'OK');

  if (route === 'create') {
    const verification = { id: randomUUID(), expiresAt: new Date(now().getTime() + config.lifetimeSecs * 1000) };
    cases.set(verification.id, verification);
    log(`POST /api/verify → 200 case ${verification.id}, expires in ${expiresInSecs(verification)} s`);
    return send(res, 200, {
      id: verification.id,
      expires_at: verification.expiresAt.toISOString(),
      validation_expiration_in_secs: config.lifetimeSecs,
    });
  }

  const id = path.slice('/api/verify/'.length);
  const verification = cases.get(id);
  if (!verification) {
    log(`${req.method} ${path} → 404`);
    return send(res, 404, { error: 'Unknown case (mock)' });
  }
  // Read the body first, so that a case that expires while it arrives is refused.
  const body = await readJson(req);
  if (expiresInSecs(verification) <= 0) {
    log(`POST ${path} → 400 (expired)`);
    return send(res, 400, { error: 'Case expired (mock)' });
  }
  if (body?.code !== config.code) {
    log(`POST ${path} → 400 (wrong code)`);
    return send(res, 400, { error: 'Wrong code (mock)' });
  }
  cases.delete(id);
  log(`POST ${path} → 200 (code accepted)`);
  const offerUri = `http://localhost:${args.port}/offers/${id}`;
  return send(
    res,
    200,
    config.offer ?? `openid-credential-offer://?credential_offer_uri=${encodeURIComponent(offerUri)}`,
  );
}

async function handleControl(req, res, path) {
  if (req.method === 'GET' && path === '/__mock') {
    return send(res, 200, { config, now: now().toISOString(), cases: [...cases.values()].map(describe) });
  }
  if (req.method !== 'POST') return send(res, 405, { error: 'Use GET /__mock or POST' });

  const body = await readJson(req);
  if (!body) return send(res, 400, { error: 'Send a JSON object' });

  if (path === '/__mock/config') {
    const unknown = Object.keys(body).filter((key) => !(key in config));
    if (unknown.length) return send(res, 400, { error: `Unknown settings: ${unknown.join(', ')}` });
    config = { ...config, ...body };
    log(`config ${JSON.stringify(body)}`);
    return send(res, 200, config);
  }
  if (path === '/__mock/fail') {
    if (!['create', 'redeem', 'healthz'].includes(body.route) || !Number.isInteger(body.status)) {
      return send(res, 400, { error: 'Send { route: create | redeem | healthz, status, body?, times? }' });
    }
    failures.set(body.route, { status: body.status, body: body.body, times: body.times ?? 1 });
    log(`next ${body.times ?? 1} ${body.route} request(s) → ${body.status}`);
    return send(res, 200, Object.fromEntries(failures));
  }
  if (path === '/__mock/reset') {
    config = defaults();
    cases = new Map();
    failures = new Map();
    log('reset');
    return send(res, 200, config);
  }
  if (path.startsWith('/__mock/cases/')) {
    const verification = cases.get(path.slice('/__mock/cases/'.length));
    if (!verification) return send(res, 404, { error: 'No such case' });
    if (typeof body.expiresAt === 'string') verification.expiresAt = new Date(body.expiresAt);
    if (typeof body.expiresInSecs === 'number') {
      verification.expiresAt = new Date(now().getTime() + body.expiresInSecs * 1000);
    }
    log(`case ${verification.id} changed: ${JSON.stringify(body)}`);
    return send(res, 200, describe(verification));
  }
  return send(res, 404, { error: 'Unknown control route' });
}

createServer((req, res) => {
  const path = new URL(req.url ?? '/', 'http://localhost').pathname.replace(/\/+$/, '') || '/';
  const control = path === '/__mock' || path.startsWith('/__mock/');
  const known =
    control ||
    (req.method === 'GET' && path === '/healthz') ||
    (req.method === 'POST' && (path === '/api/verify' || /^\/api\/verify\/[^/]+$/.test(path)));
  if (!known) return send(res, 404);
  (control ? handleControl : handleService)(req, res, path).catch((error) => {
    log(`${req.method} ${path} failed: ${error}`);
    if (!res.headersSent) send(res, 500, { error: 'Mock service error' });
  });
}).listen(Number(args.port), '127.0.0.1', () => {
  log(
    `listening on http://localhost:${args.port} (case lifetime ${config.lifetimeSecs} s, clock skew ${config.clockSkewSecs} s)`,
  );
});
