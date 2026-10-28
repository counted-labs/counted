import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const ORIGIN = "https://counted.fr";
const APP_PAGES = ["/projects", "/login", "/register", "/settings", "/help", "/privacy", "/monitor-not-found"];
const DX_BOOTSTRAP =
	'window.hydrate_queue=[];window.dx_hydrate=(id,data,debug_types,debug_locations)=>{let decoded=atob(data),bytes=Uint8Array.from(decoded,(c)=>c.charCodeAt(0));if(window.hydration_callback)window.hydration_callback(id,bytes,debug_types,debug_locations);else window.hydrate_queue.push([id,bytes,debug_types,debug_locations])};';
const HYDRATION_DATA = /^window\.initial_dioxus_hydration_data="[A-Za-z0-9+/=]*";$/;

const sums = new Map(
	readFileSync(process.argv[2], "utf8")
		.trim()
		.split("\n")
		.map((line) => line.split(/ +/).reverse()),
);
sums.delete("server");

const failures = [];
const fail = (message) => {
	failures.push(message);
	console.log(`FAIL ${message}`);
};

async function get(path, status = 200) {
	await new Promise((resolve) => setTimeout(resolve, 150));
	const response = await fetch(ORIGIN + path, { headers: { "accept-encoding": "identity" }, redirect: "manual" });
	if (response.status !== status) throw new Error(`${path}: HTTP ${response.status}, expected ${status}`);
	return Buffer.from(await response.arrayBuffer());
}

async function getOrFail(path, status) {
	try {
		return await get(path, status);
	} catch (error) {
		fail(error.message);
		return null;
	}
}

for (const [path, expected] of sums) {
	const body = await getOrFail(`/${path}`);
	if (body && createHash("sha256").update(body).digest("hex") !== expected) fail(`/${path}: served bytes differ from the signed sums`);
}
console.log(`${sums.size} served files checked against the signed sums`);

function checkMarkup(page, html, allowScript) {
	for (const [, attributes, content] of html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script\s*>/gi)) {
		const reason = allowScript(attributes, content.trim());
		if (reason) fail(`${page}: ${reason}: <script${attributes}>${content.slice(0, 120)}`);
	}
	for (const [tag] of html.matchAll(/<[a-z][^>]*\son[a-z]+\s*=[^>]*>/gi)) fail(`${page}: inline event handler: ${tag.slice(0, 160)}`);
	if (/javascript:/i.test(html)) fail(`${page}: javascript: URL`);
}

function appScript(attributes, content) {
	const src = attributes.match(/\ssrc="([^"]*)"/)?.[1];
	if (src !== undefined) return content === "" && sums.has(src.replace(/^\/(\.\/)?/, "")) ? null : "script outside the signed sums";
	return content === DX_BOOTSTRAP || HYDRATION_DATA.test(content) ? null : "unknown inline script";
}

function siteScript(attributes, content) {
	if (!/^\s*type="application\/ld\+json"\s*$/.test(attributes)) return "executable script on a site page";
	try {
		JSON.parse(content);
		return null;
	} catch {
		return "ld+json that is not JSON";
	}
}

for (const page of APP_PAGES) {
	const body = await getOrFail(page, page === "/monitor-not-found" ? 404 : 200);
	if (!body) continue;
	const html = body.toString("utf8");
	checkMarkup(page, html, appScript);
	for (const [, url] of html.matchAll(/\s(?:src|href)="(\/(?:\.\/)?assets\/[^"]*)"/g)) {
		if (!sums.has(url.replace(/^\/(\.\/)?/, ""))) fail(`${page}: ${url} is outside the signed sums`);
	}
}
console.log(`${APP_PAGES.length} app pages checked`);

const sitemap = (await get("/sitemap.xml")).toString("utf8");
const sitePages = [...sitemap.matchAll(/<loc>([^<]*)<\/loc>/g)].map(([, loc]) => new URL(loc));
for (const url of sitePages) {
	if (url.origin !== ORIGIN) {
		fail(`sitemap lists ${url}`);
		continue;
	}
	const body = await getOrFail(url.pathname);
	if (body) checkMarkup(url.pathname, body.toString("utf8"), siteScript);
}
console.log(`${sitePages.length} site pages checked`);

const securityTxt = await getOrFail("/.well-known/security.txt");
if (securityTxt) {
	const expires = Date.parse(securityTxt.toString("utf8").match(/^Expires:\s*(\S+)\s*$/m)?.[1]);
	const days = Math.floor((expires - Date.now()) / 86_400_000);
	if (!(days >= 30)) fail(`/.well-known/security.txt: ${Number.isNaN(expires) ? "no valid Expires" : `Expires in ${days} days`}, renew it`);
	else console.log(`security.txt expires in ${days} days`);
}

if (failures.length) {
	console.log(`${failures.length} failure(s)`);
	process.exit(1);
}
console.log("counted.fr serves exactly the signed release, and no page runs code outside it");
