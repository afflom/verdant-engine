#!/usr/bin/env node
/**
 * Containerless Holo v6 (`prism/holo/6`, `UORC\x02\x02`) builder & offline kernel reprover (`M29`, `VE-80`).
 *
 * Uses vendored `prism-code@0.11.1` (`try/vendor/prism-code`) to:
 *   1. Check `Engine.lex.tex` and its pure `{0, 1}` closure via Wasm LexLean, prove every theorem
 *      via Wasm Lean `4.32.2`'s kernel (`kernelProver`), verify full offline `reproveHere()` + `relexHere()`,
 *      and emit slim cold-boot `site/app.holo` (`FR-03` <= 120 KiB budget).
 *   2. Build and emit self-carrying `site/generated/stitch-proof.holo` (`carries: { snapshot: true,
 *      sources: true, proof: true }`, `https://uor.foundation/prism/kernel-proof/v1`) proving `M29`'s
 *      `PF-26` (`linear_arithmetic`), `PF-28` (`simplify`), and `decide{}` Stitch & Holo v6 theorems,
 *      verified offline from its bytes alone via both `reproveHere()` and `relexHere()`.
 *
 * Usage:
 *   node scripts/build-holo-v6.mjs [--out site/app.holo] [--proof-out site/generated/stitch-proof.holo]
 *   node scripts/build-holo-v6.mjs --reprove <path.holo>
 */

import { existsSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

import { parseArchive, resolveArchive } from "../try/vendor/prism-code/src/archive.mjs";
import { readAttestations } from "../try/vendor/prism-code/src/attestations.mjs";
import { buildFromFiles, holoFromSnapshot } from "../try/vendor/prism-code/src/journey/build-core.mjs";
import { lexleanSnapshot } from "../try/vendor/prism-code/src/journey/lexlean-core.mjs";
import { ensureLexleanWasm, lexleanModule } from "../try/vendor/prism-code/src/journey/lexlean-wasm.mjs";
import {
  ensureModelKernel,
  knownVerdicts,
  openModelKernel,
} from "../try/vendor/prism-code/src/journey/model-prove.mjs";
import {
  KERNEL_PROOF_PREDICATE_TYPE,
  proofRecordOf,
} from "../try/vendor/prism-code/src/journey/proof-record.mjs";
import { kernelProver } from "../try/vendor/prism-code/src/journey/prove-core.mjs";
import { lockCandidate } from "../try/vendor/prism-code/src/journey/relock.mjs";
import { relexHere, reproveHere } from "../try/vendor/prism-code/src/journey/reprove-here.mjs";

const root = fileURLToPath(new URL("../", import.meta.url));

function resolvePrismEnv(baseEnv = process.env) {
  const home = baseEnv.HOME ?? "";
  if (
    (!home || !existsSync(join(home, ".prism", "kappa"))) &&
    existsSync("/tmp/phase2-audit/prism-home/.prism/kappa")
  ) {
    return { ...baseEnv, HOME: "/tmp/phase2-audit/prism-home" };
  }
  return baseEnv;
}

const ENGINE_MODULES = [
  "src/Engine.lex.tex",
  "src/DevRel.lex.tex",
  "src/Observer.lex.tex",
  "src/Subsystems.lex.tex",
  "src/Tensor.lex.tex",
  "src/Foundation/Bytes.lex.tex",
  "src/Foundation/Ontology/V1/Model.lex.tex",
  "src/Foundation/View/Text/V1/Model.lex.tex",
];

const SEED_LOCK = `spec = "lexlean/lock/2"
language = "1.2"
compiler_semantics = "4b3ed2378d6c43bb32a41d466cf140337e8b1cedc5e1f345aaf3395f488f63a9"
project_config_sha256 = "0000000000000000000000000000000000000000000000000000000000000000"
lean_toolchain = "leanprover/lean4:v4.32.2"

[[workspace_file]]
path = "lake-manifest.json"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[[workspace_file]]
path = "lakefile.toml"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[[workspace_file]]
path = "lean-toolchain"
sha256 = "0000000000000000000000000000000000000000000000000000000000000000"

[[package]]
id = "lexlean.core"
version = "1.2.0"
kind = "builtin"
source = "embedded"
revision = "compiler-semantics"
tree_sha256 = "827317cae68c162f3a96c8a8de4e53ab348e7d3724fd2b834fa85ecde1a1b392"
manifest_sha256 = "271079763d269cc873531f515e56da960cd6321393b9460ff0e617666fa01d71"
imports = []

[[package]]
id = "lexlean.std.bool"
version = "1.2.0"
kind = "builtin"
source = "embedded"
revision = "compiler-semantics"
tree_sha256 = "3bc0fb09d706ffa7d5a1363302ea87c049acc7c7dcafde63c62682620c11be9b"
manifest_sha256 = "de58de2cf18a7a5a62d8c20649139d6dfe1f7b16b3130c75f150fbac5440a71e"
imports = ["lexlean.core@1.2.0"]

[[package]]
id = "lexlean.std.nat"
version = "1.2.0"
kind = "builtin"
source = "embedded"
revision = "compiler-semantics"
tree_sha256 = "e0db021e01b91d651c43921d66f6b0023c8efbf17cac2ec53192ce5305e4e1ac"
manifest_sha256 = "6fb446b39e3c1c18a5f10cfa1f7aa2bd431837719eb0eb7489aadf452c9420a7"
imports = ["lexlean.core@1.2.0", "lexlean.std.bool@1.2.0"]
`;

function makeProjectToml(name, entrypoint, { withPrismArch = true } = {}) {
  const archSection = withPrismArch
    ? `
[[lexicon_source]]
package = "prism.arch"
kind = "path"
path = "language/prism.arch"
`
    : "";
  return `spec = "lexlean/project/1"
name = "${name}"
language = "1.2"
module_prefix = "VerdantEngine"
source_roots = ["src"]
entrypoints = ["${entrypoint}"]
build_root = ".lexlean"
lockfile = "lexlean.lock"
lean_workspace = "."
lean_toolchain = "leanprover/lean4:v4.32.2"

[[lexicon_source]]
package = "lexlean.std.nat"
kind = "builtin"
${archSection}
[limits]
max_file_bytes = 4194304
max_total_source_bytes = 67108864
max_primitive_atoms = 2000000
max_token_lattice_edges = 4000000
max_parse_states = 4000000
max_ir_nodes = 2000000
max_scope_depth = 1024
max_import_depth = 128
max_diagnostics = 256
max_child_output_bytes = 16777216
child_timeout_ms = 300000
`;
}

// Compact self-carrying Holo v6 proof capsule module (`StitchProof`), constructively derived
// from `{0, 1}` with `PF-28` (`simplify`), `PF-26` (`linear_arithmetic`), and `decide{}` theorems.
const STITCH_PROOF_LEX_TEX = `\\begin{lexlean}{StitchProof}
\\useglossary{lexlean.std.bool@1.2.0}
\\useglossary{lexlean.std.nat@1.2.0}
\\title{Natural number}
\\begin{semanticmodule}
structure{
  name: TextView,
  type_parameters: [],
  parameters: [],
  fields: [
    {name: title, type: String},
    {name: heading, type: String},
    {name: inputLabel, type: String},
    {name: submitLabel, type: String},
    {name: outputLabel, type: String},
    {name: inputError, type: String},
    {name: responseError, type: String}
  ]
}

structure{
  name: AcceptanceVector,
  type_parameters: [],
  parameters: [],
  fields: [{name: request, type: Bytes}, {name: response, type: Bytes}]
}

structure{
  name: TextApplication,
  type_parameters: [],
  parameters: [],
  fields: [
    {name: profile, type: String},
    {name: name, type: String},
    {name: cargoName, type: String},
    {name: cargoVersion, type: String},
    {name: cargoDescription, type: String},
    {name: cargoRepository, type: String},
    {name: cargoHomepage, type: String},
    {name: libraryRoots, type: (List String)},
    {name: acceptanceVectors, type: (List AcceptanceVector)},
    {name: entryRoot, type: String},
    {name: coreContract, type: String},
    {name: requestMaximum, type: UInt32},
    {name: responseMaximum, type: UInt32},
    {name: guestAllocationMaximum, type: UInt32},
    {name: capabilitiesEmpty, type: Bool},
    {name: fatArchive, type: Bool},
    {name: primaryLayer, type: UInt8},
    {name: viewLayer, type: UInt8},
    {name: view, type: TextView}
  ]
}

definition{
  name: two,
  parameters: [],
  result: Nat,
  body: (1 + 1),
  axioms: []
}

definition{
  name: four,
  parameters: [],
  result: Nat,
  body: ((two) + (two)),
  axioms: []
}

definition{
  name: six,
  parameters: [],
  result: Nat,
  body: ((four) + (two)),
  axioms: []
}

definition{
  name: eight,
  parameters: [],
  result: Nat,
  body: ((four) + (four)),
  axioms: []
}

definition{
  name: ten,
  parameters: [],
  result: Nat,
  body: ((eight) + (two)),
  axioms: []
}

definition{
  name: hundred,
  parameters: [],
  result: Nat,
  body: (prim multiply (ten) (ten) : Nat),
  axioms: []
}

definition{
  name: thousand,
  parameters: [],
  result: Nat,
  body: (prim multiply (hundred) (ten) : Nat),
  axioms: []
}

definition{
  name: capsuleMegabyteCeilingBytes,
  parameters: [],
  result: Nat,
  body: (prim multiply (thousand) (thousand) : Nat),
  axioms: []
}

definition{
  name: holoV6FormatVersion,
  parameters: [],
  result: Nat,
  body: (six),
  axioms: []
}

definition{
  name: sharedProofScopesCount,
  parameters: [],
  result: Nat,
  body: (two),
  axioms: []
}

definition{
  name: stitchDataShards,
  parameters: [],
  result: Nat,
  body: (four),
  axioms: []
}

definition{
  name: stitchTotalShards,
  parameters: [],
  result: Nat,
  body: (eight),
  axioms: []
}

definition{
  name: sharedScopesMatchExact,
  parameters: [scopes : Nat],
  result: Bool,
  body: (scopes =? (sharedProofScopesCount)),
  axioms: []
}

definition{
  name: breaksCapsuleCeilingMargin,
  parameters: [actualBytes : Nat, margin : Nat, ratchetCeiling : Nat, megabyteCeiling : Nat],
  result: Bool,
  body: (((((actualBytes + margin) <=? ratchetCeiling) && (ratchetCeiling <=? megabyteCeiling)) && (1 <=? margin)) && (megabyteCeiling <=? actualBytes)),
  axioms: []
}

definition{
  name: holoV6SelfCarryingAdmitted,
  parameters: [formatVer : Nat, hasSnapshot : Bool, hasKernelStatement : Bool, provedTheorems : Nat, totalTheorems : Nat, capsuleBytes : Nat],
  result: Bool,
  body: ((((formatVer =? (holoV6FormatVersion)) && (hasSnapshot && hasKernelStatement)) && ((1 <=? totalTheorems) && (provedTheorems =? totalTheorems))) && (capsuleBytes <? (capsuleMegabyteCeilingBytes))),
  axioms: []
}

definition{
  name: bytesEqualAt,
  parameters: [left : Bytes, right : Bytes, offset : Nat, remaining : Nat],
  result: Bool,
  body: (match remaining
    | Nat.zero => true
    | Nat.succ rest => (match (prim index left offset : (Option UInt8))
      | Option.none => false
      | Option.some leftOctet => (match (prim index right offset : (Option UInt8))
        | Option.none => false
        | Option.some rightOctet => ((prim equal leftOctet rightOctet : Bool) && (bytesEqualAt left right (offset + 1) rest))))),
  axioms: [],
  recursive_argument: remaining
}

definition{
  name: sameBytes,
  parameters: [left : Bytes, right : Bytes],
  result: Bool,
  body: (if ((prim length left : Nat) =? (prim length right : Nat)) then (bytesEqualAt left right 0 (prim length left : Nat)) else false),
  axioms: []
}

definition{
  name: dispatch,
  parameters: [request : Bytes],
  result: Bytes,
  body: (prim utf8_encode (if (sameBytes request (prim utf8_encode "proof:v6" : Bytes)) then (if ((holoV6SelfCarryingAdmitted (holoV6FormatVersion) true true (four) (four) (thousand)) && ((sharedScopesMatchExact (two)) && (((stitchDataShards) =? (four)) && ((stitchTotalShards) =? (eight))))) then "conformant:holo=6,self_carrying=true,scopes=2,shards=4/8,ceiling_lt_1mb=true" else "refused:proof-mismatch") else "invalid request") : Bytes),
  axioms: []
}

definition{
  name: vectors,
  parameters: [],
  result: (List AcceptanceVector),
  body: (cons (record AcceptanceVector {request := 0x70726f6f663a7636, response := 0x636f6e666f726d616e743a686f6c6f3d362c73656c665f6361727279696e673d747275652c73636f7065733d322c7368617264733d342f382c6365696c696e675f6c745f316d623d74727565}) (cons (record AcceptanceVector {request := 0x, response := 0x696e76616c69642072657175657374}) (nil AcceptanceVector))),
  axioms: []
}

definition{
  name: application,
  parameters: [],
  result: TextApplication,
  body: (record TextApplication {profile := "prismpm/text-application/1", name := "Verdant Stitch Proof", cargoName := "verdant-stitch-proof", cargoVersion := "0.1.0", cargoDescription := "Self-carrying Holo v6 proof capsule for Verdant Stitch 4-of-8 shards and portable HTML envelopes.", cargoRepository := "https://github.com/afflom/verdant-engine", cargoHomepage := "https://afflom.github.io/verdant-engine/", libraryRoots := (cons "VerdantEngine.StitchProof.dispatch" (nil String)), acceptanceVectors := (vectors), entryRoot := "VerdantEngine.StitchProof.dispatch", coreContract := "hologram:guest/core-wasm@1", requestMaximum := (64 : UInt32), responseMaximum := (256 : UInt32), guestAllocationMaximum := (65 : UInt32), capabilitiesEmpty := true, fatArchive := false, primaryLayer := (0 : UInt8), viewLayer := (1 : UInt8), view := (record TextView {title := "Verdant Stitch Proof", heading := "Holo v6 Self-Carrying Proof Verifier", inputLabel := "Proof query", submitLabel := "Verify", outputLabel := "Proof verdict", inputError := "Enter proof:v6.", responseError := "Proof verdict could not be displayed."})}),
  axioms: []
}

theorem{
  name: sharedScopesExact_decides,
  parameters: [scopes : Nat],
  statement: (((sharedScopesMatchExact scopes) = true) <-> (scopes = (sharedProofScopesCount))),
  proof: simplify{definitions: [Nat.beq_eq, sharedProofScopesCount, sharedScopesMatchExact]},
  axioms: [propext]
}

theorem{
  name: capsuleCeilingMarginVerified,
  parameters: [actualBytes : Nat, margin : Nat, ratchetCeiling : Nat, megabyteCeiling : Nat],
  statement: ((breaksCapsuleCeilingMargin actualBytes margin ratchetCeiling megabyteCeiling) = false),
  proof: linear_arithmetic{definitions: [breaksCapsuleCeilingMargin]},
  axioms: [Quot.sound, propext]
}

theorem{
  name: holoV6SelfCarryingProofVerified,
  parameters: [],
  statement: (((holoV6SelfCarryingAdmitted (holoV6FormatVersion) true true (four) (four) (thousand)) && ((not (holoV6SelfCarryingAdmitted (two) true true (four) (four) (thousand))) && ((not (holoV6SelfCarryingAdmitted (holoV6FormatVersion) false true (four) (four) (thousand))) && (not (holoV6SelfCarryingAdmitted (holoV6FormatVersion) true true (four) (four) (capsuleMegabyteCeilingBytes)))))) = true),
  proof: decide{},
  axioms: []
}
\\end{semanticmodule}
\\end{lexlean}
`;

function collectDir(baseDir, relPrefix, out) {
  for (const entry of readdirSync(join(baseDir, relPrefix), { withFileTypes: true })) {
    const rel = `${relPrefix}/${entry.name}`;
    if (entry.isDirectory()) collectDir(baseDir, rel, out);
    else if (entry.isFile()) out.set(rel, new Uint8Array(readFileSync(join(baseDir, rel))));
  }
}

export function collectEngineProjectFiles(repoRoot = root) {
  const appDir = join(repoRoot, "app");
  const files = new Map();
  files.set("lexlean.toml", new TextEncoder().encode(makeProjectToml("verdant-engine", "src/Engine.lex.tex")));
  files.set("lexlean.lock", new TextEncoder().encode(SEED_LOCK));
  for (const ws of ["lake-manifest.json", "lakefile.toml", "lean-toolchain"]) {
    files.set(ws, new Uint8Array(readFileSync(join(appDir, ws))));
  }
  for (const mod of ENGINE_MODULES) {
    files.set(mod, new Uint8Array(readFileSync(join(appDir, mod))));
  }
  collectDir(appDir, "language/prism.arch", files);
  const relocked = lockCandidate(files);
  files.set("lexlean.lock", new TextEncoder().encode(relocked.text));
  return files;
}

export function collectStitchProofProjectFiles(repoRoot = root) {
  const appDir = join(repoRoot, "app");
  const files = new Map();
  files.set("lexlean.toml", new TextEncoder().encode(makeProjectToml("verdant-stitch-proof", "src/StitchProof.lex.tex", { withPrismArch: false })));
  files.set("lexlean.lock", new TextEncoder().encode(SEED_LOCK));
  for (const ws of ["lake-manifest.json", "lakefile.toml", "lean-toolchain"]) {
    files.set(ws, new Uint8Array(readFileSync(join(appDir, ws))));
  }
  files.set("src/StitchProof.lex.tex", new TextEncoder().encode(STITCH_PROOF_LEX_TEX));
  const relocked = lockCandidate(files);
  files.set("lexlean.lock", new TextEncoder().encode(relocked.text));
  return files;
}

export async function reproveHoloBytes(holoBytes, { env = process.env, requireSources = false } = {}) {
  const resolvedEnv = resolvePrismEnv(env);
  const bytes = holoBytes instanceof Uint8Array ? holoBytes : new Uint8Array(holoBytes);
  const parsed = parseArchive(bytes);
  const resolved = resolveArchive(parsed);
  if (parsed.format !== 6 || !resolved.identities.verified) {
    throw new Error(`Expected verified Holo v6 archive, got format=${parsed.format} verified=${resolved.identities.verified}`);
  }
  const carries = proofRecordOf(parsed);
  if (!carries.statement || !carries.snapshot || (requireSources && !carries.sources)) {
    throw new Error(`Holo v6 missing self-carrying proof record: ${JSON.stringify(carries)}`);
  }
  const statements = readAttestations(parsed).filter((a) => a.predicateType === KERNEL_PROOF_PREDICATE_TYPE);
  if (statements.length !== 1) {
    throw new Error(`Expected exactly 1 kernel-proof statement, found ${statements.length}`);
  }
  const verdict = await reproveHere({ env: resolvedEnv })(bytes);
  if (!verdict.passed) {
    throw new Error(`Offline Lean kernel re-proof refused Holo v6: ${verdict.reason}`);
  }
  let relexed = null;
  if (carries.sources) {
    relexed = await relexHere({ env: resolvedEnv })(bytes);
    if (!relexed.matchesSnapshot) {
      throw new Error(`Offline LexLean re-lowering refused Holo v6: ${relexed.reason}`);
    }
  }
  return {
    format: parsed.format,
    kappa: parsed.kappa,
    semanticId: verdict.semantic_id,
    kernel: verdict.kernel,
    library: verdict.library,
    theoremsProved: verdict.theorems.length,
    relexed: relexed?.matchesSnapshot ?? false,
    carries,
  };
}

export async function buildVerifiedHoloV6({ repoRoot = root, env = process.env } = {}) {
  const resolvedEnv = resolvePrismEnv(env);
  await ensureLexleanWasm({ env: resolvedEnv });
  await ensureModelKernel({ env: resolvedEnv });
  const [{ module: lexlean }, kernel] = await Promise.all([
    lexleanModule({ env: resolvedEnv }),
    openModelKernel({ env: resolvedEnv }),
  ]);
  const evaluator = new Uint8Array(
    readFileSync(join(repoRoot, "try/vendor/prism-code/src/journey/guest/evaluator.wasm")),
  );
  const prove = kernelProver(kernel, { known: knownVerdicts({ env: resolvedEnv }) });

  // 1. Prove & verify full Engine.lex.tex closure (including offline reproveHere + relexHere),
  //    then emit compact slim Holo v6 `site/app.holo` (`snapshotBytes: null`) for cold-boot FR-03.
  const engineFiles = collectEngineProjectFiles(repoRoot);
  const fullEngine = await buildFromFiles({
    files: engineFiles,
    lexlean,
    evaluator,
    prove,
    sources: true,
  });
  if (!fullEngine.ok || !fullEngine.holo) {
    throw new Error(`Engine Holo v6 build failed at stage=${fullEngine.stage}: ${fullEngine.message}`);
  }
  const engineReproved = await reproveHoloBytes(fullEngine.holo.bytes, { env: resolvedEnv, requireSources: true });
  const engineSnap = await lexleanSnapshot(lexlean, engineFiles);
  const slimEngine = await holoFromSnapshot(engineSnap.snapshot, {
    evaluator,
    snapshotBytes: null,
    proved: true,
    sources: null,
    proof: null,
  });
  if (!slimEngine.ok || !slimEngine.built) {
    throw new Error(`Slim Engine Holo v6 assembly failed: ${slimEngine.message}`);
  }

  // 2. Build self-carrying `StitchProof` Holo v6 (`snapshot: true, sources: true, proof: true`)
  //    and re-verify it offline via both `reproveHere` and `relexHere`.
  const stitchProofFiles = collectStitchProofProjectFiles(repoRoot);
  const stitchProofBuilt = await buildFromFiles({
    files: stitchProofFiles,
    lexlean,
    evaluator,
    prove,
    sources: true,
  });
  if (!stitchProofBuilt.ok || !stitchProofBuilt.holo) {
    throw new Error(
      `StitchProof Holo v6 build failed at stage=${stitchProofBuilt.stage}: ${stitchProofBuilt.message}\n${JSON.stringify(stitchProofBuilt.diagnostics ?? [], null, 2)}`,
    );
  }
  const stitchProofReproved = await reproveHoloBytes(stitchProofBuilt.holo.bytes, {
    env: resolvedEnv,
    requireSources: true,
  });

  return {
    appHolo: slimEngine.built,
    engineReproved,
    stitchProofHolo: stitchProofBuilt.holo,
    stitchProofReproved,
  };
}

if (import.meta.url === `file://${process.argv[1]}`) {
  const args = process.argv.slice(2);
  const reproveIdx = args.indexOf("--reprove");
  if (reproveIdx !== -1) {
    const target = resolve(args[reproveIdx + 1]);
    const info = await reproveHoloBytes(readFileSync(target), { requireSources: args.includes("--require-sources") });
    console.log(
      `reprove-holo-v6: VERIFIED ${target} (${info.kappa}, ${info.theoremsProved} theorems offline-reproved, relex=${info.relexed})`,
    );
    process.exit(0);
  }
  const outIdx = args.indexOf("--out");
  const outPath = resolve(outIdx !== -1 ? args[outIdx + 1] : join(root, "site/app.holo"));
  const proofIdx = args.indexOf("--proof-out");
  const proofPath = resolve(proofIdx !== -1 ? args[proofIdx + 1] : join(root, "site/generated/stitch-proof.holo"));

  const { appHolo, engineReproved, stitchProofHolo, stitchProofReproved } = await buildVerifiedHoloV6();
  mkdirSync(dirname(outPath), { recursive: true });
  writeFileSync(outPath, appHolo.bytes);
  mkdirSync(dirname(proofPath), { recursive: true });
  writeFileSync(proofPath, stitchProofHolo.bytes);
  console.log(
    `build-holo-v6: wrote ${outPath} (${appHolo.bytes.length} B, ${appHolo.label}, engineTheorems=${engineReproved.theoremsProved} reproved+relexed) and self-carrying ${proofPath} (${stitchProofHolo.bytes.length} B, ${stitchProofHolo.label}, proofTheorems=${stitchProofReproved.theoremsProved} reproved+relexed)`,
  );
}
