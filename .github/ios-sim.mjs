// Drives the create-project sheet on a booted iPhone simulator through Appium's XCUITest driver
// (real touches through WebDriverAgent) and reads visualViewport.offsetTop from the page after
// each tap. WebKit's focus reveal pans the visual viewport; the owned tap in
// packages/ui/src/common/keyboard.rs must keep it at 0. docs/e2e.md "iOS" in the private repo.
// Usage: node ios-sim.mjs <udid> [out dir]; an Appium server must listen on 127.0.0.1:4723.
import { mkdirSync, writeFileSync } from 'node:fs';

const [udid, out = 'sim-out'] = process.argv.slice(2);
if (!udid) {
  console.error('usage: node ios-sim.mjs <udid> [out dir]');
  process.exit(2);
}
mkdirSync(out, { recursive: true });

const base = 'http://127.0.0.1:4723';
const ELEMENT = 'element-6066-11e4-a52e-4f735466cecf';
const FIELDS = [
  ['Project name', 'add-project-name'],
  ['Your name in this project', 'add-project-me'],
  ['Participant name', 'add-project-user'],
];
let session;

async function call(method, path, body) {
  const res = await fetch(base + path, {
    method,
    headers: { 'content-type': 'application/json' },
    body: body && JSON.stringify(body),
  });
  const json = await res.json();
  if (!res.ok) throw new Error(`${method} ${path}: ${JSON.stringify(json.value)}`);
  return json.value;
}
const s = (method, path, body) => call(method, `/session/${session}${path}`, body);
const sleep = (ms) => new Promise((r) => setTimeout(r, ms));

async function element(label, type, timeout = 30000) {
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
const tap = async (label, type) => s('POST', `/element/${await element(label, type)}/click`);
async function shot(name) {
  writeFileSync(`${out}/${name}.png`, Buffer.from(await s('GET', '/screenshot'), 'base64'));
}

async function inPage(script) {
  const contexts = await s('GET', '/contexts');
  const web = contexts.find((c) => c.startsWith('WEBVIEW'));
  if (!web) throw new Error(`no webview context in ${contexts}`);
  await s('POST', '/context', { name: web });
  try {
    return await s('POST', '/execute/sync', { script, args: [] });
  } finally {
    await s('POST', '/context', { name: 'NATIVE_APP' });
  }
}

const report = [];
let failed = false;
try {
  session = (
    await call('POST', '/session', {
      capabilities: {
        alwaysMatch: {
          platformName: 'iOS',
          'appium:automationName': 'XCUITest',
          'appium:udid': udid,
          'appium:bundleId': 'fr.counted.app',
          'appium:language': 'en',
          'appium:locale': 'en_US',
          'appium:newCommandTimeout': 600,
          'appium:webviewConnectTimeout': 30000,
        },
      },
    })
  ).sessionId;

  await tap('Create a project', 'Button');
  await element('New project');
  await sleep(1000);
  await shot('sheet');

  for (const [label, id] of FIELDS) {
    await tap(label, 'TextField');
    await sleep(1500);
    await shot(id);
    const page = await inPage(
      'return { offsetTop: visualViewport.offsetTop, height: visualViewport.height, inner: innerHeight, focused: document.activeElement && document.activeElement.id }',
    );
    const keyboard = await s('GET', '/appium/device/is_keyboard_shown');
    const ok = page.offsetTop === 0 && keyboard === true && page.focused === id;
    failed ||= !ok;
    report.push({ label, id, keyboard, ...page, ok });
    console.log(
      `${ok ? 'ok  ' : 'FAIL'} ${label}: focused=${page.focused} keyboard=${keyboard} offsetTop=${page.offsetTop} vv.height=${page.height} innerHeight=${page.inner}`,
    );
  }
} finally {
  writeFileSync(`${out}/report.json`, JSON.stringify(report, null, 2));
  if (session) await s('DELETE', '').catch(() => {});
}
process.exit(failed ? 1 : 0);
