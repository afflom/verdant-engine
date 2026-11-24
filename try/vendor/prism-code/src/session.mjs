/**
 * Sessions of the session world `hologram:guest/core-wasm@2` (issue #629,
 * docs/proposals/629-session-world.md).
 *
 * One instance is one session. The guest keeps its state; the host never reads
 * or writes it. A step writes the request at the offset `holo_alloc` returns,
 * calls `holo_step`, and reads the responses the i64 locates: a big-endian u32
 * count, then each response as a big-endian u32 length and its bytes. Any
 * failure ends the session, so a session's outcomes are a function of its
 * ordered requests and a history can say so.
 */
import { fail } from './errors.mjs';
import { kappa } from './blake3.mjs';

export const SESSION_CONTRACT = 'hologram:guest/core-wasm@2';
export const SESSION_EXPORTS = Object.freeze(['holo_alloc', 'holo_step', 'memory']);
/** The checkpoint world (HO-47): a session world whose state the host can name and restore. */
export const CHECKPOINT_CONTRACT = 'hologram:guest/core-wasm@3';
export const CHECKPOINT_EXPORTS = Object.freeze(['holo_alloc', 'holo_restore', 'holo_state', 'holo_step', 'memory']);
const KAPPA = /^blake3:[0-9a-f]{64}$/;

/** True for a guest contract whose instances are sessions. */
export function isSessionContract(contract) {
  return contract === SESSION_CONTRACT || contract === CHECKPOINT_CONTRACT;
}

/** True for a guest contract whose session state can be checkpointed and restored. */
export function isCheckpointContract(contract) {
  return contract === CHECKPOINT_CONTRACT;
}

/** A copy of the resident state of a checkpoint-world instance, read through `holo_state`. */
export function readState(instance) {
  const { exports } = instance;
  const packed = BigInt.asUintN(64, exports.holo_state());
  const offset = Number(packed >> 32n);
  const length = Number(packed & 0xffffffffn);
  if (offset + length > exports.memory.buffer.byteLength) fail('core.response', `guest state ${offset}+${length} lies outside its memory`);
  return new Uint8Array(exports.memory.buffer.slice(offset, offset + length));
}

/** Install `state` as the resident state of a fresh checkpoint-world instance, through `holo_restore`. */
export function restoreState(instance, state) {
  const { exports } = instance;
  const at = exports.holo_restore(state.length) >>> 0;
  new Uint8Array(exports.memory.buffer, at, state.length).set(state);
  return instance;
}

/**
 * The state bytes a checkpoint names, checked before any instance exists: the
 * label must be a BLAKE3 kappa label, the bytes must fit the declared state
 * maximum when one is declared, and they must hash to the label. `source` is
 * the bytes themselves or a store with `get(address)`.
 */
export async function checkpointBytes(address, source, limits = {}) {
  if (typeof address !== 'string' || !KAPPA.test(address)) fail('checkpoint.address', `a checkpoint is named by a BLAKE3 kappa label, not ${JSON.stringify(address)}`);
  const found = source instanceof Uint8Array ? source : await source?.get?.(address);
  if (!(found instanceof Uint8Array)) fail('checkpoint.address', `no state bytes were given for ${address}`);
  const state = Uint8Array.from(found);
  if (limits.state !== undefined && state.length > limits.state) {
    fail('checkpoint.state', `the state of ${state.length} bytes is over the declared state maximum ${limits.state}`);
  }
  const actual = kappa(state);
  if (actual !== address) fail('checkpoint.address', `the state bytes hash to ${actual}, not ${address}`);
  return state;
}

/**
 * Decode the responses frame of one step from guest memory. Refuses with
 * `core.response` a frame that lies outside memory, has trailing bytes, or is
 * over a declared bound; the shim already traps on these, so this is the
 * host's own check of what it is handed.
 *
 * @param {ArrayBuffer} buffer the guest memory
 * @param {number} offset
 * @param {number} length
 * @param {{ response?: number, responseCount?: number }} [limits]
 * @returns {Uint8Array[]} copies of the responses, in order
 */
export function decodeResponses(buffer, offset, length, limits = {}) {
  if (offset + length > buffer.byteLength) fail('core.response', `guest responses ${offset}+${length} lie outside its memory`);
  const frame = new Uint8Array(buffer.slice(offset, offset + length));
  const view = new DataView(frame.buffer);
  if (frame.length < 4) fail('core.response', `the responses frame of ${frame.length} bytes has no count`);
  const count = view.getUint32(0);
  if (limits.responseCount !== undefined && count > limits.responseCount) {
    fail('core.response', `the step returned ${count} responses, more than the declared ${limits.responseCount}`);
  }
  const responses = [];
  let at = 4;
  for (let index = 0; index < count; index++) {
    if (at + 4 > frame.length) fail('core.response', `response ${index} of ${count} has no length`);
    const size = view.getUint32(at);
    at += 4;
    if (limits.response !== undefined && size > limits.response) {
      fail('core.response', `response ${index} has ${size} bytes, more than the declared ${limits.response}`);
    }
    if (at + size > frame.length) fail('core.response', `response ${index} runs past the end of the frame`);
    responses.push(frame.slice(at, at + size));
    at += size;
  }
  if (at !== frame.length) fail('core.response', `the responses frame has ${frame.length - at} bytes after its last response`);
  return responses;
}

/** Run one step of a session instance and return copies of its responses. */
export function runStep(instance, request, limits = {}) {
  const { exports } = instance;
  const at = exports.holo_alloc(request.length) >>> 0;
  new Uint8Array(exports.memory.buffer, at, request.length).set(request);
  const packed = BigInt.asUintN(64, exports.holo_step(at, request.length));
  return decodeResponses(exports.memory.buffer, Number(packed >> 32n), Number(packed & 0xffffffffn), limits);
}

/**
 * A session over one instance. `instantiate` makes the instance (the
 * middleware `instantiate` step) and `step` runs one request on it (the
 * middleware `dispatch` step); `check` refuses a request before the guest
 * sees it. Every failure ends the session.
 *
 * @param {{ instantiate: () => Promise<WebAssembly.Instance> | WebAssembly.Instance, step: (instance: WebAssembly.Instance, request: Uint8Array) => Promise<Uint8Array[]> | Uint8Array[], check?: (request: Uint8Array) => void }} parts
 */
export async function openSession({ instantiate, step, check = () => {}, checkpoint = null }) {
  let instance = await instantiate();
  let steps = 0;
  let busy = false;
  const end = () => {
    instance = null;
  };
  const dispatch = async (request) => {
    if (instance === null) fail('session.closed', `the session ended after ${steps} steps; open a new one`);
    if (busy) fail('session.busy', 'a step of this session is still running; steps of one session are taken in order');
    busy = true;
    try {
      check(request);
      const responses = await step(instance, request);
      steps++;
      return responses;
    } catch (error) {
      end();
      throw error;
    } finally {
      busy = false;
    }
  };
  return Object.freeze({
    /** Take one step. Resolves to its responses, in order. */
    dispatch,
    /** Take one step and yield its responses one at a time, after the step has run. */
    async *stream(request) {
      for (const response of await dispatch(request)) yield response;
    },
    /**
     * The state after the steps taken so far, as `{ address, state }`: the BLAKE3
     * kappa label of the state bytes and a copy of them (HO-48). Only a guest of
     * the checkpoint world exports its state; any other fails `checkpoint.unsupported`.
     */
    checkpoint() {
      if (checkpoint === null) fail('checkpoint.unsupported', 'only a guest of the checkpoint world hologram:guest/core-wasm@3 exports its state');
      if (instance === null) fail('session.closed', `the session ended after ${steps} steps; open a new one`);
      if (busy) fail('session.busy', 'a step of this session is still running; checkpoint after it');
      const state = checkpoint(instance);
      return Object.freeze({ address: kappa(state), state });
    },
    /** End the session. Later steps fail with `session.closed`. */
    close: end,
    get closed() {
      return instance === null;
    },
    get steps() {
      return steps;
    },
  });
}

function equalBytes(a, b) {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) if (a[i] !== b[i]) return false;
  return true;
}

/**
 * Run acceptance sessions, each on a fresh session, and compare every step's
 * responses byte for byte. A failed step ends its session; its remaining
 * steps are reported as failed with the error the runtime gives them.
 *
 * @param {{ request: Uint8Array, responses: Uint8Array[] }[][]} sessions
 * @param {() => Promise<object>} open
 */
export async function runSessionAcceptance(sessions, open) {
  const failed = [];
  let total = 0;
  for (const [session, steps] of sessions.entries()) {
    const running = await open();
    for (const [index, expected] of steps.entries()) {
      total++;
      let actual;
      try {
        actual = await running.dispatch(expected.request);
      } catch (error) {
        failed.push({ session, index, request: expected.request, expected: expected.responses, error });
        continue;
      }
      if (actual.length !== expected.responses.length || actual.some((r, i) => !equalBytes(r, expected.responses[i]))) {
        failed.push({ session, index, request: expected.request, expected: expected.responses, actual });
      }
    }
    running.close();
  }
  return { total, passed: total - failed.length, failed };
}
