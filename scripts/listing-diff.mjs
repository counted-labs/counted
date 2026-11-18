// What a store-listing push changes, printed before it is written. Used by play.mjs and
// asc.mjs. No dependencies - the text diff is `git diff --no-index` over two temp dirs.

import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";

const normalise = (value) => (value ?? "").replaceAll("\r\n", "\n").trim();

export function showTextDiff(live, wanted) {
    const dir = mkdtempSync(join(tmpdir(), "counted-listing-"));
    try {
        for (const [side, values] of [["live", live], ["new", wanted]]) {
            for (const [key, value] of Object.entries(values)) {
                const path = join(dir, side, `${key}.txt`);
                mkdirSync(dirname(path), { recursive: true });
                writeFileSync(path, `${normalise(value)}\n`);
            }
        }
        const { status, error } = spawnSync("git", ["--no-pager", "-c", "core.autocrlf=false", "-c", "core.safecrlf=false", "diff", "--no-index", "--color", "--src-prefix=", "--dst-prefix=", "live", "new"], { cwd: dir, stdio: "inherit" });
        if (error || status > 1) throw new Error(`git diff failed: ${error?.message ?? `exit ${status}`}`);
        return status === 1;
    } finally {
        rmSync(dir, { recursive: true, force: true });
    }
}

export function screenshotChanges(label, liveHashes, wanted) {
    if (liveHashes.join() === wanted.map((s) => s.hash).join()) return false;
    const live = new Set(liveHashes);
    const changed = wanted.filter((s, i) => liveHashes[i] !== s.hash).map((s) => `${s.name}${live.has(s.hash) ? " (moved)" : ""}`);
    const kept = new Set(wanted.map((s) => s.hash));
    const removed = liveHashes.filter((h) => !kept.has(h)).length;
    console.error(`${label} screenshots: ${changed.join(", ") || "order"} changed${removed ? `; ${removed} removed` : ""}`);
    return true;
}
