/**
 * Lean browser/worker entry for the locked prism-code 0.11.1 runtime.
 * Verifies and executes Hologram v4, v5, and v6 (.holo) archives and closures
 * without importing Node/CLI/OPA container namespaces into the cold-boot frame.
 */
import { asBytes, canonicalJson, parseArchive, resolveArchive } from './archive.mjs';
import { blake3, blake3Hex, kappa } from './blake3.mjs';
import {
  CAPABILITIES_SCHEMA,
  CAPABILITY_KINDS,
  DEFAULT_SET,
  grantOf,
  parseCapabilities,
} from './capabilities.mjs';
import {
  CORE_CONTRACT,
  checkRequest,
  compileGuest,
  instantiateGuest,
  runAcceptance,
  runGuest,
  runInstance,
} from './core.mjs';
import {
  CapabilityDeniedError,
  DigestMismatchError,
  PrismCodeError,
  ReferenceInvalidError,
  fail,
} from './errors.mjs';
import {
  CHECKPOINT_CONTRACT,
  SESSION_CONTRACT,
  checkpointBytes,
  isCheckpointContract,
  isSessionContract,
  openSession,
  readState,
  restoreState,
  runSessionAcceptance,
  runStep,
} from './session.mjs';
import { decodeHoloview, encodeHoloview } from './view.mjs';

export {
  CAPABILITIES_SCHEMA,
  CAPABILITY_KINDS,
  CHECKPOINT_CONTRACT,
  CORE_CONTRACT,
  CapabilityDeniedError,
  DEFAULT_SET,
  DigestMismatchError,
  PrismCodeError,
  ReferenceInvalidError,
  SESSION_CONTRACT,
  blake3,
  blake3Hex,
  canonicalJson,
  checkpointBytes,
  compileGuest,
  decodeHoloview,
  encodeHoloview,
  fail,
  grantOf,
  kappa,
  parseArchive,
  parseCapabilities,
  readState,
  resolveArchive,
  restoreState,
  runAcceptance,
  runGuest,
  runSessionAcceptance,
  runStep,
};

const decoder = new TextDecoder('utf-8', { fatal: true });

function limitsOf(application) {
  return {
    request: application.request_maximum,
    response: application.response_maximum,
    guestAllocation: application.guest_allocation_maximum,
  };
}

function vectorsOf(application) {
  const list = Array.isArray(application.acceptance_vectors) ? application.acceptance_vectors : [];
  return list.map((v) => ({ request: Uint8Array.from(v.request), response: Uint8Array.from(v.response) }));
}

function sessionLimitsOf(application) {
  const session = application.session ?? {};
  return {
    request: application.request_maximum,
    guestAllocation: application.request_maximum,
    response: session.response_maximum ?? application.response_maximum,
    responseCount: session.response_count_maximum,
    state: session.state_maximum,
  };
}

function sessionsOf(application) {
  const list = Array.isArray(application.acceptance_sessions) ? application.acceptance_sessions : [];
  return list.map((steps) =>
    steps.map((s) => ({ request: Uint8Array.from(s.request), responses: s.responses.map((r) => Uint8Array.from(r)) })),
  );
}

function profileOf(modelDocument) {
  return modelDocument.application?.profile ?? modelDocument.schema ?? null;
}

function evidenceOf(provenance) {
  return {
    leanManifestSha256: provenance.lean_manifest_sha256 ?? null,
    lcnfManifestSha256: provenance.lcnf_manifest_sha256 ?? null,
    lexleanCommit: provenance.lexlean_commit ?? null,
    lean4ProdCommit: provenance.lean4_prod_commit ?? null,
    hologramLiveCommit: provenance.hologram_live_commit ?? null,
    uorHologramCommit: provenance.uor_hologram_commit ?? null,
    generatedCoreSha256: provenance.generated_core_sha256 ?? null,
    compilerSemanticsId: provenance.compiler_semantics_id ?? null,
    stdlibSemanticsId: provenance.stdlib_semantics_id ?? null,
  };
}

function grantFor({ capabilities = null, capabilitySet = DEFAULT_SET } = {}) {
  if (capabilities === null || capabilities === undefined) {
    if (capabilitySet !== DEFAULT_SET) fail('capabilities.invalid', `capability set ${capabilitySet} was named, but no capability file was given`);
    return grantOf(null);
  }
  return grantOf(parseCapabilities(capabilities), capabilitySet);
}

function worldWithinGrant(directory) {
  const contract = directory?.layers?.find((l) => l.kind === 'wasm')?.contract ?? null;
  if (contract === null) return Object.freeze({ contract: null, within: null, pure: null, effects: null, session: false });
  const pureCore = contract === CORE_CONTRACT || isSessionContract(contract);
  return Object.freeze({
    contract,
    within: pureCore ? true : null,
    pure: pureCore ? true : null,
    effects: pureCore ? Object.freeze([]) : null,
    session: isSessionContract(contract),
  });
}

function unverifiedArchive(parsed, identities) {
  if (identities.verified) return null;
  const problems = [];
  if (!parsed.verification.footer) problems.push('the footer is not the BLAKE3 of the body');
  if (!parsed.verification.labels) problems.push('a content blob does not hash to its label');
  if (!identities.consistent) problems.push('the manifest, directory and provenance do not name the same blobs');
  return `integrity failure: the holo was altered: ${problems.join('; ')}`;
}

function refuseUnverified(problem, { allowUnverified = false } = {}) {
  if (problem === null || allowUnverified === true) return;
  throw new DigestMismatchError(`${problem}; to open it anyway, pass allowUnverified: true (--allow-unverified on the command line)`, { check: 'holo.integrity' });
}

export async function loadArchive(source, options = {}) {
  const parsed = parseArchive(asBytes(source));
  if (!parsed.verification.footer) refuseUnverified('integrity failure: the holo was altered: the footer is not the BLAKE3 of the body', options);
  const resolved = resolveArchive(parsed);
  const { modelDocument, provenance, directory } = resolved;
  const application = modelDocument.application ?? fail('archive.model', 'model document has no application');
  const capabilities = grantFor(options);
  const world = worldWithinGrant(resolved.directory);
  const module = await compileGuest(resolved.guest.content, capabilities, world.contract ?? undefined);
  const limits = world.session ? sessionLimitsOf(application) : limitsOf(application);
  const vectors = world.session ? [] : vectorsOf(application);
  const sessions = world.session ? sessionsOf(application) : [];
  const view = decodeHoloview(resolved.view.content);
  let metadata = null;
  if (parsed.metadata) {
    try {
      metadata = JSON.parse(parsed.metadata.text);
    } catch {
      metadata = parsed.metadata.text;
    }
  }
  refuseUnverified(unverifiedArchive(parsed, resolved.identities), options);
  const dispatch = async (input) => {
    if (world.session) fail('session.required', `the world ${world.contract} keeps state across steps; open a session with app.open() and dispatch on it`);
    const request = asBytes(input);
    checkRequest(request, limits);
    const instance = instantiateGuest(module, {});
    return runInstance(instance, request);
  };
  const checkpointing = isCheckpointContract(world.contract);
  const makeSession = (state) =>
    openSession({
      instantiate: async () => {
        const instance = instantiateGuest(module, {});
        return state === null ? instance : restoreState(instance, state);
      },
      check: (request) => checkRequest(request, limits),
      step: async (instance, input) => runStep(instance, asBytes(input), limits),
      checkpoint: checkpointing ? readState : null,
    });
  const open = () => {
    if (!world.session) fail('session.unsupported', `the world ${world.contract} is stateless; it has no sessions, so dispatch on the application`);
    return makeSession(null);
  };
  const openAt = async (address, stateSource) => {
    if (!checkpointing) fail('checkpoint.unsupported', `the world ${world.contract} does not export its state; only hologram:guest/core-wasm@3 restores a checkpoint`);
    return makeSession(await checkpointBytes(address, stateSource, limits));
  };
  return Object.freeze({
    kind: 'holo',
    name: application.name ?? provenance.cargo_name ?? null,
    model: modelDocument,
    profile: profileOf(modelDocument),
    limits,
    vectors,
    provenance: { model: provenance, browser: null },
    directory,
    metadata,
    identities: resolved.identities,
    evidence: evidenceOf(provenance),
    view,
    archive: parsed,
    capabilities,
    world,
    sessions,
    dispatch,
    open,
    openAt,
    attestations: () => [],
    acceptance: () => (world.session ? runSessionAcceptance(sessions, open) : runAcceptance(vectors, dispatch)),
  });
}

export async function loadClosure(files, importGlue, { model = null, capabilities = null, capabilitySet = DEFAULT_SET, allowUnverified = false } = {}) {
  const { closureDispatch, initializeGlue, verifyClosure } = await import('./closure.mjs');
  const grant = grantFor({ capabilities, capabilitySet });
  const { stem, glueName, wasmName, provenance, verification } = await verifyClosure(files);
  if (!verification.verified) {
    refuseUnverified(`the closure's files do not match its provenance: ${verification.mismatches.map((m) => m.field).join(', ')}`, { allowUnverified });
  }
  const glue = initializeGlue(await importGlue(glueName), files.get(wasmName));
  const modelDocument = model ? (model instanceof Uint8Array ? JSON.parse(decoder.decode(model)) : model) : null;
  const application = modelDocument?.application ?? null;
  const limits = application ? limitsOf(application) : null;
  const vectors = application ? vectorsOf(application) : [];
  const profile = modelDocument ? profileOf(modelDocument) : typeof glue.invoke_bytes === 'function' ? 'text' : 'numeric';
  const dispatch = closureDispatch(glue, { profile, model: modelDocument, limits });
  return Object.freeze({
    kind: 'closure',
    name: stem,
    model: modelDocument,
    profile,
    limits,
    vectors,
    provenance: { model: null, browser: provenance },
    directory: null,
    metadata: null,
    identities: { ...verification, stem, glueName, wasmName },
    evidence: {
      modelId: provenance.model_id ?? null,
      viewModelId: provenance.view_model_id ?? null,
      generatedCoreSha256: provenance.generated_core_sha256 ?? null,
    },
    view: { entry: 'index.html', files },
    capabilities: grant,
    world: Object.freeze({ contract: null, within: null, pure: null, effects: null }),
    dispatch: async (request) => dispatch(asBytes(request)),
    attestations: () => [],
    acceptance: () => runAcceptance(vectors, (r) => dispatch(asBytes(r))),
  });
}

export async function load(source, options = {}) {
  if (source instanceof Uint8Array || source instanceof ArrayBuffer || ArrayBuffer.isView(source)) {
    return loadArchive(source, options);
  }
  if (typeof Blob !== 'undefined' && source instanceof Blob) {
    return loadArchive(new Uint8Array(await source.arrayBuffer()), options);
  }
  if (typeof source === 'string' || source instanceof URL) {
    const browser = await import('./browser.mjs');
    return browser.load(source, options);
  }
  fail('load.source', 'load() takes bytes, a Blob, a path or a URL');
}
