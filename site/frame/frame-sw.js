// Verdant Composable Blank Frame — ServiceWorker Virtual FS (`site/frame/frame-sw.js`)
// Serves cryptographically verified lifted assets from the recovered `.holo` capsule
// under `<scope>app/<path>` with zero loose UI files required on the hosting origin.

const CACHE_NAME = "verdant-stitch-vfs-v1";
const vfsMap = new Map();

self.addEventListener("install", (event) => {
  event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener("message", (event) => {
  const msg = event.data;
  if (!msg || typeof msg !== "object") return;
  const reply = (payload) => {
    if (event.ports && event.ports[0]) event.ports[0].postMessage(payload);
  };
  if (msg.type === "MOUNT_CAPSULE_VFS" && Array.isArray(msg.files)) {
    event.waitUntil(
      (async () => {
        vfsMap.clear();
        const cache = await caches.open(CACHE_NAME);
        const oldKeys = await cache.keys();
        await Promise.all(oldKeys.map((k) => cache.delete(k)));
        const base = new URL("app/", self.registration.scope).href;
        for (const entry of msg.files) {
          const clean = String(entry.path || "").replace(/^\/+/, "");
          if (!clean || clean.includes("..")) continue;
          const bytes = entry.bytes instanceof Uint8Array ? entry.bytes : new Uint8Array(entry.bytes);
          const mime = entry.mimeType || "application/octet-stream";
          const url = new URL(clean, base).href;
          vfsMap.set(url, { bytes, mime });
          await cache.put(
            url,
            new Response(bytes, {
              status: 200,
              headers: {
                "Content-Type": mime,
                "X-Verdant-Stitch-Vfs": "verified",
                "Cache-Control": "no-store",
              },
            }),
          );
        }
        reply({ ok: true, mountedFiles: vfsMap.size, scope: self.registration.scope });
      })().catch((err) => reply({ ok: false, error: String(err) })),
    );
  } else if (msg.type === "VFS_STATUS") {
    reply({ ok: true, mountedFiles: vfsMap.size, scope: self.registration.scope });
  }
});

self.addEventListener("fetch", (event) => {
  const reqUrl = new URL(event.request.url);
  const appBase = new URL("app/", self.registration.scope);
  if (reqUrl.origin !== appBase.origin || !reqUrl.pathname.startsWith(appBase.pathname)) {
    return;
  }
  event.respondWith(
    (async () => {
      let lookupUrl = `${reqUrl.origin}${reqUrl.pathname}`;
      if (lookupUrl.endsWith("/")) lookupUrl += "index.html";
      const mem = vfsMap.get(lookupUrl);
      if (mem) {
        return new Response(mem.bytes, {
          status: 200,
          headers: {
            "Content-Type": mem.mime,
            "X-Verdant-Stitch-Vfs": "memory-verified",
            "Cache-Control": "no-store",
          },
        });
      }
      const cache = await caches.open(CACHE_NAME);
      const cached = await cache.match(lookupUrl);
      if (cached) return cached;
      return new Response(`Verdant Stitch Virtual FS: verified path not found (${reqUrl.pathname})`, {
        status: 404,
        headers: { "Content-Type": "text/plain; charset=utf-8" },
      });
    })(),
  );
});
