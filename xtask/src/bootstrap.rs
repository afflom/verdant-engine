//! Checks for the hand-reviewed bootstrap policy (`uor/template-contract/2`, `M29`).

use std::path::{Path, PathBuf};

use crate::proof::blake3_hex;
use crate::Fail;

const UNIVERSAL_POLICY_PATHS: [&str; 4] = [
    "AGENTS.md",
    "VERIFICATION.md",
    "template-contract.json",
    "template.lock",
];
const PROJECT_CONTENT_PATHS: [&str; 1] = ["CONFORMANCE.md"];

fn read(root: &Path, path: &str) -> Result<String, Fail> {
    Ok(std::fs::read_to_string(root.join(path))
        .map_err(|error| format!("reading {path}: {error}"))?)
}

fn immutable_image(value: &str) -> bool {
    value.rsplit_once("@sha256:").is_some_and(|(name, digest)| {
        let valid_registry = name.split_once('/').is_some_and(|(registry, path)| {
            let (host, port) = registry
                .rsplit_once(':')
                .map_or((registry, None), |(host, port)| (host, Some(port)));
            !host.is_empty()
                && host.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || b".-".contains(&byte)
                })
                && port.is_none_or(|port| port.parse::<u16>().is_ok_and(|port| port > 0))
                && !path.is_empty()
                && path.bytes().all(|byte| {
                    byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"./_-".contains(&byte)
                })
        });
        valid_registry
            && digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
}

fn policy_boundary_is_canonical(universal: &[&str], project: &[&str], required: &[&str]) -> bool {
    let mut union = universal.to_vec();
    union.extend(project.iter().copied());
    union.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    universal == UNIVERSAL_POLICY_PATHS
        && project == PROJECT_CONTENT_PATHS
        && union == required
        && union
            .windows(2)
            .all(|pair| pair[0].as_bytes() < pair[1].as_bytes())
}

/// Zero-runner draft PR gating (`#600`) + two-lane `fast-gate` & `proof-and-test` (`#603`):
/// `main` and tags run `proof-and-test` on both native amd64 and arm64, PRs/merge_group run once
/// on amd64, draft PRs skip runner allocation, and `main`/tag runs are never cancelled.
fn native_coverage_is_complete(bootstrap: &str) -> bool {
    const TRIGGERS: &str = "on:\n  push:\n    branches: [main]\n    tags: ['**']\n  pull_request:\n  merge_group:\n  workflow_dispatch:\n\n";
    const BOTH: &str = "[ubuntu-24.04, ubuntu-24.04-arm]";
    const PULL_REQUEST_AMD64: &str = r#"${{ fromJSON((github.event_name == 'pull_request' || github.event_name == 'merge_group') && '["ubuntu-24.04"]' || '["ubuntu-24.04", "ubuntu-24.04-arm"]') }}"#;
    const GROUP: &str = "${{ github.event_name == 'pull_request' && format('bootstrap-pr-{0}', github.event.pull_request.number) || format('bootstrap-run-{0}', github.run_id) }}";
    const CANCEL: &str = "${{ github.event_name == 'pull_request' }}";
    const DRAFT_SKIP: &str =
        "github.event_name != 'pull_request' || github.event.pull_request.draft == false";
    let values = |key: &str| {
        bootstrap
            .lines()
            .filter_map(|line| line.trim().strip_prefix(key))
            .map(str::trim)
            .collect::<Vec<_>>()
    };
    let runners = values("runner:");
    let groups = values("group:");
    let cancels = values("cancel-in-progress:");
    let ifs = values("if:");
    let forbidden = bootstrap.lines().map(str::trim).any(|line| {
        [
            "continue-on-error:",
            "include:",
            "exclude:",
            "paths:",
            "paths-ignore:",
            "branches-ignore:",
            "tags-ignore:",
        ]
        .iter()
        .any(|key| line.starts_with(key))
    });
    bootstrap.contains(TRIGGERS)
        && bootstrap.contains("\n  fast-gate:\n")
        && bootstrap.contains("\n  proof-and-test:\n")
        && bootstrap.contains("needs: fast-gate")
        && (runners == [BOTH] || runners == [PULL_REQUEST_AMD64])
        && values("runs-on:") == ["ubuntu-24.04", "${{ matrix.runner }}"]
        && ((groups.is_empty() && cancels.is_empty()) || (groups == [GROUP] && cancels == [CANCEL]))
        && ifs == [DRAFT_SKIP, DRAFT_SKIP]
        && !forbidden
}

const PROOF_KEY: &str = "key: prism-proof-${{ runner.os }}-${{ runner.arch }}-lexlean-${{ steps.proof-inputs.outputs.lexlean }}-lean-${{ steps.proof-inputs.outputs.lean }}-${{ hashFiles('app/**', 'template.lock', 'Justfile', 'rust-toolchain.toml', 'scripts/build-holo-v6.mjs') }}";

fn proof_reuse_is_confined(bootstrap: &str, justfile: &str) -> bool {
    let key_count = bootstrap
        .lines()
        .filter(|raw| raw.trim().starts_with("key: prism-proof-"))
        .count();
    let key_matches = bootstrap.lines().any(|raw| raw.trim() == PROOF_KEY);
    if key_count != 1 || !key_matches {
        return false;
    }
    let Some((_, after_app)) = justfile.split_once("\napp:\n") else {
        return false;
    };
    let recipe_lines: Vec<&str> = after_app
        .lines()
        .take_while(|line| line.starts_with("    ") || line.is_empty())
        .map(str::trim)
        .collect();
    let has_event = recipe_lines.contains(&"pull_request:*|merge_group:*|push:refs/heads/main)");
    let has_reuse =
        recipe_lines.contains(&"if cargo run -q --locked -p xtask -- proof-reuse; then");
    let has_clean = recipe_lines.contains(&"rm -rf .prism-proof");
    let has_verify = recipe_lines
        .contains(&"(cd app && lexlean --diagnostic-format json verify) > .prism-proof.json");
    let has_holo = recipe_lines.contains(&"node scripts/build-holo-v6.mjs");
    let has_keep =
        recipe_lines.contains(&r#"cargo run -q --locked -p xtask -- proof-keep "$attestation""#);
    bootstrap.contains("PRISM_PROOF_EVENT: ${{ github.event_name }}:${{ github.ref }}")
        && bootstrap.contains(".prism-proof")
        && has_event
        && has_reuse
        && has_clean
        && has_verify
        && has_holo
        && has_keep
}

fn workflow_history_is_complete(source: &str) -> bool {
    source.split("\n      - ").all(|step| {
        !step.contains("uses: actions/checkout@")
            || (step.lines().any(|line| line.trim() == "fetch-depth: 0")
                && step
                    .lines()
                    .any(|line| line.trim() == "persist-credentials: false"))
    })
}

fn update_preserves_project_content(source: &str) -> bool {
    source.lines().find_map(|line| {
        line.trim()
            .strip_prefix("for path in ")
            .and_then(|line| line.strip_suffix("; do"))
    }) == Some("AGENTS.md VERIFICATION.md template-contract.json template.lock")
        && source
            .contains("test \"$(git -C .template-policy rev-parse HEAD)\" = \"$TEMPLATE_REVISION\"")
        && source.contains(
            "git add -N -- AGENTS.md VERIFICATION.md template-contract.json template.lock",
        )
}

fn is_canonical_json_bytes(bytes: &[u8], value: &serde_json::Value) -> Result<bool, Fail> {
    let encoded = serde_json::to_vec(value)?;
    Ok(bytes == encoded.as_slice() || bytes.strip_suffix(b"\n") == Some(encoded.as_slice()))
}

fn audit_policy_files(root: &Path, lock: &serde_json::Value) -> Result<(), Fail> {
    let contract_bytes = std::fs::read(root.join("template-contract.json"))?;
    let contract: serde_json::Value = serde_json::from_slice(&contract_bytes)?;
    if !is_canonical_json_bytes(&contract_bytes, &contract)? {
        return Err("template-contract.json is not canonical JSON".into());
    }
    let expected_keys = [
        "project_content_paths",
        "required_paths",
        "schema",
        "universal_policy_paths",
        "version",
    ];
    if contract
        .as_object()
        .is_none_or(|object| object.keys().map(String::as_str).ne(expected_keys))
        || contract["schema"] != "uor/template-contract/2"
        || contract["version"] != "2.0.0"
    {
        return Err("template contract schema or version is unsupported".into());
    }
    let universal = contract["universal_policy_paths"]
        .as_array()
        .ok_or("template contract has no universal_policy_paths")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .ok_or("universal policy path is not a string")
        })
        .collect::<Result<Vec<_>, _>>()?;
    let project = contract["project_content_paths"]
        .as_array()
        .ok_or("template contract has no project_content_paths")?
        .iter()
        .map(|value| value.as_str().ok_or("project content path is not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    let required = contract["required_paths"]
        .as_array()
        .ok_or("template contract has no required_paths")?
        .iter()
        .map(|value| value.as_str().ok_or("required path is not a string"))
        .collect::<Result<Vec<_>, _>>()?;
    if !policy_boundary_is_canonical(&universal, &project, &required) {
        return Err("template contract path boundary is not canonical or disjoint".into());
    }
    if lock["schema"] != "uor/template-lock/2"
        || lock["template_repository"] != "UOR-Foundation/template"
        || lock["contract"] != format!("blake3:{}", blake3_hex(&contract_bytes))
    {
        return Err("template.lock contract label or schema is stale".into());
    }
    let mut expected = universal
        .iter()
        .filter(|path| **path != "template.lock")
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    expected.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    expected.dedup();

    let policy_files = lock["policy_files"]
        .as_array()
        .ok_or("template.lock has no policy_files")?;
    let observed = policy_files
        .iter()
        .map(|row| -> Result<String, Fail> {
            let object = row.as_object().ok_or("policy_files row is not an object")?;
            if object.len() != 2 || !object.contains_key("path") || !object.contains_key("blake3") {
                return Err("policy_files row is not closed".into());
            }
            let path = row["path"]
                .as_str()
                .ok_or("policy_files path is not a string")?;
            let label = row["blake3"]
                .as_str()
                .ok_or("policy_files blake3 is not a string")?;
            let bytes = std::fs::read(root.join(path))?;
            if format!("blake3:{}", blake3_hex(&bytes)) != label {
                return Err(format!("universal policy drift: {path}").into());
            }
            Ok(path.to_owned())
        })
        .collect::<Result<Vec<_>, Fail>>()?;
    if observed != expected {
        return Err("template.lock policy_files does not close the universal policy paths".into());
    }
    Ok(())
}

fn action_reference_is_pinned(line: &str) -> bool {
    let Some(reference) = line.trim().strip_prefix("uses:") else {
        return true;
    };
    let reference = reference.split('#').next().unwrap_or_default().trim();
    if reference.starts_with("./") {
        return false;
    }
    let Some((name, revision)) = reference.rsplit_once('@') else {
        return false;
    };
    !name.is_empty()
        && revision.len() == 40
        && revision
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn workflow_files(root: &Path) -> Result<Vec<PathBuf>, Fail> {
    let directory = root.join(".github/workflows");
    let mut paths = std::fs::read_dir(&directory)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "yml" || extension == "yaml")
        })
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

fn upstream_pins(authorities: &repo_model::Authorities) -> Option<Vec<String>> {
    let mut pins: Vec<String> = authorities
        .authority
        .iter()
        .flat_map(|row| row.pins.iter().cloned())
        .collect();
    let count = pins.len();
    pins.sort();
    pins.dedup();
    let named = pins.iter().all(|name| {
        !name.is_empty()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
    });
    (pins.len() == count && named).then_some(pins)
}

fn dependency_updates_are_confined(source: &str, pins: &[String]) -> bool {
    let Ok(config) = serde_json::from_str::<serde_json::Value>(source) else {
        return false;
    };
    let Ok(encoded) = serde_json::to_string_pretty(&config) else {
        return false;
    };
    if format!("{encoded}\n") != source {
        return false;
    }
    let mut cargo = serde_json::json!({
        "package-ecosystem": "cargo",
        "directory": "/",
        "schedule": {"interval": "weekly"},
        "groups": {"build-policy": {"patterns": ["*"]}},
        "commit-message": {"prefix": "chore(deps)"}
    });
    if !pins.is_empty() {
        cargo["ignore"] = pins
            .iter()
            .map(|name| serde_json::json!({"dependency-name": name}))
            .collect();
    }
    config
        == serde_json::json!({
            "version": 2,
            "updates": [
                {
                    "package-ecosystem": "github-actions",
                    "directory": "/",
                    "schedule": {"interval": "weekly"},
                    "groups": {"pinned-actions": {"patterns": ["*"]}},
                    "commit-message": {"prefix": "chore(deps)"},
                    "ignore": [{"dependency-name": "UOR-Foundation/PrismPM/action"}],
                    "exclude-paths": [".github/workflows/bootstrap.yml"]
                },
                cargo
            ]
        })
}

pub fn audit_dependency_updates(root: &Path) -> Result<(), Fail> {
    let model = repo_model::Model::load(&root.join("model"))?;
    let Some(pins) = upstream_pins(&model.authorities) else {
        return Err("an authority's pins must name each Cargo dependency once, exactly".into());
    };
    if !dependency_updates_are_confined(&read(root, ".github/dependabot.yml")?, &pins) {
        return Err("Dependabot policy must use canonical JSON-form YAML, exclude only the locked bootstrap and SDK action, ignore exactly the Cargo dependencies an authority pins, and retain weekly Actions/Cargo maintenance".into());
    }
    println!("audit-dependency-updates: SDK ownership and ordinary maintenance are preserved");
    Ok(())
}

pub fn audit(root: &Path) -> Result<(), Fail> {
    audit_dependency_updates(root)?;
    if root.join("prismpm.lock").exists() {
        return Err("prismpm.lock must not exist under uor/template-contract/2".into());
    }
    let template_lock_bytes = std::fs::read(root.join("template.lock"))
        .map_err(|error| format!("template.lock is required: {error}"))?;
    let template_lock: serde_json::Value = serde_json::from_slice(&template_lock_bytes)?;
    if !is_canonical_json_bytes(&template_lock_bytes, &template_lock)? {
        return Err("template.lock is not canonical JSON".into());
    }
    audit_policy_files(root, &template_lock)?;

    let devcontainer = read(root, ".devcontainer/devcontainer.json")?;
    let devcontainer_value: serde_json::Value = serde_json::from_str(&devcontainer)?;
    if !devcontainer_value["image"]
        .as_str()
        .is_some_and(immutable_image)
    {
        return Err(
            ".devcontainer/devcontainer.json must select an immutable digest-pinned LexLean image"
                .into(),
        );
    }

    let bootstrap = read(root, ".github/workflows/bootstrap.yml")?;
    if bootstrap.contains("pull_request_target")
        || !bootstrap.contains("permissions:\n  contents: read")
        || !bootstrap.contains("persist-credentials: false")
    {
        return Err("bootstrap.yml is not a read-only pull-request trust root".into());
    }
    if !proof_reuse_is_confined(&bootstrap, &read(root, "Justfile")?) {
        return Err(
            "bootstrap.yml and the Justfile may reuse a proof only on a pull request, a merge queue run or a main push, keyed by every proof input and checked on read"
                .into(),
        );
    }
    if !native_coverage_is_complete(&bootstrap) {
        return Err(
            "bootstrap.yml must skip draft PRs (#600), split fast-gate and proof-and-test (#603), and run both native architectures on main and tags"
                .into(),
        );
    }

    for path in workflow_files(root)? {
        let source = std::fs::read_to_string(&path)?;
        if !workflow_history_is_complete(&source) {
            return Err(format!(
                "{} omits complete history or retains checkout credentials",
                path.display()
            )
            .into());
        }
        if source.contains("ubuntu-latest")
            || source.contains("@main")
            || source.contains("@master")
            || source
                .lines()
                .any(|line| line.trim().starts_with("uses:") && !action_reference_is_pinned(line))
        {
            return Err(format!("{} contains a floating workflow input", path.display()).into());
        }
    }
    if !update_preserves_project_content(&read(root, ".github/workflows/template-update.yml")?) {
        return Err("template-update.yml overwrites project-owned inputs".into());
    }

    println!("audit-bootstrap: uor/template-contract/2 policy, LexLean 0.4.5 digest, and two-lane CI trust roots verified");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        action_reference_is_pinned, dependency_updates_are_confined, immutable_image,
        native_coverage_is_complete, policy_boundary_is_canonical, proof_reuse_is_confined,
        update_preserves_project_content, upstream_pins, workflow_history_is_complete,
        PROJECT_CONTENT_PATHS, UNIVERSAL_POLICY_PATHS,
    };

    #[test]
    fn dropped_native_coverage_or_draft_skip_plant_is_rejected() {
        let bootstrap = include_str!("../../.github/workflows/bootstrap.yml");
        assert!(native_coverage_is_complete(bootstrap));
        let matrix = r#"'["ubuntu-24.04"]' || '["ubuntu-24.04", "ubuntu-24.04-arm"]'"#;
        for plant in [
            bootstrap.replace(matrix, r#"'["ubuntu-24.04"]' || '["ubuntu-24.04"]'"#),
            bootstrap.replace("    tags: ['**']\n", ""),
            bootstrap.replace(
                "cancel-in-progress: ${{ github.event_name == 'pull_request' }}",
                "cancel-in-progress: true",
            ),
            bootstrap.replace("needs: fast-gate", ""),
            bootstrap.replace("github.event.pull_request.draft == false", "true"),
        ] {
            assert!(!native_coverage_is_complete(&plant));
        }
    }

    #[test]
    fn widened_proof_reuse_plant_is_rejected() {
        let bootstrap = include_str!("../../.github/workflows/bootstrap.yml");
        let justfile = include_str!("../../Justfile");
        assert!(proof_reuse_is_confined(bootstrap, justfile));
        assert!(!proof_reuse_is_confined(
            bootstrap,
            &justfile.replace("push:refs/heads/main)", "push:*)")
        ));
        assert!(!proof_reuse_is_confined(
            bootstrap,
            &justfile.replace("node scripts/build-holo-v6.mjs", "true")
        ));
    }

    #[test]
    fn dependency_updates_preserve_sdk_ownership_and_ordinary_maintenance() {
        let source = include_str!("../../.github/dependabot.yml");
        assert!(dependency_updates_are_confined(source, &[]));
        assert_eq!(
            upstream_pins(&repo_model::Authorities {
                spec: "template/1".into(),
                authority: vec![],
            }),
            Some(vec![])
        );
    }

    #[test]
    fn initial_standards_binding_and_holo_cache_pass() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap();
        let s1 = std::process::Command::new("node")
            .args(["--test", "bootstrap/standards-lock.test.mjs"])
            .current_dir(root)
            .status()
            .unwrap();
        assert!(s1.success());
        let s2 = std::process::Command::new("node")
            .args(["--test", "ci/kappa-cache.test.mjs"])
            .current_dir(root)
            .status()
            .unwrap();
        assert!(s2.success());
    }

    #[test]
    fn floating_action_and_mutable_image_and_shallow_checkout_are_rejected() {
        assert!(!action_reference_is_pinned("uses: actions/checkout@v4"));
        assert!(action_reference_is_pinned(
            "uses: actions/checkout@11bd71901bbe5b1630ceea73d27597364c9af683"
        ));
        assert!(!immutable_image("ghcr.io/uor-foundation/lexlean:latest"));
        assert!(immutable_image(&format!(
            "ghcr.io/uor-foundation/lexlean@sha256:{}",
            "a".repeat(64)
        )));
        let wf = include_str!("../../.github/workflows/pages.yml");
        assert!(workflow_history_is_complete(wf));
        assert!(!workflow_history_is_complete(&wf.replacen(
            "          fetch-depth: 0\n",
            "",
            1
        )));
        let tu = include_str!("../../.github/workflows/template-update.yml");
        assert!(update_preserves_project_content(tu));
    }

    #[test]
    fn narrowed_policy_boundary_plant_is_rejected() {
        let mut required = UNIVERSAL_POLICY_PATHS.to_vec();
        required.extend(PROJECT_CONTENT_PATHS);
        required.sort_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
        assert!(policy_boundary_is_canonical(
            &UNIVERSAL_POLICY_PATHS,
            &PROJECT_CONTENT_PATHS,
            &required,
        ));
        assert!(!policy_boundary_is_canonical(
            &UNIVERSAL_POLICY_PATHS[..3],
            &PROJECT_CONTENT_PATHS,
            &required,
        ));
    }
}
