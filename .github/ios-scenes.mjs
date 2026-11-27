// Checks the scene life cycle on a booted simulator through Appium's XCUITest driver, reading the
// debug-only #scene-probe (packages/mobile/src/scene_probe.rs) off the page: the window shows, the
// key-window lookup the share sheet and alerts use finds it, one scene, `counted://` links reach
// Rust warm and cold, and coming back to the app fires `Resumed`. docs/plans/ios-uiscene.md §3.3.
// Usage: node ios-scenes.mjs <udid> <iPhone|iPad> [out dir]; an Appium server on 127.0.0.1:4723.
import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';

const [udid, kind, out = 'scenes-out'] = process.argv.slice(2);
if (!udid || !['iPhone', 'iPad'].includes(kind)) {
  console.error('usage: node ios-scenes.mjs <udid> <iPhone|iPad> [out dir]');
  process.exit(2);
}
mkdirSync(out, { recursive: true });

const base = 'http://127.0.0.1:4723';
const ELEMENT = 'element-6066-11e4-a52e-4f735466cecf';
const BUNDLE = 'fr.counted.app';
const LINK = 'counted://add';
let session;
let step = 'session';
const lane = process.env.LANE || udid;
const annotate = (title, message) =>
  console.log(
    `::error title=${`${lane}: ${title}`.replace(/[%\r\n:,]/g, encodeURIComponent)}::${String(message).replace(/[%\r\n]/g, encodeURIComponent)}`,
  );
setTimeout(() => {
  annotate('scenes', `no result after 10 min, stuck at ${step}`);
  writeFileSync(`${out}/report.json`, JSON.stringify(report, null, 2));
  process.exit(1);
}, 10 * 60000).unref();

async function call(method, path, body) {
  const res = await fetch(base + path, {
    method,
    headers: { 'content-type': 'application/json' },
    body: body && JSON.stringify(body),
    signal: AbortSignal.timeout(path === '/session' ? 360000 : 120000),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(`${method} ${path}: ${JSON.stringify(json.value)}`);
  return json.value;
}
const s = (method, path, body) => call(method, `/session/${session}${path}`, body);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));
const mobile = (script, args = {}) => s('POST', '/execute/sync', { script: `mobile: ${script}`, args: [args] });
const openUrl = (url) => execFileSync('xcrun', ['simctl', 'openurl', udid, url], { timeout: 60000 });

async function element(label, type, timeout = 60000) {
  const named = `(label == "${label}" OR name == "${label}")`;
  const value = type ? `type == "XCUIElementType${type}" AND ${named}` : named;
  const end = Date.now() + timeout;
  for (;;) {
    try {
      return (await s('POST', '/element', { using: '-ios predicate string', value }))[ELEMENT];
    } catch (e) {
      if (Date.now() > end) throw e;
      await sleep(500);
    }
  }
}
async function shot(name) {
  writeFileSync(`${out}/${name}.png`, Buffer.from(await s('GET', '/screenshot'), 'base64'));
}

async function dump(title) {
  try {
    const contexts = await s('GET', '/contexts');
    const source = await s('GET', '/source');
    const tree = [...source.matchAll(/<XCUIElementType(\w+)([^>]*)>/g)].map(([, type, attrs]) => {
      const label = /\blabel="([^"]*)"/.exec(attrs) || /\bname="([^"]*)"/.exec(attrs);
      return label ? `${type}:${label[1]}` : type;
    });
    annotate(`${title} screen`, `contexts ${JSON.stringify(contexts)}\n${tree.join('\n').slice(0, 6000) || source.slice(0, 6000)}`);
  } catch (e) {
    annotate(`${title} screen`, `no source: ${e}`);
  }
}

async function probe() {
  const contexts = await s('GET', '/contexts');
  const web = contexts.find((c) => c.startsWith('WEBVIEW'));
  if (!web) return null;
  await s('POST', '/context', { name: web });
  try {
    return await s('POST', '/execute/sync', {
      script: "const p = document.getElementById('scene-probe'); return p && { ...p.dataset };",
      args: [],
    });
  } finally {
    await s('POST', '/context', { name: 'NATIVE_APP' });
  }
}

async function until(what, test, timeout = 30000) {
  const end = Date.now() + timeout;
  let last;
  for (;;) {
    last = await probe().catch((e) => ({ error: String(e) }));
    if (last && !last.error && test(last)) return last;
    if (Date.now() > end) throw new Error(`${what}: probe stayed ${JSON.stringify(last)}`);
    await sleep(1000);
  }
}

const report = [];
let failed = false;
let dumped = false;
async function check(name, run) {
  step = name;
  try {
    const detail = await run();
    report.push({ name, ok: true, detail });
    console.log(`ok   ${name} ${JSON.stringify(detail)}`);
  } catch (e) {
    failed = true;
    report.push({ name, ok: false, error: String(e) });
    console.log(`FAIL ${name}: ${e}`);
    annotate(name, e);
    if (!dumped) {
      dumped = true;
      await dump(name);
    }
  }
  await shot(name.replace(/\W+/g, '-')).catch(() => {});
}

try {
  session = (
    await call('POST', '/session', {
      capabilities: {
        alwaysMatch: {
          platformName: 'iOS',
          'appium:automationName': 'XCUITest',
          'appium:udid': udid,
          'appium:isHeadless': true,
          'appium:simulatorStartupTimeout': 300000,
          'appium:bundleId': BUNDLE,
          'appium:language': 'en',
          'appium:locale': 'en_US',
          'appium:newCommandTimeout': 600,
          'appium:webviewConnectTimeout': 30000,
          'appium:usePrebuiltWDA': true,
          'appium:derivedDataPath': process.env.WDA_DERIVED_DATA,
          'appium:wdaLaunchTimeout': 240000,
          'appium:showXcodeLog': true,
        },
      },
    })
  ).sessionId;

  await check('launch shows the window in one scene', async () => {
    await element('Create a project', 'Button');
    return until('key window in one scene', (p) => p.keyWindow === 'true' && p.scenes === '1');
  });

  await check('warm link reaches Rust', async () => {
    const before = Number((await until('probe', () => true)).opened);
    openUrl(LINK);
    return until('Opened after a warm link', (p) => Number(p.opened) >= before + 1);
  });

  await check('returning to the app fires Resumed', async () => {
    const before = Number((await until('probe', () => true)).resumed);
    await mobile('backgroundApp', { seconds: 3 });
    return until('Resumed after background', (p) => Number(p.resumed) >= before + 1);
  });

  if (kind === 'iPad') {
    await check('rotation keeps one scene and its key window', async () => {
      await s('POST', '/orientation', { orientation: 'LANDSCAPE' });
      await sleep(2000);
      const turned = await until('key window after rotation', (p) => p.keyWindow === 'true' && p.scenes === '1');
      await s('POST', '/orientation', { orientation: 'PORTRAIT' });
      return turned;
    });
  }

  await check('cold link reaches Rust', async () => {
    await mobile('terminateApp', { bundleId: BUNDLE });
    await sleep(2000);
    openUrl(LINK);
    return until('Opened after a cold link', (p) => Number(p.opened) >= 1 && p.keyWindow === 'true', 90000);
  });
} catch (e) {
  failed = true;
  annotate('scenes', `${step}: ${e}${e?.cause ? ` (cause: ${e.cause.code || e.cause})` : ''}`);
} finally {
  writeFileSync(`${out}/report.json`, JSON.stringify(report, null, 2));
  if (session) await s('DELETE', '').catch(() => {});
}
process.exit(failed ? 1 : 0);
