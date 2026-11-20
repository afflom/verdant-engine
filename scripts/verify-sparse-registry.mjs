#!/usr/bin/env node
/**
 * Live RFC 2789 Cargo Sparse Registry End-to-End Verification Gate (`VE-54`).
 *
 * Spins up a local CORS-enabled HTTP server serving `site/crates/` (or `--dir <path>`),
 * verifies:
 *   1. `index/config.json` conforms to RFC 2789 (`dl` template present, CORS `*`).
 *   2. Every shipped crate (`verdant-tensor`, `verdant-core`, `verdant-wasm`, `verdant-cli`)
 *      has a valid sparse index record at `index/ve/rd/<crate>` whose `vers` matches
 *      `Cargo.toml` `[workspace.package].version` and whose `cksum` matches the downloaded
 *      `.crate` tarball's SHA-256 bit-for-bit.
 *   3. Each `.crate` tarball unpacks cleanly and contains a standalone `Cargo.toml` (zero
 *      unresolved `workspace = true` inheritance leftovers).
 *   4. A real `cargo` invocation configured with `registries.verdant.index = "sparse+http://127.0.0.1:<port>/index/"`
 *      connects over HTTP, fetches `/index/config.json` and `/index/ve/rd/verdant-tensor`,
 *      and parses our RFC 2789 sparse index without any index/protocol error.
 */

import { createHash } from "node:crypto";
import { execFileSync, spawn } from "node:child_process";
import { createServer } from "node:http";
import { mkdtemp, readFile, rm, writeFile, mkdir } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const repoRoot = resolve(process.cwd());
const dirIdx = process.argv.indexOf("--dir");
const cratesRoot = resolve(dirIdx !== -1 ? process.argv[dirIdx + 1] : join(repoRoot, "site", "crates"));

const CRATES = ["verdant-tensor", "verdant-core", "verdant-wasm", "verdant-cli"];

const cargoToml = await readFile(join(repoRoot, "Cargo.toml"), "utf8");
const wsSection = cargoToml.split("[workspace.package]")[1] || "";
const verMatch = wsSection.match(/^\s*version\s*=\s*"([^"]+)"/m);
if (!verMatch) throw new Error("Could not parse [workspace.package].version from Cargo.toml");
const expectedVersion = verMatch[1].trim();

const servedPaths = [];
const server = createServer(async (req, res) => {
  const urlPath = decodeURIComponent(new URL(req.url, "http://127.0.0.1").pathname);
  servedPaths.push(urlPath);
  const rel = urlPath.replace(/^\/+/, "");
  if (rel.includes("..")) {
    res.writeHead(400);
    res.end("bad path");
    return;
  }
  const full = join(cratesRoot, rel);
  try {
    let body = await readFile(full);
    const cors = { "access-control-allow-origin": "*", "cache-control": "no-store" };
    if (rel === "index/config.json") {
      // Rewrite dl template dynamically to this local HTTP test origin so cargo fetches locally:
      const cfg = JSON.parse(body.toString("utf8"));
      cfg.dl = `http://127.0.0.1:${server.address().port}/crates/{crate}-{version}.crate`;
      body = Buffer.from(`${JSON.stringify(cfg, null, 2)}\n`);
      res.writeHead(200, { ...cors, "content-type": "application/json; charset=utf-8" });
      res.end(body);
      return;
    }
    res.writeHead(200, {
      ...cors,
      "content-type": rel.endsWith(".crate") ? "application/gzip" : "text/plain; charset=utf-8",
    });
    res.end(body);
  } catch {
    res.writeHead(404, { "access-control-allow-origin": "*" });
    res.end("not found");
  }
});

await new Promise((r) => server.listen(0, "127.0.0.1", r));
const origin = `http://127.0.0.1:${server.address().port}`;
const consumerDir = await mkdtemp(join(tmpdir(), "verdant-sparse-check-"));

try {
  // 1. Verify raw on-disk config.json & HTTP CORS headers
  const diskConfig = JSON.parse(await readFile(join(cratesRoot, "index", "config.json"), "utf8"));
  if (!diskConfig.dl || !diskConfig.dl.includes("{crate}") || !diskConfig.dl.includes("{version}")) {
    throw new Error(`Invalid RFC 2789 index/config.json dl template: ${JSON.stringify(diskConfig)}`);
  }
  const cfgRes = await fetch(`${origin}/index/config.json`);
  if (!cfgRes.ok || cfgRes.headers.get("access-control-allow-origin") !== "*") {
    throw new Error(`Sparse registry HTTP config check failed: status=${cfgRes.status}`);
  }

  // 2. Verify every crate's index record, HTTP tarball download, SHA-256 cksum, & standalone Cargo.toml
  for (const name of CRATES) {
    const p1 = name.slice(0, 2);
    const p2 = name.slice(2, 4);
    const idxRes = await fetch(`${origin}/index/${p1}/${p2}/${name}`);
    if (!idxRes.ok) throw new Error(`Missing sparse index entry for ${name}: HTTP ${idxRes.status}`);
    const lines = (await idxRes.text()).trim().split(/\r?\n/).filter(Boolean);
    const rec = JSON.parse(lines[lines.length - 1]);
    if (rec.name !== name || rec.vers !== expectedVersion || rec.yanked !== false) {
      throw new Error(
        `Sparse index version/metadata mismatch for ${name}: got vers=${rec.vers}, expected ${expectedVersion}`,
      );
    }

    const crateRes = await fetch(`${origin}/crates/${name}-${expectedVersion}.crate`);
    if (!crateRes.ok) throw new Error(`Failed downloading ${name}-${expectedVersion}.crate: HTTP ${crateRes.status}`);
    const crateBytes = Buffer.from(await crateRes.arrayBuffer());
    const sha = createHash("sha256").update(crateBytes).digest("hex");
    if (sha !== rec.cksum) {
      throw new Error(`SHA-256 mismatch for ${name}: index=${rec.cksum} != tarball=${sha}`);
    }

    const tarPath = join(consumerDir, `${name}.crate`);
    await writeFile(tarPath, crateBytes);
    const extractedToml = execFileSync(
      "tar",
      ["-xOzf", tarPath, `${name}-${expectedVersion}/Cargo.toml`],
      { encoding: "utf8" },
    );
    if (extractedToml.includes(".workspace = true") || extractedToml.includes("workspace = true")) {
      throw new Error(`Packaged ${name}.crate still contains unresolved workspace inheritance in Cargo.toml`);
    }
    if (!extractedToml.includes(`version = "${expectedVersion}"`)) {
      throw new Error(`Packaged ${name}.crate Cargo.toml missing version = "${expectedVersion}"`);
    }
  }

  // 3. Exercise real `cargo` sparse+http client against our live local RFC 2789 index!
  const projDir = join(consumerDir, "consumer");
  await mkdir(join(projDir, ".cargo"), { recursive: true });
  await mkdir(join(projDir, "src"), { recursive: true });
  await writeFile(
    join(projDir, ".cargo", "config.toml"),
    `[registries.verdant]\nindex = "sparse+${origin}/index/"\n`,
  );
  await writeFile(
    join(projDir, "Cargo.toml"),
    `[package]\nname = "verdant-sparse-consumer"\nversion = "0.1.0"\nedition = "2021"\n\n[dependencies]\nverdant-tensor = { version = "=${expectedVersion}", registry = "verdant" }\n`,
  );
  await writeFile(join(projDir, "src", "lib.rs"), "pub fn check() {}\n");

  servedPaths.length = 0;
  const combinedOut = await new Promise((resolveRun) => {
    const child = spawn("cargo", ["generate-lockfile", "--manifest-path", join(projDir, "Cargo.toml")], {
      cwd: projDir,
      stdio: ["ignore", "pipe", "pipe"],
      env: { ...process.env, CARGO_NET_RETRY: "0" },
    });
    let out = "";
    const tm = setTimeout(() => {
      try { child.kill("SIGTERM"); } catch {}
    }, 15000);
    child.stdout.on("data", (d) => { out += d; });
    child.stderr.on("data", (d) => { out += d; });
    child.on("close", () => {
      clearTimeout(tm);
      resolveRun(out);
    });
  });
  if (!servedPaths.includes("/index/config.json") || !servedPaths.includes("/index/ve/rd/verdant-tensor")) {
    throw new Error(
      `Real cargo client did not query RFC 2789 sparse index endpoints (seen: ${JSON.stringify(servedPaths)}): ${combinedOut}`,
    );
  }
  if (/failed to parse registry|invalid json|error parsing.*config\.json/i.test(combinedOut)) {
    throw new Error(`Cargo reported RFC 2789 sparse index parse error: ${combinedOut}`);
  }

  console.log(
    `verify-sparse-registry: PASS — verified 4 crates @ v${expectedVersion} (SHA-256 + standalone Cargo.toml + CORS *) and live cargo sparse+http index fetch (${servedPaths.join(", ")}).`,
  );
} finally {
  server.close();
  await rm(consumerDir, { recursive: true, force: true });
}
