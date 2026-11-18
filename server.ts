import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { readFile, stat } from "node:fs/promises";
import { createServer, type IncomingMessage, type ServerResponse } from "node:http";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { gzipSync } from "node:zlib";
import { contentTypeFor, resolveMountTarget, shouldGzip } from "./server-routes.mjs";

const rootDir = fileURLToPath(new URL("./", import.meta.url));
const port = Number(process.env.PORT) || 3000;
const host = process.env.HOST || "0.0.0.0";

function ensureSiteAssembled(): void {
  const indexHtml = join(rootDir, "site", "index.html");
  const workerRuntime = join(rootDir, "site", "worker-runtime", "browser.mjs");
  const precacheJson = join(rootDir, "site", "precache.json");
  if (!existsSync(indexHtml) || !existsSync(workerRuntime) || !existsSync(precacheJson)) {
    execFileSync(process.execPath, [join(rootDir, "scripts", "build-site.mjs")], {
      cwd: rootDir,
      stdio: "inherit",
    });
  }
}

async function handleRequest(req: IncomingMessage, res: ServerResponse): Promise<void> {
  const method = req.method ?? "GET";
  if (method !== "GET" && method !== "HEAD") {
    res.writeHead(405, {
      "content-type": "text/plain; charset=utf-8",
      allow: "GET, HEAD",
    });
    res.end("Method Not Allowed");
    return;
  }

  let parsedUrl: URL;
  try {
    parsedUrl = new URL(req.url ?? "/", `http://${req.headers.host ?? "localhost"}`);
  } catch {
    res.writeHead(400, { "content-type": "text/plain; charset=utf-8" });
    res.end("Bad Request");
    return;
  }

  if (parsedUrl.pathname === "/api/health") {
    const payload = JSON.stringify({ ok: true, service: "verdant-engine" });
    res.writeHead(200, {
      "content-type": "application/json; charset=utf-8",
      "cache-control": "no-store",
    });
    res.end(method === "HEAD" ? undefined : payload);
    return;
  }

  const target = resolveMountTarget(rootDir, parsedUrl.pathname);
  if (target.kind === "bad-request") {
    res.writeHead(400, { "content-type": "text/plain; charset=utf-8" });
    res.end("Bad Request");
    return;
  }
  if (target.kind === "forbidden") {
    res.writeHead(403, { "content-type": "text/plain; charset=utf-8" });
    res.end("Forbidden");
    return;
  }

  let fileToServe = target.filePath;
  try {
    const info = await stat(fileToServe);
    if (info.isDirectory()) {
      if (!target.endsWithSlash) {
        res.writeHead(301, { location: `${parsedUrl.pathname}/${parsedUrl.search}` });
        res.end();
        return;
      }
      fileToServe = join(fileToServe, "index.html");
      const indexInfo = await stat(fileToServe);
      if (!indexInfo.isFile()) {
        res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
        res.end("Not Found");
        return;
      }
    } else if (!info.isFile()) {
      res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
      res.end("Not Found");
      return;
    }
  } catch {
    res.writeHead(404, { "content-type": "text/plain; charset=utf-8" });
    res.end("Not Found");
    return;
  }

  const rawBody = await readFile(fileToServe);
  const acceptEncoding = String(req.headers["accept-encoding"] ?? "");
  const useGzip = shouldGzip(fileToServe, acceptEncoding, rawBody.byteLength);
  const payload = useGzip ? gzipSync(rawBody) : rawBody;

  const headers: Record<string, string | number> = {
    "content-type": contentTypeFor(fileToServe),
    "content-length": payload.byteLength,
    "cache-control": "no-cache",
    vary: "Accept-Encoding",
  };
  if (useGzip) {
    headers["content-encoding"] = "gzip";
  }

  res.writeHead(200, headers);
  res.end(method === "HEAD" ? undefined : payload);
}

ensureSiteAssembled();

const server = createServer((req, res) => {
  handleRequest(req, res).catch((err) => {
    console.error("server error:", err);
    if (!res.headersSent) {
      res.writeHead(500, { "content-type": "text/plain; charset=utf-8" });
    }
    res.end("Internal Server Error");
  });
});

server.listen(port, host, () => {
  console.log(`Verdant Engine listening on http://${host}:${port}`);
});
