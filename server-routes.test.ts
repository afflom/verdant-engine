import test from "node:test";
import assert from "node:assert/strict";
import { resolve } from "node:path";
import { contentTypeFor, resolveMountTarget, shouldGzip } from "./server-routes.mjs";

const ROOT = "/workspace/verdant";

test("maps root and docs routes into site/ mount", () => {
  const home = resolveMountTarget(ROOT, "/");
  assert.equal(home.kind, "candidate");
  assert.equal(home.filePath, resolve(ROOT, "site"));
  assert.equal(home.endsWithSlash, true);

  const docs = resolveMountTarget(ROOT, "/docs/getting-started/");
  assert.equal(docs.kind, "candidate");
  assert.equal(docs.filePath, resolve(ROOT, "site/docs/getting-started"));
  assert.equal(docs.endsWithSlash, true);

  const docsNoSlash = resolveMountTarget(ROOT, "/docs");
  assert.equal(docsNoSlash.kind, "candidate");
  assert.equal(docsNoSlash.filePath, resolve(ROOT, "site/docs"));
  assert.equal(docsNoSlash.endsWithSlash, false);

  const demoPage = resolveMountTarget(ROOT, "/demo/");
  assert.equal(demoPage.kind, "candidate");
  assert.equal(demoPage.filePath, resolve(ROOT, "site/demo"));
  assert.equal(demoPage.endsWithSlash, true);
});

test("maps secondary surfaces (/try and /templates/browser-3d-starter)", () => {
  const tryPage = resolveMountTarget(ROOT, "/try/");
  assert.equal(tryPage.kind, "candidate");
  assert.equal(tryPage.filePath, resolve(ROOT, "try"));
  assert.equal(tryPage.endsWithSlash, true);

  const starterJs = resolveMountTarget(ROOT, "/templates/browser-3d-starter/game.mjs");
  assert.equal(starterJs.kind, "candidate");
  assert.equal(starterJs.filePath, resolve(ROOT, "templates/browser-3d-starter/game.mjs"));

  const starterAlias = resolveMountTarget(ROOT, "/starter/");
  assert.equal(starterAlias.kind, "candidate");
  assert.equal(starterAlias.filePath, resolve(ROOT, "templates/browser-3d-starter"));
  assert.equal(starterAlias.endsWithSlash, true);
});

test("refuses path traversal and malformed URI encodings", () => {
  assert.equal(resolveMountTarget(ROOT, "/../Cargo.toml").kind, "forbidden");
  assert.equal(resolveMountTarget(ROOT, "/docs/../../package.json").kind, "forbidden");
  assert.equal(resolveMountTarget(ROOT, "/%2e%2e/Cargo.toml").kind, "forbidden");
  assert.equal(resolveMountTarget(ROOT, "/%E0%A4%A").kind, "bad-request");
  assert.equal(resolveMountTarget(ROOT, "/bad\0path").kind, "bad-request");
});

test("returns streaming-compliant MIME types and gzip decisions", () => {
  assert.equal(contentTypeFor("verdant-wasm.wasm"), "application/wasm");
  assert.equal(contentTypeFor("engine-worker.mjs"), "text/javascript; charset=utf-8");
  assert.equal(contentTypeFor("manifest.webmanifest"), "application/manifest+json; charset=utf-8");
  assert.equal(contentTypeFor("Archivo-SemiBold.woff2"), "font/woff2");
  assert.equal(contentTypeFor("bundle.tar"), "application/x-tar");

  assert.equal(shouldGzip("verdant-wasm.wasm", "br, gzip, deflate", 4096), true);
  assert.equal(shouldGzip("Archivo-SemiBold.woff2", "gzip", 24000), false);
  assert.equal(shouldGzip("index.html", "identity", 4096), false);
  assert.equal(shouldGzip("tiny.json", "gzip", 64), false);
});
