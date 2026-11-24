# `just vv` is the normative acceptance gate. Every other recipe is either one
# of its parts or a writer the gate checks the output of.
#
# Each check in `vv` runs exactly once. The parts are ordered so the cheapest
# failure is reported first: formatting needs no compiler, the repository gates
# compile only `xtask`, and the SDK's checks run before the workspace build.

default: vv

# The whole gate.
vv: fmt-check model template-check experience-check fidelity lint test deny app
    @echo "vv: the acceptance gate passed"

fmt-check:
    just fmt --check

# R1, R4, R5 and the bootstrap trust root: `check-model`, `audit-limits`,
# `audit-deferral` and `audit-bootstrap` (which includes
# `audit-dependency-updates`), in one `xtask` process.
model:
    cargo run -q --locked -p xtask -- validate

# Containerless `uor/template-contract/2` + `CONFORMANCE.md` freshness check (`M29`).
template-check:
    #!/usr/bin/env bash
    set -euo pipefail
    node -e 'import("./try/vendor/prism-code/src/contract.mjs").then(({checkContract})=>{const d=checkContract(".");if(d.length){console.error(d);process.exit(1);}console.log("template-check: uor/template-contract/2 verified (0 violations)");})'
    cargo run -q --locked -p xtask -- check-model
    if command -v lexlean >/dev/null 2>&1; then
      (cd app && lexlean lock --check)
    fi

# Clippy at `-D warnings` over every target with every feature enabled. This is
# the one place an optional feature is compiled: a rename upstream of an
# optional dependency fails nowhere else until someone turns the flag on, and
# `--all-targets` because the tests behind a flag are code too. `test` compiles
# the default feature set.
lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# The workspace suite, which includes the `repo-conformance` meta-gate (R3,
# `CM-02`, `CM-03`).
test:
    cargo test --workspace --locked

# R6: nothing shipped depends on a dev-only crate, no wildcard version
# requirement, no advisory against anything in the tree. `cargo-deny` is
# supplied by the locked SDK.
deny:
    cargo deny --all-features check

# The template's own Prism application, the contract checker. The model must
# be canonical and locked. `prismpm verify` builds the holo, replaying its
# acceptance vectors in the compiled guest, then has the Lean kernel and
# leanchecker check every theorem (`TC-01`) and the generated code against the
# model. It runs the build itself, so a separate `prismpm build` would repeat it.
#
# A pull request, a merge queue run and a main push may reuse a kept proof;
# a tag, a manual run and every local run prove in full. bootstrap.yml
# restores `.prism-proof` from a cache keyed by every proof input and passes
# `<event>:<ref>` in PRISM_PROOF_EVENT. The key only finds a candidate:
# `xtask proof-reuse` reuses it only if its attestation ID resolves to the
# attestation it names and that attestation binds this checkout's model, this
# image's Lean and Lake, the architecture, the SDK image and every input file.
# Anything else proves in full, and every proof is kept and read back.
app:
    #!/usr/bin/env bash
    set -euo pipefail
    (cd app && lexlean fmt --check && lexlean lock --check)
    case "${PRISM_PROOF_EVENT:-}" in
      pull_request:*|merge_group:*|push:refs/heads/main)
        if cargo run -q --locked -p xtask -- proof-reuse; then
          exit 0
        fi
        ;;
    esac
    rm -rf .prism-proof
    (cd app && lexlean --diagnostic-format json verify) > .prism-proof.json
    attestation=$(node -e 'const r=JSON.parse(require("fs").readFileSync(".prism-proof.json","utf8")); if(!r.success) process.exit(1); process.stdout.write(String(r.attestation_id||"").replace(/^blake3:/,""))')
    rm -f .prism-proof.json
    node scripts/build-holo-v6.mjs
    cargo run -q --locked -p xtask -- proof-keep "$attestation"

# Only the meta-gate, while iterating on a scenario. `test` already runs it.
bdd:
    cargo test --locked -p repo-conformance

fmt *options:
    cargo metadata --locked --no-deps --format-version 1 | node -e 'const fs = require("node:fs"), cp = require("node:child_process"); const m = JSON.parse(fs.readFileSync(0, "utf8")); const p = m.packages.filter(p => m.workspace_members.includes(p.id)); const a = ["fmt", ...p.flatMap(p => ["--package", p.name]), "--", ...process.argv.slice(1)]; const r = cp.spawnSync("cargo", a, {stdio: "inherit"}); if (r.error) throw r.error; process.exit(r.status ?? 1);' -- {{options}}

# Regenerate everything the model owns: CONFORMANCE.md.
model-write:
    cargo run -q --locked -p xtask -- check-model --write

# FI-01..FI-08: W3C PROV-O citations, ISO 15939 measures, monotonic register,
# and the permanent zero-constants parametric ratchet.
fidelity:
    cargo run -q --locked -p prism-fidelity -- --project app --repo . --register fidelity.toml --ratchet model/parametric-ratchet.toml --cite model/provenance.toml app/src/Provenance.lex.tex --measures model/measures.toml --lean app/src/Fidelity.lex.tex --check

# Cite, rescore, and audit the model: Provenance.lex.tex, fidelity.toml, and Fidelity.lex.tex.
fidelity-write:
    cargo run -q --locked -p prism-fidelity -- --project app --repo . --register fidelity.toml --ratchet model/parametric-ratchet.toml --cite model/provenance.toml app/src/Provenance.lex.tex --measures model/measures.toml --lean app/src/Fidelity.lex.tex

# Emit Brand.lex.tex, Users.lex.tex, brand/tokens.json, and brand/foundation.css.
experience:
    python3 -I scripts/experience/authority.py . --check
    python3 -I scripts/experience/experience.py .
    cd app && lexlean lock && lexlean fmt

experience-check:
    scripts/experience/check.sh

# Check the mechanized Diataxis documentation and plain-language content standard.
content site="site":
    node scripts/content/check.mjs {{site}}

# R3's first step in one command: a register row, its scenario, a failing test
# named for the ID, and the regenerated CONFORMANCE.md.
#
#     just new-capability CX-01 build parsing "A parsed document round-trips"
new-capability id level suite statement:
    cargo run -q --locked -p xtask -- new-capability {{quote(id)}} {{quote(level)}} {{quote(suite)}} {{quote(statement)}}

# Rebuild the complete Tensor certificate once; its roots share one proof scope.
tensor-export:
    just certified-export tensor tensor-execution

quotient-export:
    just certified-export quotient tensor-quotient

encoding-export:
    just certified-export encoding tensor-encoding

certified-export scope directory:
    #!/usr/bin/env bash
    set -euo pipefail
    scope={{quote(scope)}}
    directory={{quote(directory)}}
    receipt=$(mktemp)
    trap 'rm -f "$receipt"' EXIT
    if ! lexlean --project "app/$scope.lexlean.toml" --diagnostic-format json production build --request "app/$scope-production.json" > "$receipt"; then
      cat "$receipt" >&2
      exit 1
    fi
    node - "$receipt" "$scope" "$directory" <<'JS'
    const fs = require('node:fs'), path = require('node:path'), crypto = require('node:crypto');
    const sha256File = p => crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
    const scope = process.argv[3], directory = process.argv[4];
    const receipt = JSON.parse(fs.readFileSync(process.argv[2], 'utf8'));
    if (!receipt.success || receipt.artifacts.length !== 1) throw Error('expected one certified bundle');
    const artifact = receipt.artifacts[0], prefix = `.lexlean-${scope}/production/`;
    if (!artifact.startsWith(prefix) || !/^[0-9a-f]{64}$/.test(artifact.slice(prefix.length))) throw Error('unexpected SDK output path');
    const source = path.resolve('app', artifact);
    const result = JSON.parse(fs.readFileSync(path.join(source, 'result.json'), 'utf8'));
    const rawResultId = String(result.result_id || '').replace(/^blake3:/, '');
    if (path.basename(source) !== rawResultId) throw Error('result identity differs from output path');
    const wanted = JSON.parse(fs.readFileSync(`app/${scope}-production.json`, 'utf8')).roots;
    if (result.roots.length !== wanted.length || wanted.some(root => !result.roots.some(r => r.root === root.root && r.outputs[0].package.name === root.package.name && r.outputs[0].package.version === root.package.version))) throw Error('unexpected roots');
    fs.mkdirSync('generated', {recursive: true});
    const staging = path.join('generated', `.${directory}-${process.pid}`);
    fs.cpSync(source, staging, {recursive: true});
    fs.rmSync(`generated/${directory}`, {recursive: true, force: true});
    fs.renameSync(staging, `generated/${directory}`);
    if (scope === 'quotient') {
      const primary = result.roots.flatMap(r => r.outputs).find(o => o.package.name === 'verdant_quotient_f64').package.files.find(f => f.path.endsWith('/src/lib.rs'));
      const primaryRel = `generated/${directory}/${primary.path}`;
      const files = result.roots.flatMap(r => r.outputs.flatMap(o => [...o.package.files.filter(f => f.path.endsWith('.rs')), o.crate].map(f => {
        const rel = `generated/${directory}/${f.path}`;
        return {path: rel, kappa: f.kappa, sha256: sha256File(rel)};
      })));
      fs.writeFileSync('quotient-kernel.lock', JSON.stringify({spec:'prism/kernel-lock/1',source:{repository:'afflom/verdant-engine',attestation:result.verification.attestation_id,result:result.result_id},output:{path:primaryRel,kappa:primary.kappa,sha256:sha256File(primaryRel)},files})+'\n');
    }
    JS
