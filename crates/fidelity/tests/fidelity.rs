//! The fidelity scorer and permanent zero-constants parametric ratchet rows (`FI-01`..`FI-08`).

use std::path::{Path, PathBuf};

use prism_fidelity::{
    file_kappa, lean,
    ratchet::{audit_parametric_ratchet, scan_lex_magic_literals, scan_rust_magic_literals},
    score, span_kappa, Axis, Register,
};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

const SOURCE: &str =
    "# Charter\nMaintainer: The maintainer keeps the engine true.\nAnother line.\n";

fn module(name: &str, body: &str) -> String {
    format!("\\begin{{lexlean}}{{{name}}}\n\\title{{t}}\n\\begin{{semanticmodule}}\n{body}\\end{{semanticmodule}}\n\\end{{lexlean}}\n")
}

fn project(dir: &Path, kappa: &str, derive: bool) {
    let src = dir.join("app/src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(dir.join("charter.md"), SOURCE).unwrap();
    std::fs::write(dir.join("measure.txt"), "3\n").unwrap();
    let measure_kappa = file_kappa("measure.txt", b"3\n");
    let records = "\
definition{name: stakeholderMaintainer, parameters: [], result: Foundation.Arch::Stakeholder, body: (record Foundation.Arch::Stakeholder {index := 0, name := \"Maintainer\", text := \"The maintainer keeps the engine true.\"})}
definition{name: concernTruth, parameters: [], result: Foundation.Arch::Concern, body: (record Foundation.Arch::Concern {index := 0, name := \"Truth\", text := \"x\"})}
definition{name: measureOpen, parameters: [], result: Foundation.Qual::QualityMeasure, body: (record Foundation.Qual::QualityMeasure {index := 0, requirementIndex := 0, threshold := 0})}
definition{name: componentBare, parameters: [], result: Foundation.Arch::Component, body: (record Foundation.Arch::Component {index := 0, name := \"Bare\", text := \"y\"})}
theorem{name: truthHolds, parameters: [], statement: ((project (concernTruth) index) = 0), proof: decide{}}
";
    std::fs::write(
        src.join("Architecture.lex.tex"),
        module("Architecture", records),
    )
    .unwrap();
    let derivation = if derive {
        "(cons (record Foundation.Provenance.V1.Model::Derivation {generatedEntity := \"Architecture::stakeholderMaintainer\", usedEntityIndex := 0, activityIndex := 0}) (nil Foundation.Provenance.V1.Model::Derivation))"
    } else {
        "(nil Foundation.Provenance.V1.Model::Derivation)"
    };
    let prov = format!(
        "definition{{name: sources, parameters: [], result: (List Foundation.Provenance.V1.Model::SourceEntity), body: (cons (record Foundation.Provenance.V1.Model::SourceEntity {{index := 0, location := \"charter.md\", firstLine := 2, lastLine := 2, kappa := \"{kappa}\"}}) (nil Foundation.Provenance.V1.Model::SourceEntity))}}
definition{{name: derivations, parameters: [], result: (List Foundation.Provenance.V1.Model::Derivation), body: {derivation}}}
definition{{name: indicators, parameters: [], result: (List Foundation.Measurement.V1.Model::Indicator), body: (cons (record Foundation.Measurement.V1.Model::Indicator {{index := 0, name := \"Architecture::measureOpen\", conceptIndex := 0, derivedMeasureIndex := 0, analysisModel := \"a\", criteriaIndex := 0}}) (nil Foundation.Measurement.V1.Model::Indicator))}}
definition{{name: results, parameters: [], result: (List Foundation.Measurement.V1.Model::MeasurementResult), body: (cons (record Foundation.Measurement.V1.Model::MeasurementResult {{indicatorIndex := 0, value := 3, source := \"measure.txt\", kappa := \"{measure_kappa}\"}}) (nil Foundation.Measurement.V1.Model::MeasurementResult))}}
"
    );
    std::fs::write(src.join("Provenance.lex.tex"), module("Provenance", &prov)).unwrap();
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("verdant-fidelity-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn open(score: &prism_fidelity::Score, record: &str, axis: Axis) -> bool {
    score
        .residue
        .iter()
        .any(|r| r.record == record && r.axis == axis)
}

/// `FI-01`: every record is scored on derived, meant and measured, a false κ
/// is refused, and a span without the record's text leaves it underived.
#[test]
fn scored_against_standards_fi_01() {
    let dir = scratch("fi01");
    let good = span_kappa(
        "charter.md",
        "Maintainer: The maintainer keeps the engine true.",
    );
    project(&dir, &good, true);
    let s = score(&dir.join("app"), &dir).expect("scores");
    assert_eq!(s.records.len(), 4);
    assert!(!open(
        &s,
        "Architecture::stakeholderMaintainer",
        Axis::Derived
    ));
    assert!(open(&s, "Architecture::concernTruth", Axis::Derived));
    assert!(!open(&s, "Architecture::concernTruth", Axis::Meant));
    assert!(open(&s, "Architecture::componentBare", Axis::Meant));
    assert!(!open(&s, "Architecture::measureOpen", Axis::Measured));

    project(&dir, &"0".repeat(64), true);
    let err = score(&dir.join("app"), &dir).expect_err("refused");
    assert!(err.to_string().contains("κ"), "{err}");

    let other = span_kappa("charter.md", "Another line.");
    project(&dir, &other, true);
    let text = std::fs::read_to_string(dir.join("app/src/Provenance.lex.tex")).unwrap();
    std::fs::write(
        dir.join("app/src/Provenance.lex.tex"),
        text.replace(
            "firstLine := 2, lastLine := 2",
            "firstLine := 3, lastLine := 3",
        ),
    )
    .unwrap();
    let s = score(&dir.join("app"), &dir).expect("scores");
    assert!(s
        .residue
        .iter()
        .any(|r| r.record == "Architecture::stakeholderMaintainer"
            && r.axis == Axis::Derived
            && r.reason.contains("verbatim")));
    let _ = std::fs::remove_dir_all(&dir);
}

/// `FI-02`: a closed κ never reopens, and the committed register matches the live model.
#[test]
fn register_only_shrinks_fi_02() {
    let dir = scratch("fi02");
    let good = span_kappa(
        "charter.md",
        "Maintainer: The maintainer keeps the engine true.",
    );
    project(&dir, &good, true);
    let first = score(&dir.join("app"), &dir).unwrap();
    let reg = Register::next(&Register::default(), &first).unwrap();
    assert_eq!(Register::parse(&reg.render()).unwrap(), reg);
    project(&dir, &good, false);
    let second = score(&dir.join("app"), &dir).unwrap();
    let err = Register::next(&reg, &second).expect_err("reopening refused");
    assert!(err
        .to_string()
        .contains("Architecture::stakeholderMaintainer"));
    let _ = std::fs::remove_dir_all(&dir);

    let repo = root();
    let s = score(&repo.join("app"), &repo).unwrap();
    let committed = std::fs::read_to_string(repo.join("fidelity.toml")).unwrap();
    let previous = Register::parse(&committed).unwrap();
    let next = Register::next(&previous, &s).unwrap().render();
    assert_eq!(
        committed, next,
        "fidelity.toml is stale: run `just fidelity-write`"
    );
    let specs = prism_fidelity::measures(
        &std::fs::read_to_string(repo.join("model/measures.toml")).unwrap(),
    )
    .unwrap();
    let measured: Vec<_> = specs
        .into_iter()
        .map(|m| {
            let k = file_kappa(&m.source, &std::fs::read(repo.join(&m.source)).unwrap());
            (m, k)
        })
        .collect();
    let lean_text = lean::module(
        &s,
        "fidelity.toml",
        &file_kappa("fidelity.toml", next.as_bytes()),
        &measured,
    );
    assert_eq!(
        std::fs::read_to_string(repo.join("app/src/Fidelity.lex.tex")).unwrap(),
        lean_text,
        "app/src/Fidelity.lex.tex is stale: run `just fidelity-write`"
    );
}

/// `FI-03`: one ISO/IEC/IEEE 15939 indicator per property checked by the Lean kernel.
#[test]
fn own_fidelity_measured_fi_03() {
    let repo = root();
    let text = std::fs::read_to_string(repo.join("app/src/Fidelity.lex.tex")).unwrap();
    let register = std::fs::read(repo.join("fidelity.toml")).unwrap();
    let kappa = file_kappa("fidelity.toml", &register);
    let s = score(&repo.join("app"), &repo).unwrap();
    for (i, axis) in Axis::ALL.into_iter().enumerate() {
        let row = format!(
            "indicatorIndex := {i}, value := {}, source := \"fidelity.toml\", kappa := \"{kappa}\"",
            s.open(axis)
        );
        assert!(text.contains(&row), "no result for {}: {row}", axis.name());
    }
    assert!(text.contains("name: indicatorsKnown"));
    assert!(text.contains("name: everyIndicatorMeasured"));
    let lexlean = std::fs::read_to_string(repo.join("app/lexlean.toml")).unwrap();
    assert!(lexlean.contains("\"src/Fidelity.lex.tex\""));
}

/// `FI-04`: records are cited mechanically from primary sources listed in `model/provenance.toml`.
#[test]
fn cited_from_primary_sources_fi_04() {
    let dir = scratch("fi04");
    let good = span_kappa(
        "charter.md",
        "Maintainer: The maintainer keeps the engine true.",
    );
    project(&dir, &good, false);
    let list = "sources = [\n  \"charter.md\",\n]\n";
    let (module_text, cited) = prism_fidelity::cite::module(&dir.join("app"), &dir, list).unwrap();
    assert_eq!(cited, 1);
    assert!(module_text.contains("location := \"charter.md\", firstLine := 2, lastLine := 2"));
    std::fs::write(
        dir.join("app/src/Cited.lex.tex"),
        module_text.replace("{Provenance}", "{Cited}"),
    )
    .unwrap();
    let s = score(&dir.join("app"), &dir).unwrap();
    assert!(!open(
        &s,
        "Architecture::stakeholderMaintainer",
        Axis::Derived
    ));
    assert!(open(&s, "Architecture::componentBare", Axis::Derived));
    let _ = std::fs::remove_dir_all(&dir);

    let repo = root();
    let list = std::fs::read_to_string(repo.join("model/provenance.toml")).unwrap();
    let (expected_prov, _) = prism_fidelity::cite::module(&repo.join("app"), &repo, &list).unwrap();
    assert_eq!(
        std::fs::read_to_string(repo.join("app/src/Provenance.lex.tex")).unwrap(),
        expected_prov,
        "app/src/Provenance.lex.tex is stale: run `just fidelity-write`"
    );
}

/// `FI-05`: every listed quality measure is evaluated by the Lean kernel against its threshold.
#[test]
fn measures_measured_by_the_kernel_fi_05() {
    let repo = root();
    let specs = prism_fidelity::measures(
        &std::fs::read_to_string(repo.join("model/measures.toml")).unwrap(),
    )
    .unwrap();
    assert!(!specs.is_empty());
    let text = std::fs::read_to_string(repo.join("app/src/Fidelity.lex.tex")).unwrap();
    let s = score(&repo.join("app"), &repo).unwrap();
    for m in &specs {
        let short = m.record.rsplit("::").next().unwrap();
        assert!(text.contains(&format!("bound := (project ({}) threshold)", m.record)));
        assert!(text.contains(&format!("value := ({})", m.method)));
        assert!(text.contains(&format!("name: {short}Holds")));
        assert!(s
            .closed
            .iter()
            .any(|r| r.record == m.record && r.axis == Axis::Measured));
    }
    assert!(prism_fidelity::measures(
        "[[measure]]\nrecord = \"a\"\nmethod = \"b\"\nsource = \"c\"\ndirection = \"sideways\"\n"
    )
    .is_err());
}

/// `FI-06`: record-to-record W3C PROV-O derivations close only when upstream records close.
#[test]
fn derived_from_records_fi_06() {
    let dir = scratch("fi06");
    let good = span_kappa(
        "charter.md",
        "Maintainer: The maintainer keeps the engine true.",
    );
    project(&dir, &good, false);
    let users = "\
definition{name: groupMaintainer, parameters: [], result: Foundation.Experience.V1.Model::UserGroup, body: (record Foundation.Experience.V1.Model::UserGroup {stakeholder := (Architecture::stakeholderMaintainer), characteristics := \"\"})}
definition{name: groupBare, parameters: [], result: Foundation.Experience.V1.Model::UserGroup, body: (record Foundation.Experience.V1.Model::UserGroup {stakeholder := (Architecture::componentBare), characteristics := \"y\"})}
";
    std::fs::write(dir.join("app/src/Users.lex.tex"), module("Users", users)).unwrap();
    let list = "sources = [\n  \"charter.md\",\n]\n";
    let (module_text, _) = prism_fidelity::cite::module(&dir.join("app"), &dir, list).unwrap();
    std::fs::write(
        dir.join("app/src/Cited.lex.tex"),
        module_text.replace("{Provenance}", "{Cited}"),
    )
    .unwrap();
    let s = score(&dir.join("app"), &dir).unwrap();
    assert!(!open(&s, "Users::groupMaintainer", Axis::Derived));
    assert!(open(&s, "Users::groupBare", Axis::Derived));

    let forged = module_text.replace(
        "usedEntity := \"Architecture::stakeholderMaintainer\"",
        "usedEntity := \"Architecture::concernTruth\"",
    );
    std::fs::write(
        dir.join("app/src/Cited.lex.tex"),
        forged.replace("{Provenance}", "{Cited}"),
    )
    .unwrap();
    assert!(score(&dir.join("app"), &dir).is_err());
    let _ = std::fs::remove_dir_all(&dir);

    // Verify intra-module and foreign-key derivations in the live Verdant Engine repository:
    let repo = root();
    let live = score(&repo.join("app"), &repo).unwrap();
    for closed_rec in [
        "Subsystems::epochVisual0",
        "Subsystems::placement6",
        "Architecture::edgeCoreToStudio",
        "Architecture::viewpointGameDeveloper",
        "Quality::subcharacteristicTimeBehaviour",
        "Security::assetVerdantCoreRuntime",
        "Security::riskSnapshotTampering",
    ] {
        if live.records.iter().any(|r| r.name == closed_rec) {
            assert!(
                !open(&live, closed_rec, Axis::Derived),
                "expected `{closed_rec}` derived axis to be closed via RecordDerivation"
            );
        }
    }
}

/// `FI-07`: charter sources are attributed to their human author as a W3C PROV-O person agent.
#[test]
fn sources_attributed_to_their_authors_fi_07() {
    let dir = scratch("fi07");
    let good = span_kappa(
        "charter.md",
        "Maintainer: The maintainer keeps the engine true.",
    );
    project(&dir, &good, false);
    let list = "sources = [\n  \"charter.md\",\n]\n\n[[author]]\nname = \"Ada\"\nkind = \"person\"\nattributed = [\"charter.md\"]\n";
    let (module_text, _) = prism_fidelity::cite::module(&dir.join("app"), &dir, list).unwrap();
    assert!(module_text.contains(
        "name := \"Ada\", kind := (ctor Foundation.Provenance.V1.Model::AgentKind.person)"
    ));
    let _ = std::fs::remove_dir_all(&dir);

    let repo = root();
    let list = std::fs::read_to_string(repo.join("model/provenance.toml")).unwrap();
    let authors = prism_fidelity::cite::authors(&list).unwrap();
    assert!(authors
        .iter()
        .any(|a| a.kind == "person" && a.attributed.iter().any(|s| s == "model/charter.md")));
}

/// `FI-08`: permanent zero-constants parametric ratchet enforces zero magic literals across
/// `crates/verdant-core/src` and constructive LexLean modules, and refuses planted defects.
#[test]
fn zero_constants_parametric_ratchet_fi_08() {
    let repo = root();
    let report = audit_parametric_ratchet(&repo, Path::new("model/parametric-ratchet.toml"))
        .expect("live repository satisfies zero-constants parametric ratchet");
    assert_eq!(report.ontology_root_literal_gt_one_count, 0);
    assert_eq!(report.ontology_root_string_field_count, 0);
    assert_eq!(report.classical_choice_count, 0);
    assert_eq!(report.ungrounded_lex_module_count, 0);
    assert_eq!(report.domain_lex_magic_literal_count, 0);
    assert_eq!(report.engine_rust_magic_literal_count, 0);
    assert!(report.audited_rust_file_count >= 10);

    // Planted defect checks in an isolated scratch workspace:
    let dir = scratch("fi08");
    std::fs::create_dir_all(dir.join("app/src/Foundation/Ontology/V1")).unwrap();
    std::fs::create_dir_all(dir.join("crates/verdant-core/src")).unwrap();
    std::fs::create_dir_all(dir.join("model")).unwrap();

    let clean_ont = "\\begin{lexlean}{Foundation.Ontology.V1.Model}\n\\title{t}\n\\begin{semanticmodule}\ndefinition{name: two, parameters: [], result: Nat, body: (1 + 1)}\n\\end{semanticmodule}\n\\end{lexlean}\n";
    let clean_rs = "pub fn half() -> f64 { 1.0 / (1.0 + 1.0) }\n";
    let base_cfg = r#"
spec = "prism/parametric-ratchet/1"
ontology_root = "app/src/Foundation/Ontology/V1/Model.lex.tex"
engine_rust_root = "crates/verdant-core/src"
pure_zero_one_lex_modules = ["app/src/Foundation/Ontology/V1/Model.lex.tex"]

[ceilings]
max_ontology_root_literal_gt_one_count = 0
max_ontology_root_string_field_count = 0
max_classical_choice_count = 0
max_ungrounded_lex_module_count = 0
max_domain_lex_magic_literal_count = 0
max_engine_rust_magic_literal_count = 0

[[stratum_module]]
path = "app/src/Foundation/Ontology/V1/Model.lex.tex"
stratum = 0
role = "Constructive ontology root."
"#;

    std::fs::write(
        dir.join("app/src/Foundation/Ontology/V1/Model.lex.tex"),
        clean_ont,
    )
    .unwrap();
    std::fs::write(dir.join("crates/verdant-core/src/lib.rs"), clean_rs).unwrap();
    std::fs::write(dir.join("model/parametric-ratchet.toml"), base_cfg).unwrap();
    assert!(audit_parametric_ratchet(&dir, Path::new("model/parametric-ratchet.toml")).is_ok());

    // 1. Planted magic float & integer constant in shipped Rust engine code is refused:
    std::fs::write(
        dir.join("crates/verdant-core/src/lib.rs"),
        "pub fn sabine() -> f64 { 0.161 * 60 as f64 }\n",
    )
    .unwrap();
    assert_eq!(
        scan_rust_magic_literals("pub fn sabine() -> f64 { 0.161 * 60 as f64 }\n").len(),
        2
    );
    assert!(audit_parametric_ratchet(&dir, Path::new("model/parametric-ratchet.toml")).is_err());
    std::fs::write(dir.join("crates/verdant-core/src/lib.rs"), clean_rs).unwrap();

    // 2. Planted integer literal > 1 in constructive LexLean module is refused:
    let dirty_ont = clean_ont.replace("(1 + 1)", "42");
    assert_eq!(scan_lex_magic_literals(&dirty_ont).len(), 1);
    std::fs::write(
        dir.join("app/src/Foundation/Ontology/V1/Model.lex.tex"),
        &dirty_ont,
    )
    .unwrap();
    assert!(audit_parametric_ratchet(&dir, Path::new("model/parametric-ratchet.toml")).is_err());

    // 3. Planted Classical.choice axiom is refused:
    let choice_ont =
        clean_ont.replace("body: (1 + 1)", "body: (1 + 1), axioms: [Classical.choice]");
    std::fs::write(
        dir.join("app/src/Foundation/Ontology/V1/Model.lex.tex"),
        &choice_ont,
    )
    .unwrap();
    assert!(audit_parametric_ratchet(&dir, Path::new("model/parametric-ratchet.toml")).is_err());
    std::fs::write(
        dir.join("app/src/Foundation/Ontology/V1/Model.lex.tex"),
        clean_ont,
    )
    .unwrap();

    // 4. Attempting to weaken any ratchet ceiling above 0 is refused:
    std::fs::write(
        dir.join("model/parametric-ratchet.toml"),
        base_cfg.replace(
            "max_engine_rust_magic_literal_count = 0",
            "max_engine_rust_magic_literal_count = 1",
        ),
    )
    .unwrap();
    assert!(audit_parametric_ratchet(&dir, Path::new("model/parametric-ratchet.toml")).is_err());

    let _ = std::fs::remove_dir_all(&dir);
}
