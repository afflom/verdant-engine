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

    // Copy Cargo.toml, src/, and any tests/examples/build.rs belonging to this crate
    await cp(join(pkgDir, "Cargo.toml"), join(crateStage, "Cargo.toml"));
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
