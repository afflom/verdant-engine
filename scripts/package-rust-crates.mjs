#!/usr/bin/env node
/**
 * Phase 3 — Optional Maintainer/Developer Rust Library & RFC 2789 Cargo Sparse Index Packager.
 *
 * Packages Verdant's 4 shipped workspace crates (`verdant-tensor`, `verdant-core`, `verdant-wasm`,
 * `verdant-cli`) into deterministic `.crate` gzip tarballs (RFC 2789 format) and generates a valid
 * Cargo Sparse Registry Index (`config.json` + `ve/rd/<crate>`) ready for GitHub Packages (`ghcr.io`)
 * and GitHub Releases — completely offline without hitting crates.io.
 */

import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { cp, mkdir, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";

const repoRoot = resolve(process.cwd());
const outIdx = process.argv.indexOf("--out");
const outDir = resolve(outIdx !== -1 ? process.argv[outIdx + 1] : join(repoRoot, "dist", "crates"));
const dlIdx = process.argv.indexOf("--dl");
const dlTemplate =
  dlIdx !== -1
    ? process.argv[dlIdx + 1]
    : "https://github.com/afflom/verdant-engine/releases/download/v{version}/{crate}-{version}.crate";

const CRATES = ["verdant-tensor", "verdant-core", "verdant-wasm", "verdant-cli"];

const metadata = JSON.parse(
  execFileSync("cargo", ["metadata", "--format-version=1", "--no-deps", "--locked"], {
    cwd: repoRoot,
    encoding: "utf8",
  }),
);

const cratesDir = join(outDir, "crates");
const indexDir = join(outDir, "index");
await mkdir(cratesDir, { recursive: true });
await mkdir(indexDir, { recursive: true });

await writeFile(
  join(indexDir, "config.json"),
  `${JSON.stringify({ dl: dlTemplate, api: "https://github.com/afflom/verdant-engine" }, null, 2)}\n`,
);

const stageRoot = await mkdtemp(join(tmpdir(), "verdant-crate-stage-"));
const summary = [];

try {
  for (const name of CRATES) {
    const pkg = metadata.packages.find((p) => p.name === name);
    if (!pkg) throw new Error(`Workspace crate not found in cargo metadata: ${name}`);

    const pkgDir = dirname(pkg.manifest_path);
    const dirName = `${pkg.name}-${pkg.version}`;
    const crateStage = join(stageRoot, dirName);
    await mkdir(crateStage, { recursive: true });

    // Materialize a standalone Cargo.toml (resolving workspace inheritance so `.crate` is valid outside the repo):
    let rawToml = await readFile(join(pkgDir, "Cargo.toml"), "utf8");
    const wsFields = {
      version: `"${pkg.version}"`,
      edition: `"${pkg.edition}"`,
      "rust-version": `"${pkg.rust_version || "1.89"}"`,
      license: `"${pkg.license || "MIT OR Apache-2.0"}"`,
      repository: `"${pkg.repository || "https://github.com/afflom/verdant-engine"}"`,
      homepage: `"${pkg.homepage || "https://afflom.github.io/verdant-engine/"}"`,
      authors: JSON.stringify(pkg.authors || ["Alex"]),
      keywords: JSON.stringify(pkg.keywords || []),
      categories: JSON.stringify(pkg.categories || []),
    };
    for (const [k, v] of Object.entries(wsFields)) {
      rawToml = rawToml.replace(new RegExp(`^${k}\\.workspace\\s*=\\s*true\\s*$`, "m"), `${k} = ${v}`);
    }
    rawToml = rawToml
      .replace(/^verdant-tensor\.workspace\s*=\s*true\s*$/gm, `verdant-tensor = { version = "${pkg.version}", registry = "verdant" }`)
      .replace(/^verdant-core\.workspace\s*=\s*true\s*$/gm, `verdant-core = { version = "${pkg.version}", registry = "verdant" }`)
      .replace(/^verdant-core\s*=\s*\{\s*workspace\s*=\s*true\s*\}\s*$/gm, `verdant-core = { version = "${pkg.version}", registry = "verdant" }`)
      .replace(/^verdant-core\s*=\s*\{[^}]*path\s*=\s*"\.\.\/verdant-core"([^}]*)\}\s*$/gm, `verdant-core = { version = "${pkg.version}", registry = "verdant"$1 }`)
      .replace(/^uor-matmul\.workspace\s*=\s*true\s*$/gm, `uor-matmul = { version = "0.1.0", git = "https://github.com/UOR-Foundation/uor-matmul", branch = "main", default-features = false }`)
      .replace(/^uorc\.workspace\s*=\s*true\s*$/gm, `uorc = { version = "0.6.0", git = "https://github.com/UOR-Foundation/compression", branch = "main" }`)
      .replace(/^uorc\s*=\s*\{\s*workspace\s*=\s*true\s*,\s*optional\s*=\s*true\s*\}\s*$/gm, `uorc = { version = "0.6.0", git = "https://github.com/UOR-Foundation/compression", branch = "main", optional = true }`)
      .replace(/^uorc_decompress\.workspace\s*=\s*true\s*$/gm, `uorc_decompress = { version = "0.1.0", git = "https://github.com/UOR-Foundation/compression", branch = "main" }`)
      .replace(/^uorc_decompress\s*=\s*\{\s*workspace\s*=\s*true\s*,\s*optional\s*=\s*true\s*\}\s*$/gm, `uorc_decompress = { version = "0.1.0", git = "https://github.com/UOR-Foundation/compression", branch = "main", optional = true }`)
      .replace(/\n*\[lints\]\s*\nworkspace\s*=\s*true\s*/g, "\n");
    await writeFile(join(crateStage, "Cargo.toml"), `${rawToml.trimEnd()}\n`);
    await cp(join(pkgDir, "src"), join(crateStage, "src"), { recursive: true });
    for (const extra of ["build.rs", "README.md", "examples"]) {
      try {
        await cp(join(pkgDir, extra), join(crateStage, extra), { recursive: true });
      } catch {}
    }

    const crateFileName = `${dirName}.crate`;
    const dstCratePath = join(cratesDir, crateFileName);
    execFileSync(
      "tar",
      [
        "--sort=name",
        "--mtime=2026-01-01 00:00:00Z",
        "--owner=0",
        "--group=0",
        "--numeric-owner",
        "-czf",
        dstCratePath,
        "-C",
        stageRoot,
        dirName,
      ],
      { stdio: "pipe" },
    );

    const bytes = await readFile(dstCratePath);
    const cksum = createHash("sha256").update(bytes).digest("hex");

    const prefix1 = pkg.name.slice(0, 2).toLowerCase();
    const prefix2 = pkg.name.slice(2, 4).toLowerCase();
    const entryDir = join(indexDir, prefix1, prefix2);
    await mkdir(entryDir, { recursive: true });

    const deps = pkg.dependencies
      .filter((d) => d.kind === null || d.kind === "normal" || d.kind === "build")
      .map((d) => ({
        name: d.rename || d.name,
        req: d.req,
        features: d.features || [],
        optional: Boolean(d.optional),
        default_features: Boolean(d.uses_default_features),
        target: d.target || null,
        kind: d.kind || "normal",
        package: d.rename ? d.name : null,
      }));

    const indexRecord = {
      name: pkg.name,
      vers: pkg.version,
      deps,
      cksum,
      features: pkg.features || {},
      yanked: false,
      rust_version: pkg.rust_version || null,
    };
    await writeFile(join(entryDir, pkg.name.toLowerCase()), `${JSON.stringify(indexRecord)}\n`);

    summary.push({
      name: pkg.name,
      version: pkg.version,
      file: crateFileName,
      bytes: bytes.length,
      sha256: cksum,
      indexPath: `${prefix1}/${prefix2}/${pkg.name.toLowerCase()}`,
    });
  }
} finally {
  await rm(stageRoot, { recursive: true, force: true });
}

await writeFile(
  join(outDir, "crates-manifest.json"),
  `${JSON.stringify({ spec: "verdant/rust-crates-sparse/1", crates: summary }, null, 2)}\n`,
);

console.log(
  `package-rust-crates: PASS — packaged ${summary.length} Rust crates (${summary.map((c) => `${c.name}@${c.version}`).join(", ")}) + RFC 2789 Cargo sparse index into ${outDir}`,
);
