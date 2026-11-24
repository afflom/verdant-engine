import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { mkdtemp, readFile, rm, cp, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import vm from "node:vm";
import test from "node:test";

const root = fileURLToPath(new URL("../", import.meta.url));
const temp = await mkdtemp(join(tmpdir(), "frame-worker-runtime-"));
test.after(async () => rm(temp, { recursive: true, force: true }));
await cp(join(root, "try/vendor"), join(temp, "vendor"), { recursive: true });
const { prepareWorkerRuntime: prepare, compactWorkerSource } = await import("../bootstrap/worker-runtime.mjs");
const output = await prepare(temp, join(temp, "vendor"));
const sha256 = (bytes) => createHash("sha256").update(bytes).digest("hex");

async function linkGraph(entry) {
  const context = vm.createContext({ TextEncoder, TextDecoder, Uint8Array, WebAssembly, URL, crypto: globalThis.crypto });
  const cache = new Map();
  async function load(file) {
    file = resolve(file);
    if (cache.has(file)) return cache.get(file);
    const module = new vm.SourceTextModule(await readFile(file, "utf8"), { context, identifier: file });
    cache.set(file, module);
    return module;
  }
  const entryModule = await load(entry);
  await entryModule.link((specifier, parent) => {
    assert.ok(specifier.startsWith("."), `module workers cannot resolve bare specifier ${specifier}`);
    return load(resolve(dirname(parent.identifier), specifier));
  });
  await entryModule.evaluate();
  return { module: entryModule, files: [...cache.keys()] };
}
test("the actual worker dependency graph links without a document import map", async () => {
  const { module, files } = await linkGraph(join(output, "browser.mjs"));
  assert.equal(typeof module.namespace.load, "function");
  assert.equal(files.length, 11);
});
test("worker packaging binds runtime digests deterministically to the locked vendor tree", async () => {
  assert.equal(typeof prepare, "function", "production worker packager exists");
  const receipt = JSON.parse(await readFile(join(output, "packaging.json"), "utf8"));
  assert.equal(receipt.files.length, 12);
  for (const file of receipt.files) {
    const before = await readFile(join(temp, "vendor", file.source), "utf8");
    const after = await readFile(join(output, file.path), "utf8");
    assert.equal(sha256(before), file.source_sha256);
    assert.equal(sha256(after), file.output_sha256);
    assert.equal(after, compactWorkerSource(before));
  }
});
test("unknown bare imports fail closed rather than falling back to page execution", async () => {
  assert.equal(typeof prepare, "function");
  const file = join(temp, "vendor/prism-code/src/core.mjs");
  const before = await readFile(file, "utf8");
  await writeFile(file, before + '\nimport "unregistered-module";\n');
  await assert.rejects(prepare(temp, join(temp, "vendor")), /unregistered-module/);
  await writeFile(file, before);
});
test("a dependency leaving the locked runtime refuses assembly", async () => {
  assert.equal(typeof prepare, "function");
  const file = join(temp, "vendor/prism-code/src/core.mjs");
  const before = await readFile(file, "utf8");
  await writeFile(file, before + '\nimport "../../../external.mjs";\n');
  await assert.rejects(prepare(temp, join(temp, "vendor")), /outside|unregistered/);
  await writeFile(file, before);
});
test("packaging uses its explicit locked source even when the output has no vendor directory", async () => {
  assert.equal(typeof prepare, "function");
  const destination = join(temp, "fresh-scaffold");
  const packaged = await prepare(destination, join(root, "try/vendor"));
  const { module, files } = await linkGraph(join(packaged, "browser.mjs"));
  assert.equal(typeof module.namespace.load, "function");
  assert.equal(files.length, 11);
});
