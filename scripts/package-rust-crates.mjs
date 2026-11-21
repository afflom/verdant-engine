#!/usr/bin/env node
/**
 * Packages Verdant's 4 shipped workspace crates into deterministic `.crate` gzip tarballs and an
 * RFC 2789 Cargo Sparse Registry Index natively inside Verdant (`verdant operator package-crates`).
 *
 * Thin delegate into Verdant's lifted native Rust packager (`build_ops.rs`).
 */

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";

const root = fileURLToPath(new URL("../", import.meta.url));
const cliArgs = process.argv.slice(2);

const res = spawnSync(
  "cargo",
  [
    "run",
    "--locked",
    "-q",
    "-p",
    "verdant-cli",
    "--",
    "operator",
    "package-crates",
    "--root",
    ".",
    ...cliArgs,
  ],
  { cwd: root, stdio: "inherit", env: process.env },
);

if (res.status !== 0) {
  process.exit(res.status ?? 1);
}
