/**
 * The public proof store: static, content-addressed files anyone can GET, in
 * the same layout as the local store (`kappa-store.mjs`): `derived/<key>` holds
 * the label a derivation key led to, and `bytes/<label>` the bytes. It is read
 * through the serverless boundary (`hosts.mjs`), so it lives on GitHub, for
 * example a repository at a pinned commit, and nowhere else.
 *
 * Nothing it says is trusted. Bytes are kept only when they re-derive to the
 * label asked for, a term is reused only under `termReusable`, and the kernel
 * judges every term in the build that uses it (model/canonical-build). The
 * store saves producing a term, never judging one. It is only read here:
 * publishing to a shared store is a separate step the user approves.
 */
import { isKappa, kappa } from '../uorc.mjs';
import { buildFetch } from './hosts.mjs';

const dec = new TextDecoder('utf-8', { fatal: true });
const path = (kind, label) => `${kind}/${label.slice(7, 9)}/${label.slice(9)}`;

/** The public store at `base`, a URL ending in `/`. `bound` and `get` resolve null for anything absent or not what it is named. */
export function publicStore({ base, fetch = globalThis.fetch, optIn = [] }) {
  if (!base.endsWith('/')) throw new Error(`${base} must end in /`);
  const get = buildFetch({ fetch, optIn });
  const read = async (url) => {
    const response = await get(url);
    return response.ok ? new Uint8Array(await response.arrayBuffer()) : null;
  };
  return {
    base,
    async bound(key) {
      if (!isKappa(key)) return null;
      const bytes = await read(new URL(path('derived', key), base).href);
      if (!bytes) return null;
      const label = dec.decode(bytes).trim();
      return isKappa(label) ? label : null;
    },
    async get(label) {
      if (!isKappa(label)) return null;
      const bytes = await read(new URL(path('bytes', label), base).href);
      return bytes && kappa(bytes) === label ? bytes : null;
    },
  };
}

/** The store named by PRISM_PROOF_STORE, or null: there is no default, since which store to trust with reuse is the user's. */
export const configuredStore = ({ env = process.env, fetch = globalThis.fetch } = {}) => (env.PRISM_PROOF_STORE ? publicStore({ base: env.PRISM_PROOF_STORE, fetch }) : null);

/**
 * Copy into `local` the proofs `remote` holds for `keys` that `local` does not:
 * the record and its term, each kept only when its bytes re-derive. Resolves
 * how many were copied. Whether one is then reused is `termReusable`'s to say.
 */
export async function pullProofs(local, remote, keys) {
  let pulled = 0;
  for (const key of keys) {
    if (local.bound(key)) continue;
    const label = await remote.bound(key);
    const record = label && (await remote.get(label));
    if (!record) continue;
    let bundle;
    try {
      bundle = JSON.parse(dec.decode(record)).bundle;
    } catch {
      continue;
    }
    const term = isKappa(bundle) ? await remote.get(bundle) : null;
    if (!term) continue;
    local.put(term);
    local.put(record);
    local.bind(key, label);
    pulled += 1;
  }
  return pulled;
}
