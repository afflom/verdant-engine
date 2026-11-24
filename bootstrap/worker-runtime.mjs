// Module workers have no document import map. Package the locked runtime with
// its closed browser archive/verifier graph (Holo v4, v5, and v6).
import { createHash } from "node:crypto";
import { mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, join, posix } from "node:path";

const PRISM_MODULES = [
  "browser.mjs",
  "archive.mjs",
  "uorc.mjs",
  "view.mjs",
  "core.mjs",
  "capabilities.mjs",
  "session.mjs",
  "closure.mjs",
  "errors.mjs",
  "blake3.mjs",
  "sha256.mjs",
];
const ALL_MODULES = [...PRISM_MODULES, "index.mjs"];
const digest = (bytes) => createHash("sha256").update(bytes).digest("hex");
const refusal = (message) => Object.assign(new Error(`frame.worker.bundle: ${message}`), { code: "frame.worker.bundle" });

export function compactWorkerSource(source) {
  return source
    .replace(/\/\*\*[\s\S]*?\*\//g, "")
    .replace(/^[ \t]*\/\/.*$/gm, "")
    .replace(/\n[ \t]+\n/g, "\n")
    .replace(/\n{2,}/g, "\n")
    .trim() + "\n";
}

export async function prepareWorkerRuntime(site, vendor) {
  const output = join(site, "worker-runtime");
  const files = [];
  for (const path of ALL_MODULES) {
    const source = path === "index.mjs" ? "worker-index.mjs" : `prism-code/src/${path}`;
    const before = await readFile(join(vendor, source), "utf8");
    const imports = [...before.matchAll(/^\s*(?:import\s+(?:[^;\n]*?\s+from\s+)?|export\s+[^;\n]*?\s+from\s+)(["'])([^"']+)\1/gm)];
    for (const [, , specifier] of imports) {
      if (!specifier.startsWith(".")) throw refusal(`unregistered import ${specifier} in ${path}`);
      const target = posix.normalize(posix.join(posix.dirname(path), specifier));
      if (!ALL_MODULES.includes(target)) throw refusal(`import ${specifier} in ${path} is outside the locked runtime`);
    }
    const after = compactWorkerSource(before);
    files.push({ path, source, before, after });
  }
  await rm(output, { recursive: true, force: true });
  for (const file of files) {
    await mkdir(dirname(join(output, file.path)), { recursive: true });
    await writeFile(join(output, file.path), file.after);
  }
  const receipt = { schema: "prism/worker-runtime-packaging/1", files: files.map(({ path, source, before, after }) => ({ path, source, source_sha256: digest(before), output_sha256: digest(after) })) };
  await writeFile(join(output, "packaging.json"), JSON.stringify(receipt, null, 2) + "\n");
  return output;
}
