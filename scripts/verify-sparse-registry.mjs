#!/usr/bin/env node
/**
 * Live RFC 2789 Cargo Sparse Registry End-to-End Verification Gate (`VE-54`) executed natively
 * inside Verdant (`verdant operator verify-registry`) over a Rust loopback HTTP server.
 *
 * Thin delegate into Verdant's lifted native Rust verifier (`build_ops.rs`).
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
    "verify-registry",
    "--root",
    ".",
    ...cliArgs,
  ],
  { cwd: root, stdio: "inherit", env: process.env },
);

if (res.status !== 0) {
  process.exit(res.status ?? 1);
}
