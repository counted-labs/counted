#!/usr/bin/env node
// App Store Connect API client for the App Store version of `version` in packages/mobile/Cargo.toml:
// its release notes, and the listing (text and iPhone screenshots). Submitting for review stays a
// click in App Store Connect. No dependencies - Node >= 20 (fetch, crypto).
//
//   node scripts/asc.mjs release-notes
//   node scripts/asc.mjs listing
//   node scripts/asc.mjs check-version    (fails unless `version` is above every approved one)
//   node scripts/asc.mjs build-number     (the next CFBundleVersion)
//   node scripts/asc.mjs wait-build <n>   (until App Store Connect lists build n)
//
// APP_STORE_CONNECT_PRIVATE_KEY (the .p8 contents, or a path to it), APP_STORE_CONNECT_KEY_IDENTIFIER
// and APP_STORE_CONNECT_ISSUER_ID - the names Codemagic's app_store group uses (see docs/ios.md).

import { readFileSync, readdirSync } from "node:fs";
import { createHash, createSign } from "node:crypto";
import { screenshotChanges, showTextDiff } from "./listing-diff.mjs";
import { IOS_LOCALES as LOCALES, ROOT } from "./store-locales.mjs";

const APP_ID = "6772807915";
const API = "https://api.appstoreconnect.apple.com/v1";
const EDITABLE = new Set(["PREPARE_FOR_SUBMISSION", "DEVELOPER_REJECTED", "REJECTED", "METADATA_REJECTED"]);

function fail(message) {
    console.error(`ERROR: ${message}`);
    process.exit(1);
}

function text(path, limit) {
    let value;
    try {
        value = readFileSync(`${ROOT}/${path}`, "utf8").replaceAll("\r\n", "\n").trim();
    } catch (e) {
        fail(`cannot read ${path}: ${e.message}`);
    }
    if (!value) fail(`${path} is empty`);
    if (value.length > limit) fail(`${path} is ${value.length} characters, the limit is ${limit}`);
    const emoji = value.match(/\p{Extended_Pictographic}|\p{Regional_Indicator}/u);
    if (emoji) fail(`${path} contains ${emoji[0]}: App Store Connect rejects emoji`);
    return value;
}

function cargoVersion() {
    const match = readFileSync(`${ROOT}/packages/mobile/Cargo.toml`, "utf8").match(/^version = "(\d+)\.(\d+)\.(\d+)"/m);
    if (!match) fail("no version in packages/mobile/Cargo.toml");
    const [major, minor, patch] = match.slice(1).map(Number);
    return { versionString: `${major}.${minor}.${patch}`, code: major * 1_000_000 + minor * 1000 + patch };
}

const b64url = (input) => Buffer.from(input).toString("base64url");
let token = { value: null, expires: 0 };

function bearer() {
    const now = Math.floor(Date.now() / 1000);
    if (token.expires - now > 60) return token.value;
    const { APP_STORE_CONNECT_PRIVATE_KEY: key, APP_STORE_CONNECT_KEY_IDENTIFIER: kid, APP_STORE_CONNECT_ISSUER_ID: iss } = process.env;
    if (!key || !kid || !iss) fail("APP_STORE_CONNECT_PRIVATE_KEY, APP_STORE_CONNECT_KEY_IDENTIFIER and APP_STORE_CONNECT_ISSUER_ID must be set");
    let pem = key;
    if (!key.includes("BEGIN PRIVATE KEY")) {
        try {
            pem = readFileSync(key, "utf8");
        } catch (e) {
            fail(`cannot read the App Store Connect key at ${key}: ${e.message}`);
        }
    }
    // Apple's cap is 20 minutes on its own clock; a local clock even a second ahead gets a 401.
    const expires = now + 1140;
    const unsigned = `${b64url(JSON.stringify({ alg: "ES256", kid, typ: "JWT" }))}.${b64url(JSON.stringify({ iss, iat: now, exp: expires, aud: "appstoreconnect-v1" }))}`;
    const signature = createSign("SHA256").update(unsigned).sign({ key: pem, dsaEncoding: "ieee-p1363" }, "base64url");
    token = { value: `${unsigned}.${signature}`, expires };
    return token.value;
}

async function call(method, path, body) {
    const res = await fetch(path.startsWith("https://") ? path : `${API}${path}`, {
        method,
        headers: { Authorization: `Bearer ${bearer()}`, "Content-Type": "application/json" },
        body: body === undefined ? undefined : JSON.stringify(body),
    });
    const text = await res.text();
    if (!res.ok) fail(`${method} ${path} → ${res.status}\n${text}`);
    return text ? JSON.parse(text) : {};
}

const state = (resource) => resource.attributes.appVersionState ?? resource.attributes.appStoreState ?? resource.attributes.state;

async function appStoreVersion(versionString) {
    const { data } = await call("GET", `/apps/${APP_ID}/appStoreVersions?filter[platform]=IOS&limit=200`);
    const existing = data.find((v) => v.attributes.versionString === versionString);
    if (existing) return existing;
    const draft = data.find((v) => EDITABLE.has(state(v)));
    if (draft) {
        fail(`App Store version ${draft.attributes.versionString} is still a draft (${state(draft)}), and only one can be open - rename it to ${versionString} or delete it in App Store Connect`);
    }
    const created = await call("POST", "/appStoreVersions", {
        data: {
            type: "appStoreVersions",
            attributes: { platform: "IOS", versionString },
            relationships: { app: { data: { type: "apps", id: APP_ID } } },
        },
    });
    console.error(`created App Store version ${versionString}`);
    return created.data;
}

async function upsertLocalizations(listPath, type, parent, attributesByLocale) {
    const { data } = await call("GET", `${listPath}?limit=200`);
    const ids = {};
    for (const [locale, attributes] of Object.entries(attributesByLocale)) {
        const existing = data.find((l) => l.attributes.locale === locale);
        const saved = existing
            ? await call("PATCH", `/${type}/${existing.id}`, { data: { type, id: existing.id, attributes } })
            : await call("POST", `/${type}`, { data: { type, attributes: { locale, ...attributes }, relationships: parent } });
        ids[locale] = saved.data.id;
    }
    return ids;
}

function versionLocalizations(version, attributesByLocale) {
    return upsertLocalizations(
        `/appStoreVersions/${version.id}/appStoreVersionLocalizations`,
        "appStoreVersionLocalizations",
        { appStoreVersion: { data: { type: "appStoreVersions", id: version.id } } },
        attributesByLocale,
    );
}

async function releaseNotes() {
    const { versionString, code } = cargoVersion();
    const notes = Object.fromEntries(Object.entries(LOCALES).map(([locale, { notes }]) =>
        [locale, { whatsNew: text(`packages/mobile/fastlane/metadata/android/${notes}/changelogs/${code}.txt`, 500) }]));
    const version = await appStoreVersion(versionString);
    if (!EDITABLE.has(state(version))) {
        console.log(`App Store ${versionString} is ${state(version)} - release notes left as they are`);
        return;
    }
    await versionLocalizations(version, notes);
    console.log(`App Store ${versionString}: release notes set for ${Object.keys(notes).join(", ")}`);
}

// App Store Connect closes a version's train once it is approved: a build whose
// CFBundleShortVersionString is not above the highest approved version is rejected at upload
// (90062 + 90186), after the hour-long build. The listed states are the ones still open for new
// builds; anything else - including a state Apple adds later - counts as approved, the side that
// only costs a version bump.
const OPEN = new Set(["PREPARE_FOR_SUBMISSION", "READY_FOR_REVIEW", "WAITING_FOR_REVIEW", "WAITING_FOR_EXPORT_COMPLIANCE",
    "IN_REVIEW", "REJECTED", "METADATA_REJECTED", "DEVELOPER_REJECTED", "INVALID_BINARY", "PENDING_CONTRACT"]);

function compareVersions(a, b) {
    const [x, y] = [a.split(".").map(Number), b.split(".").map(Number)];
    for (let i = 0; i < Math.max(x.length, y.length); i++) {
        if ((x[i] ?? 0) !== (y[i] ?? 0)) return (x[i] ?? 0) - (y[i] ?? 0);
    }
    return 0;
}

async function checkVersion() {
    const { versionString } = cargoVersion();
    const { data } = await call("GET", `/apps/${APP_ID}/appStoreVersions?filter[platform]=IOS&limit=200`);
    for (const v of data) console.log(`${v.attributes.versionString}\t${state(v)}`);
    const approved = data.filter((v) => !OPEN.has(state(v))).map((v) => v.attributes.versionString);
    const highest = approved.sort(compareVersions).at(-1);
    if (highest && compareVersions(versionString, highest) <= 0) {
        fail(`packages/mobile/Cargo.toml version ${versionString} is not above the approved App Store version ${highest}: App Store Connect refuses the upload (90062/90186). Bump \`version\` and push again.`);
    }
    console.log(`OK: ${versionString} > approved ${highest ?? "none"}`);
}

async function highestBuildNumber() {
    const { data } = await call("GET", `/builds?filter[app]=${APP_ID}&sort=-uploadedDate&limit=200&fields[builds]=version`);
    return Math.max(0, ...data.map((b) => Number(b.attributes.version)).filter(Number.isInteger));
}

// Not altool's exit code: since Xcode 26 it exits 0 on a failed upload. The upload is real once
// App Store Connect lists the build.
async function waitBuild(number) {
    if (!/^\d+$/.test(number ?? "")) fail("usage: asc.mjs wait-build <build number>");
    const end = Date.now() + 20 * 60_000;
    for (;;) {
        const { data } = await call("GET", `/builds?filter[app]=${APP_ID}&filter[version]=${number}&limit=1&fields[builds]=version,processingState`);
        const build = data[0];
        const processing = build?.attributes.processingState;
        if (processing === "INVALID" || processing === "FAILED") fail(`build ${number} is ${processing} in App Store Connect - Apple's email says why`);
        if (build) {
            console.log(`build ${number} is in App Store Connect (${processing})`);
            return;
        }
        if (Date.now() > end) fail(`build ${number} did not appear in App Store Connect within 20 minutes`);
        await new Promise((r) => setTimeout(r, 30_000));
    }
}

// One gallery per device folder of gen-store-screenshots.ps1; each goes to every slot its size fits.
const DEVICES = ["ios", "ipad"];

function screenshots(device, language) {
    const dir = `marketing/store/out/${device}/${language}`;
    let files;
    try {
        files = readdirSync(`${ROOT}/${dir}`).filter((f) => f.endsWith(".png")).sort();
    } catch {
        fail(`no ${dir} - run \`make store-shots\` then \`pwsh scripts/gen-store-screenshots.ps1\``);
    }
    if (files.length === 0 || files.length > 10) fail(`${dir} has ${files.length} panels, the App Store takes 1 to 10`);
    const shots = files.map((name) => {
        const bytes = readFileSync(`${ROOT}/${dir}/${name}`);
        if (bytes.toString("latin1", 12, 16) !== "IHDR") fail(`${dir}/${name} is not a PNG`);
        const hash = createHash("sha256").update(bytes).digest("hex").slice(0, 12);
        return {
            name,
            bytes,
            hash,
            width: bytes.readUInt32BE(16),
            height: bytes.readUInt32BE(20),
            referenceName: `${language}/${name} ${hash}`,
        };
    });
    const odd = shots.find((s) => s.width !== shots[0].width || s.height !== shots[0].height);
    if (odd) fail(`${dir}/${odd.name} is ${odd.width}x${odd.height}, the others ${shots[0].width}x${shots[0].height}`);
    return shots;
}

function screenshotGroups(refData, width, height) {
    const specs = new Set(refData.imageSpecs
        .filter((s) => s.compatiblePlacementTypes?.includes("APP_SCREENSHOT")
            && s.dimensions.minWidth === width && s.dimensions.maxWidth === width
            && s.dimensions.minHeight === height && s.dimensions.maxHeight === height)
        .map((s) => s.specId));
    const type = refData.placementTypes.find((t) => t.placementTypeId === "APP_SCREENSHOT");
    const groups = (type?.specMappings ?? []).filter((m) => m.specs.some((s) => specs.has(s))).map((m) => m.placementGroupId);
    if (groups.length === 0) fail(`no App Store screenshot slot takes ${width}x${height}`);
    return groups;
}

async function uploadImage(library, shot) {
    const { data: found } = await call("GET", `/appAssetLibraries/${library}/images?filter[referenceName]=${encodeURIComponent(shot.referenceName)}&limit=10`);
    const reusable = found.find((i) => i.attributes.referenceName === shot.referenceName && i.attributes.state !== "FAILED");
    if (reusable) return reusable.id;
    const { data: image } = await call("POST", "/appAssetLibraryImages", {
        data: {
            type: "appAssetLibraryImages",
            attributes: { fileName: shot.name, fileSize: shot.bytes.length, category: "APP_SCREENSHOTS_AND_PREVIEWS", referenceName: shot.referenceName },
            relationships: { assetLibrary: { data: { type: "appAssetLibraries", id: library } } },
        },
    });
    for (const op of image.attributes.uploadOperations) {
        const res = await fetch(op.url, {
            method: op.method,
            headers: Object.fromEntries(op.requestHeaders.map((h) => [h.name, h.value])),
            body: shot.bytes.subarray(op.offset, op.offset + op.length),
        });
        if (!res.ok) fail(`upload of ${shot.referenceName} → ${res.status}\n${await res.text()}`);
    }
    await call("PATCH", `/appAssetLibraryImages/${image.id}`, { data: { type: "appAssetLibraryImages", id: image.id, attributes: { uploaded: true } } });
    console.error(`uploaded ${shot.referenceName}`);
    return image.id;
}

async function waitForProcessing(ids) {
    const deadline = Date.now() + 10 * 60_000;
    for (const id of ids) {
        for (;;) {
            const { data } = await call("GET", `/appAssetLibraryImages/${id}`);
            if (data.attributes.state === "FAILED") fail(`${data.attributes.referenceName} failed processing: ${JSON.stringify(data.attributes.stateDetails)}`);
            if (!["AWAITING_UPLOAD", "UPLOAD_COMPLETE"].includes(data.attributes.state)) break;
            if (Date.now() > deadline) fail(`${data.attributes.referenceName} still processing after 10 minutes`);
            await new Promise((resolve) => setTimeout(resolve, 5000));
        }
    }
}

const placementsOf = (localization, group) =>
    call("GET", `/appStoreVersionLocalizations/${localization}/placements?filter[placementType]=APP_SCREENSHOT&filter[placementGroup]=${group}&sort=placementGroupPosition&include=image&limit=50`);

// Only images this script uploaded carry the hash in their referenceName; any other one never matches.
async function placedHashes(localization, group) {
    const { data, included = [] } = await placementsOf(localization, group);
    const names = Object.fromEntries(included.map((i) => [i.id, i.attributes.referenceName ?? ""]));
    return data.map((p) => names[p.relationships?.image?.data?.id]?.match(/ ([0-9a-f]{12})$/)?.[1] ?? `unknown ${p.id}`);
}

async function placeScreenshots(localization, group, imageIds) {
    const { data: current } = await placementsOf(localization, group);
    if (current.map((p) => p.relationships?.image?.data?.id).join() === imageIds.join()) return false;
    for (const placement of current) await call("DELETE", `/appAssetLibraryPlacements/${placement.id}`);
    const placements = [];
    for (const id of imageIds) {
        const { data } = await call("POST", "/appAssetLibraryPlacements", {
            data: {
                type: "appAssetLibraryPlacements",
                attributes: { placementType: "APP_SCREENSHOT", placementGroup: group },
                relationships: {
                    image: { data: { type: "appAssetLibraryImages", id } },
                    appStoreVersionLocalization: { data: { type: "appStoreVersionLocalizations", id: localization } },
                },
            },
        });
        placements.push({ type: "appAssetLibraryPlacements", id: data.id });
    }
    await call("POST", "/appAssetLibraryPlacementOrderingRequests", {
        data: {
            type: "appAssetLibraryPlacementOrderingRequests",
            attributes: { placementGroup: group },
            relationships: {
                orderedPlacements: { data: placements },
                appStoreVersionLocalization: { data: { type: "appStoreVersionLocalizations", id: localization } },
            },
        },
    });
    return true;
}

async function listing() {
    const { versionString } = cargoVersion();
    const info = {}, store = {};
    for (const [locale, { listing: dir }] of Object.entries(LOCALES)) {
        info[locale] = { name: text(`${dir}/name.txt`, 30), subtitle: text(`${dir}/subtitle.txt`, 30), privacyPolicyUrl: text(`${dir}/privacy_url.txt`, 255) };
        store[locale] = {
            description: text(`${dir}/description.txt`, 4000),
            keywords: text(`${dir}/keywords.txt`, 100),
            promotionalText: text(`${dir}/promotional_text.txt`, 170),
            supportUrl: text(`${dir}/support_url.txt`, 255),
        };
    }
    const shots = Object.fromEntries([...new Set(Object.values(LOCALES).map((l) => l.lang))].map((language) => [language, DEVICES.map((device) => screenshots(device, language))]));

    const { data: versions } = await call("GET", `/apps/${APP_ID}/appStoreVersions?filter[platform]=IOS&limit=200`);
    const target = versions.find((v) => v.attributes.versionString === versionString)
        ?? versions.filter((v) => state(v) === "READY_FOR_DISTRIBUTION").sort((a, b) => b.attributes.createdDate.localeCompare(a.attributes.createdDate))[0];
    if (target?.attributes.versionString === versionString && !EDITABLE.has(state(target))) {
        fail(`App Store ${versionString} is ${state(target)} - the listing can only change on a version not yet submitted`);
    }
    const { data: appInfos } = await call("GET", `/apps/${APP_ID}/appInfos`);
    const liveInfo = appInfos.find((i) => EDITABLE.has(state(i))) ?? appInfos.find((i) => state(i) === "READY_FOR_SALE");
    const { data: infoLocalizations } = liveInfo ? await call("GET", `/appInfos/${liveInfo.id}/appInfoLocalizations?limit=200`) : { data: [] };
    const { data: storeLocalizations } = target ? await call("GET", `/appStoreVersions/${target.id}/appStoreVersionLocalizations?limit=200`) : { data: [] };
    const { data: [refData] } = await call("GET", "/appAssetLibraryRefData?fields[appAssetLibraryRefData]=imageSpecs,placementTypes");
    console.error(`comparing with App Store ${target ? `${target.attributes.versionString} (${state(target)})` : "- no version yet"}`);

    const liveText = {}, wantedText = {};
    let changed = false;
    for (const locale of Object.keys(LOCALES)) {
        for (const [localizations, values] of [[infoLocalizations, info[locale]], [storeLocalizations, store[locale]]]) {
            const current = localizations.find((l) => l.attributes.locale === locale);
            for (const [field, value] of Object.entries(values)) {
                if (current) liveText[`${locale}/${field}`] = current.attributes[field];
                wantedText[`${locale}/${field}`] = value;
            }
        }
        const language = LOCALES[locale].lang;
        const localization = storeLocalizations.find((l) => l.attributes.locale === locale);
        for (const set of shots[language]) {
            for (const group of screenshotGroups(refData.attributes, set[0].width, set[0].height)) {
                const live = localization ? await placedHashes(localization.id, group) : [];
                changed = screenshotChanges(`App Store ${locale} ${group}`, live, set) || changed;
            }
        }
    }
    changed = showTextDiff(liveText, wantedText) || changed;
    if (!changed) {
        console.log(`App Store ${versionString}: listing already up to date`);
        return;
    }
    const version = await appStoreVersion(versionString);
    if (!EDITABLE.has(state(version))) fail(`App Store ${versionString} is ${state(version)} - the listing can only change on a version not yet submitted`);
    const { data: openInfos } = await call("GET", `/apps/${APP_ID}/appInfos`);
    const appInfo = openInfos.find((i) => EDITABLE.has(state(i)));
    if (!appInfo) fail("no editable app info - App Store Connect opens one with each new version");

    await upsertLocalizations(`/appInfos/${appInfo.id}/appInfoLocalizations`, "appInfoLocalizations",
        { appInfo: { data: { type: "appInfos", id: appInfo.id } } }, info);
    const localizations = await versionLocalizations(version, store);
    console.error(`App Store ${versionString}: text set for ${Object.keys(store).join(", ")}`);

    const { data: library } = await call("GET", `/apps/${APP_ID}/assetLibrary`);
    const imageIds = {};
    for (const [language, sets] of Object.entries(shots)) {
        imageIds[language] = [];
        for (const set of sets) {
            const ids = [];
            for (const shot of set) ids.push(await uploadImage(library.id, shot));
            imageIds[language].push(ids);
        }
    }
    await waitForProcessing(Object.values(imageIds).flat(2));

    for (const locale of Object.keys(LOCALES)) {
        const language = LOCALES[locale].lang;
        for (const [i, set] of shots[language].entries()) {
            const ids = imageIds[language][i];
            for (const group of screenshotGroups(refData.attributes, set[0].width, set[0].height)) {
                const changed = await placeScreenshots(localizations[locale], group, ids);
                console.error(`${locale} ${group}: ${changed ? `${ids.length} screenshots placed` : "unchanged"}`);
            }
        }
    }
    console.log(`App Store ${versionString}: listing set - submit the version for review in App Store Connect`);
}

const [command] = process.argv.slice(2);
switch (command) {
    case "release-notes":
        await releaseNotes();
        break;
    case "listing":
        await listing();
        break;
    case "check-version":
        await checkVersion();
        break;
    case "build-number":
        console.log((await highestBuildNumber()) + 1);
        break;
    case "wait-build":
        await waitBuild(process.argv[3]);
        break;
    default:
        fail("usage: asc.mjs release-notes | listing | check-version | build-number | wait-build <n>");
}
