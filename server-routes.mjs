import { extname, normalize, resolve, sep } from "node:path";

export const MIME_TYPES = Object.freeze({
  ".html": "text/html; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".mjs": "text/javascript; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".webmanifest": "application/manifest+json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".woff2": "font/woff2",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".mp4": "video/mp4",
  ".wasm": "application/wasm",
  ".holo": "application/octet-stream",
  ".uorc": "application/octet-stream",
  ".glb": "application/octet-stream",
  ".tar": "application/x-tar",
  ".md": "text/markdown; charset=utf-8",
  ".txt": "text/plain; charset=utf-8",
});

export const COMPRESSIBLE_EXTENSIONS = new Set([
  ".html",
  ".css",
  ".mjs",
  ".js",
  ".json",
  ".webmanifest",
  ".svg",
  ".wasm",
  ".holo",
  ".uorc",
  ".tar",
  ".md",
  ".txt",
]);

export function contentTypeFor(filePath) {
  const ext = extname(filePath).toLowerCase();
  return MIME_TYPES[ext] ?? "application/octet-stream";
}

export function shouldGzip(filePath, acceptEncoding = "", byteLength = 0) {
  if (byteLength < 256) return false;
  if (!/\bgzip\b/i.test(acceptEncoding)) return false;
  const ext = extname(filePath).toLowerCase();
  return COMPRESSIBLE_EXTENSIONS.has(ext);
}

const PREFIX_MOUNTS = [
  { prefix: "/try", dir: "try" },
  { prefix: "/templates/browser-3d-starter", dir: "templates/browser-3d-starter" },
  { prefix: "/starter", dir: "templates/browser-3d-starter" },
  { prefix: "/frame/backdrops", dir: "frame/backdrops" },
];

export function resolveMountTarget(rootDir, rawPathname) {
  if (typeof rawPathname !== "string" || !rawPathname.startsWith("/")) {
    return { kind: "bad-request" };
  }
  let decoded;
  try {
    decoded = decodeURIComponent(rawPathname);
  } catch {
    return { kind: "bad-request" };
  }
  if (decoded.includes("\0")) {
    return { kind: "bad-request" };
  }

  // Reject explicit dot-dot traversal segments before filesystem resolution.
  const rawSegments = decoded.split("/");
  if (rawSegments.some((seg) => seg === "..")) {
    return { kind: "forbidden" };
  }

  let mountDir = "site";
  let subPath = decoded;
  for (const mount of PREFIX_MOUNTS) {
    if (decoded === mount.prefix || decoded.startsWith(`${mount.prefix}/`)) {
      mountDir = mount.dir;
      subPath = decoded.slice(mount.prefix.length) || "/";
      break;
    }
  }

  const endsWithSlash = decoded.endsWith("/");
  const mountRoot = resolve(rootDir, mountDir);
  const relativeSubPath = normalize(subPath).replace(/^[/\\]+/, "");
  const filePath = resolve(mountRoot, relativeSubPath);

  if (filePath !== mountRoot && !filePath.startsWith(mountRoot + sep)) {
    return { kind: "forbidden" };
  }

  return {
    kind: "candidate",
    mountRoot,
    filePath,
    endsWithSlash,
    decodedPathname: decoded,
  };
}
