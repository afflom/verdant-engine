/**
 * A WASI preview-1 host over a file system held in memory, read only, for
 * LexLean's front end as WebAssembly (lexlean-wasm.mjs). It imports nothing,
 * so the same host runs in Node on every operating system and in a browser
 * worker, the host LexLean's SPEC §8.3 names.
 *
 * Why not `node:wasi`: on Windows it does not list a directory (`fd_readdir`
 * is ENOSYS), and LexLean lists its language folders; and on Node 22 it must
 * run with `--no-turbo-fast-api-calls`. A host of our own has neither limit.
 *
 * The file system is a map of `/`-rooted paths to bytes; every directory is
 * implied by the files under it. Each file and directory has its own inode,
 * so LexLean's file-identity check (§25.1) holds. Every write but to stdout
 * and stderr is refused with EROFS: `check` and `snapshot` write nothing.
 */
const E = Object.freeze({ SUCCESS: 0, BADF: 8, INVAL: 28, ISDIR: 31, NOENT: 44, NOSYS: 52, NOTDIR: 54, ROFS: 69, NOTCAPABLE: 76 });
const FILETYPE = Object.freeze({ CHARACTER_DEVICE: 2, DIRECTORY: 3, REGULAR_FILE: 4 });
const ALL_RIGHTS = 0x1fffffffn;
const enc = new TextEncoder();
const dec = new TextDecoder();

/** The process ended through `proc_exit`: its code. */
export class WasiExit extends Error {
  constructor(code) {
    super(`exit ${code}`);
    this.code = code;
  }
}

/** The tree of a map of `path -> bytes`: each node `{ ino, dir, children | bytes }`. */
function treeOf(files) {
  let ino = 1n;
  const root = { ino: ino++, dir: true, children: new Map() };
  for (const [path, bytes] of files) {
    const parts = path.split('/').filter(Boolean);
    let at = root;
    for (const [i, part] of parts.entries()) {
      if (i === parts.length - 1) at.children.set(part, { ino: ino++, dir: false, bytes });
      else {
        if (!at.children.has(part)) at.children.set(part, { ino: ino++, dir: true, children: new Map() });
        at = at.children.get(part);
      }
    }
  }
  return root;
}

/**
 * Run a WASI command module. `files` maps `/`-rooted paths to bytes; `args`
 * are the arguments after the program name. Resolves `{ exit, stdout, stderr }`
 * with stdout and stderr as bytes. `module` is a compiled `WebAssembly.Module`,
 * which a caller keeps to run it again without compiling it again.
 */
export async function runWasi(module, { files, args = [], env = {}, program = 'lexlean' }) {
  const root = treeOf(files);
  const out = { 1: [], 2: [] };
  // Open descriptors: 0-2 are the standard streams, 3 the preopened root.
  const fds = new Map([[3, { node: root, path: '/', cursor: 0 }]]);
  let next = 4;
  let memory;
  const view = () => new DataView(memory.buffer);
  const bytesAt = (ptr, len) => new Uint8Array(memory.buffer, ptr, len);
  const argv = [program, ...args].map((a) => enc.encode(`${a}\0`));
  const envv = Object.entries(env).map(([k, v]) => enc.encode(`${k}=${v}\0`));

  const resolve = (base, path) => {
    const parts = base.path.split('/').filter(Boolean);
    for (const part of path.split('/')) {
      if (part === '' || part === '.') continue;
      if (part === '..') {
        if (parts.length === 0) return null;
        parts.pop();
      } else parts.push(part);
    }
    let node = root;
    for (const part of parts) {
      if (!node.dir) return { error: E.NOTDIR };
      node = node.children.get(part);
      if (!node) return { error: E.NOENT };
    }
    return { node, path: `/${parts.join('/')}` };
  };
  const writeStat = (ptr, node) => {
    const v = view();
    v.setBigUint64(ptr, 1n, true);
    v.setBigUint64(ptr + 8, node.ino, true);
    v.setUint8(ptr + 16, node.dir ? FILETYPE.DIRECTORY : FILETYPE.REGULAR_FILE);
    v.setBigUint64(ptr + 24, 1n, true);
    v.setBigUint64(ptr + 32, BigInt(node.dir ? 0 : node.bytes.length), true);
    for (const at of [40, 48, 56]) v.setBigUint64(ptr + at, 0n, true);
  };
  const lists = (strings, ptrs, buf) => {
    const v = view();
    let at = buf;
    strings.forEach((s, i) => {
      v.setUint32(ptrs + i * 4, at, true);
      bytesAt(at, s.length).set(s);
      at += s.length;
    });
    return E.SUCCESS;
  };
  const sizes = (strings, countPtr, sizePtr) => {
    view().setUint32(countPtr, strings.length, true);
    view().setUint32(sizePtr, strings.reduce((n, s) => n + s.length, 0), true);
    return E.SUCCESS;
  };

  const wasi = {
    args_sizes_get: (c, s) => sizes(argv, c, s),
    args_get: (p, b) => lists(argv, p, b),
    environ_sizes_get: (c, s) => sizes(envv, c, s),
    environ_get: (p, b) => lists(envv, p, b),
    clock_time_get: (_id, _precision, ptr) => {
      view().setBigUint64(ptr, BigInt(Date.now()) * 1000000n, true);
      return E.SUCCESS;
    },
    random_get: (ptr, len) => {
      globalThis.crypto.getRandomValues(bytesAt(ptr, len));
      return E.SUCCESS;
    },
    sched_yield: () => E.SUCCESS,
    proc_exit: (code) => {
      throw new WasiExit(code);
    },
    fd_prestat_get: (fd, ptr) => {
      if (fd !== 3) return E.BADF;
      view().setUint8(ptr, 0);
      view().setUint32(ptr + 4, 1, true);
      return E.SUCCESS;
    },
    fd_prestat_dir_name: (fd, ptr, len) => {
      if (fd !== 3 || len < 1) return E.BADF;
      bytesAt(ptr, 1).set(enc.encode('/'));
      return E.SUCCESS;
    },
    fd_fdstat_get: (fd, ptr) => {
      const v = view();
      const open = fds.get(fd);
      if (fd > 2 && !open) return E.BADF;
      v.setUint8(ptr, fd <= 2 ? FILETYPE.CHARACTER_DEVICE : open.node.dir ? FILETYPE.DIRECTORY : FILETYPE.REGULAR_FILE);
      v.setUint16(ptr + 2, 0, true);
      v.setBigUint64(ptr + 8, ALL_RIGHTS, true);
      v.setBigUint64(ptr + 16, ALL_RIGHTS, true);
      return E.SUCCESS;
    },
    fd_filestat_get: (fd, ptr) => {
      const open = fds.get(fd);
      if (!open) return fd <= 2 ? E.NOTCAPABLE : E.BADF;
      writeStat(ptr, open.node);
      return E.SUCCESS;
    },
    path_filestat_get: (fd, _flags, path, len, ptr) => {
      const base = fds.get(fd);
      if (!base) return E.BADF;
      const found = resolve(base, dec.decode(bytesAt(path, len)));
      if (!found) return E.NOTCAPABLE;
      if (found.error) return found.error;
      writeStat(ptr, found.node);
      return E.SUCCESS;
    },
    path_open: (fd, _dirflags, path, len, oflags, _rightsBase, _rightsInheriting, _fdflags, opened) => {
      const base = fds.get(fd);
      if (!base) return E.BADF;
      // O_CREAT, O_EXCL and O_TRUNC change the file system, which is read only.
      if (oflags & 0b1101) return E.ROFS;
      const found = resolve(base, dec.decode(bytesAt(path, len)));
      if (!found) return E.NOTCAPABLE;
      if (found.error) return found.error;
      if (oflags & 0b10 && !found.node.dir) return E.NOTDIR;
      const n = next++;
      fds.set(n, { node: found.node, path: found.path, cursor: 0 });
      view().setUint32(opened, n, true);
      return E.SUCCESS;
    },
    fd_read: (fd, iovs, count, nread) => {
      const open = fds.get(fd);
      if (fd === 0) {
        view().setUint32(nread, 0, true);
        return E.SUCCESS;
      }
      if (!open) return E.BADF;
      if (open.node.dir) return E.ISDIR;
      let total = 0;
      for (let i = 0; i < count; i++) {
        const v = view();
        const buf = v.getUint32(iovs + i * 8, true);
        const want = v.getUint32(iovs + i * 8 + 4, true);
        const chunk = open.node.bytes.subarray(open.cursor, open.cursor + want);
        bytesAt(buf, chunk.length).set(chunk);
        open.cursor += chunk.length;
        total += chunk.length;
        if (chunk.length < want) break;
      }
      view().setUint32(nread, total, true);
      return E.SUCCESS;
    },
    fd_write: (fd, iovs, count, nwritten) => {
      if (fd !== 1 && fd !== 2) return fds.has(fd) ? E.ROFS : E.BADF;
      let total = 0;
      for (let i = 0; i < count; i++) {
        const v = view();
        const buf = v.getUint32(iovs + i * 8, true);
        const len = v.getUint32(iovs + i * 8 + 4, true);
        out[fd].push(bytesAt(buf, len).slice());
        total += len;
      }
      view().setUint32(nwritten, total, true);
      return E.SUCCESS;
    },
    fd_readdir: (fd, buf, len, cookie, used) => {
      const open = fds.get(fd);
      if (!open) return E.BADF;
      if (!open.node.dir) return E.NOTDIR;
      const entries = [...open.node.children.entries()].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0));
      let at = 0;
      for (let i = Number(cookie); i < entries.length && at < len; i++) {
        const [name, node] = entries[i];
        const nameBytes = enc.encode(name);
        const header = new Uint8Array(24);
        const h = new DataView(header.buffer);
        h.setBigUint64(0, BigInt(i + 1), true);
        h.setBigUint64(8, node.ino, true);
        h.setUint32(16, nameBytes.length, true);
        h.setUint8(20, node.dir ? FILETYPE.DIRECTORY : FILETYPE.REGULAR_FILE);
        const entry = new Uint8Array(24 + nameBytes.length);
        entry.set(header);
        entry.set(nameBytes, 24);
        // A buffer that cannot hold the whole entry takes what fits, and the caller asks again with a larger one.
        const take = Math.min(entry.length, len - at);
        bytesAt(buf + at, take).set(entry.subarray(0, take));
        at += take;
      }
      view().setUint32(used, at, true);
      return E.SUCCESS;
    },
    fd_close: (fd) => (fds.delete(fd) || fd <= 2 ? E.SUCCESS : E.BADF),
    fd_sync: () => E.SUCCESS,
    path_create_directory: () => E.ROFS,
    path_link: () => E.ROFS,
    path_readlink: () => E.INVAL,
    path_remove_directory: () => E.ROFS,
    path_rename: () => E.ROFS,
    path_unlink_file: () => E.ROFS,
  };

  const instance = await WebAssembly.instantiate(module, { wasi_snapshot_preview1: wasi });
  memory = instance.exports.memory;
  let exit = 0;
  try {
    instance.exports._start();
  } catch (error) {
    if (!(error instanceof WasiExit)) throw error;
    exit = error.code;
  }
  const join = (chunks) => {
    const all = new Uint8Array(chunks.reduce((n, c) => n + c.length, 0));
    let at = 0;
    for (const c of chunks) {
      all.set(c, at);
      at += c.length;
    }
    return all;
  };
  return { exit, stdout: join(out[1]), stderr: join(out[2]) };
}
