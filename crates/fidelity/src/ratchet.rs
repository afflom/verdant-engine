//! Permanent Zero-Constants & Purely Parametric Ratchet.
//!
//! Enforces mechanically during `prism-fidelity` (and therefore `just vv`) that:
//! 1. `Foundation/Ontology/V1/Model.lex.tex` contains zero numeric literals outside `{0, 1}`
//!    and zero `String` fields.
//! 2. Every `.lex.tex` module under `app/src/` contains zero `Classical.choice` axioms and is
//!    explicitly grounded in the 8-Stratum Causal Obstruction Chain (`Stratum 0..=7`).
//! 3. Every domain logic LexLean module listed in `pure_zero_one_lex_modules` contains zero
//!    numeric literals outside `{0, 1}` inside `\begin{semanticmodule}...\end{semanticmodule}`.
//! 4. Every shipped Rust source file under `crates/verdant-core/src/**/*.rs` (excluding comments,
//!    string/char literals, and `#[cfg(test)]` test modules) contains zero floating-point literals
//!    outside `{0.0, 1.0}` and zero integer literals `> 1`.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::FidelityError;

/// Expected `spec` header in `model/parametric-ratchet.toml`.
pub const SPEC: &str = "prism/parametric-ratchet/1";

const MAX_STRATUM_RANK: u8 = 7;

/// Parsed configuration from `model/parametric-ratchet.toml`.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ParametricRatchetConfig {
    /// Format identifier (`prism/parametric-ratchet/1`).
    pub spec: String,
    /// Relative path to `Foundation/Ontology/V1/Model.lex.tex`.
    pub ontology_root: String,
    /// Relative directory of shipped Rust engine sources (`crates/verdant-core/src`).
    pub engine_rust_root: String,
    /// LexLean modules whose `semanticmodule` body must contain zero numeric literals `> 1`.
    pub pure_zero_one_lex_modules: Vec<String>,
    /// Hard zero ceilings enforced by the ratchet.
    pub ceilings: RatchetCeilings,
    /// Complete mapping of every `app/src/**/*.lex.tex` module to its causal Stratum `0..=7`.
    #[serde(rename = "stratum_module")]
    pub stratum_modules: Vec<StratumModuleEntry>,
}

/// Hard ceilings in `model/parametric-ratchet.toml` (all required to be `0`).
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RatchetCeilings {
    /// Maximum allowed numeric literals `> 1` in `ontology_root` (`0`).
    pub max_ontology_root_literal_gt_one_count: usize,
    /// Maximum allowed `String` fields in `ontology_root` (`0`).
    pub max_ontology_root_string_field_count: usize,
    /// Maximum allowed `Classical.choice` occurrences across `app/src/**/*.lex.tex` (`0`).
    pub max_classical_choice_count: usize,
    /// Maximum allowed ungrounded `.lex.tex` modules (`0`).
    pub max_ungrounded_lex_module_count: usize,
    /// Maximum allowed magic numeric literals `> 1` in `pure_zero_one_lex_modules` (`0`).
    pub max_domain_lex_magic_literal_count: usize,
    /// Maximum allowed magic numeric literals in shipped `crates/verdant-core/src/**/*.rs` (`0`).
    pub max_engine_rust_magic_literal_count: usize,
}

/// Grounding entry assigning a backend `.lex.tex` module to a causal Stratum `0..=7`.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StratumModuleEntry {
    /// Repository-relative `.lex.tex` path.
    pub path: String,
    /// Causal obstruction stratum rank in `0..=7`.
    pub stratum: u8,
    /// Concise statement of the obstruction resolved at this stratum.
    pub role: String,
}

/// Single ratchet violation diagnostic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RatchetViolation {
    /// Repository-relative file path.
    pub path: String,
    /// 1-based line number (`0` for module-level structural violations).
    pub line: usize,
    /// Offending token or structural reason.
    pub detail: String,
}

/// Complete audit outcome of the Zero-Constants Parametric Ratchet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RatchetReport {
    /// Observed literals `> 1` in `ontology_root`.
    pub ontology_root_literal_gt_one_count: usize,
    /// Observed `String` fields in `ontology_root`.
    pub ontology_root_string_field_count: usize,
    /// Observed `Classical.choice` occurrences across `app/src/`.
    pub classical_choice_count: usize,
    /// Observed ungrounded `.lex.tex` modules in `app/src/`.
    pub ungrounded_lex_module_count: usize,
    /// Observed literals `> 1` in `pure_zero_one_lex_modules`.
    pub domain_lex_magic_literal_count: usize,
    /// Observed magic float/integer literals in shipped `crates/verdant-core/src/**/*.rs`.
    pub engine_rust_magic_literal_count: usize,
    /// Total grounded `.lex.tex` modules verified.
    pub grounded_lex_module_count: usize,
    /// Total shipped Rust `.rs` files audited.
    pub audited_rust_file_count: usize,
}

/// Audits the repository against `config_rel` and fails closed on any violation or weakened ceiling.
pub fn audit_parametric_ratchet(
    repo: &Path,
    config_rel: &Path,
) -> Result<RatchetReport, FidelityError> {
    let config_path = if config_rel.is_absolute() {
        config_rel.to_path_buf()
    } else {
        repo.join(config_rel)
    };
    let raw_cfg = std::fs::read_to_string(&config_path)
        .map_err(|e| FidelityError::Io(config_path.clone(), e))?;
    let cfg: ParametricRatchetConfig = toml::from_str(&raw_cfg)
        .map_err(|e| FidelityError::Register(format!("{}: {e}", config_path.display())))?;

    if cfg.spec != SPEC {
        return Err(FidelityError::Register(format!(
            "{}: unsupported spec {:?}, expected {:?}",
            config_path.display(),
            cfg.spec,
            SPEC
        )));
    }

    // Enforce that ceilings themselves can never be weakened above zero:
    let c = cfg.ceilings;
    if c.max_ontology_root_literal_gt_one_count != 0
        || c.max_ontology_root_string_field_count != 0
        || c.max_classical_choice_count != 0
        || c.max_ungrounded_lex_module_count != 0
        || c.max_domain_lex_magic_literal_count != 0
        || c.max_engine_rust_magic_literal_count != 0
    {
        return Err(FidelityError::Register(
            "parametric-ratchet ceilings must all remain strictly 0 (zero-constants policy)".into(),
        ));
    }

    let mut violations: Vec<RatchetViolation> = Vec::new();

    // 1. Audit ontology_root (`Foundation/Ontology/V1/Model.lex.tex`):
    let ont_full = repo.join(&cfg.ontology_root);
    let ont_src =
        std::fs::read_to_string(&ont_full).map_err(|e| FidelityError::Io(ont_full.clone(), e))?;
    let ont_lits = scan_lex_magic_literals(&ont_src);
    for (line, tok) in &ont_lits {
        violations.push(RatchetViolation {
            path: cfg.ontology_root.clone(),
            line: *line,
            detail: format!("ontology root literal outside {{0, 1}}: `{tok}`"),
        });
    }
    let mut ontology_root_string_field_count = 0usize;
    for (idx, line) in ont_src.lines().enumerate() {
        if line.contains("type: String") || line.contains("type: (List String)") {
            ontology_root_string_field_count += 1;
            violations.push(RatchetViolation {
                path: cfg.ontology_root.clone(),
                line: idx + 1,
                detail: "ontology root contains forbidden String field".into(),
            });
        }
    }

    // 2. Walk every `.lex.tex` file under `app/src/`:
    let app_src = repo.join("app/src");
    let mut lex_files = Vec::new();
    collect_files_with_suffix(&app_src, ".lex.tex", &mut lex_files)?;
    lex_files.sort();

    let mut grounded_map: BTreeMap<String, u8> = BTreeMap::new();
    for entry in &cfg.stratum_modules {
        if entry.role.trim().is_empty() {
            return Err(FidelityError::Register(format!(
                "stratum_module `{}` has an empty role",
                entry.path
            )));
        }
        if entry.stratum > MAX_STRATUM_RANK {
            return Err(FidelityError::Register(format!(
                "stratum_module `{}` has stratum {} > {}",
                entry.path, entry.stratum, MAX_STRATUM_RANK
            )));
        }
        if grounded_map
            .insert(entry.path.clone(), entry.stratum)
            .is_some()
        {
            return Err(FidelityError::Register(format!(
                "duplicate stratum_module `{}`",
                entry.path
            )));
        }
    }

    let mut disk_lex_paths = BTreeSet::new();
    let mut classical_choice_count = 0usize;
    let mut ungrounded_lex_module_count = 0usize;

    for full in &lex_files {
        let rel = rel_slash(repo, full);
        disk_lex_paths.insert(rel.clone());
        if !grounded_map.contains_key(&rel) {
            ungrounded_lex_module_count += 1;
            violations.push(RatchetViolation {
                path: rel.clone(),
                line: 0,
                detail: "LexLean module is not grounded in model/parametric-ratchet.toml".into(),
            });
        }
        let text = std::fs::read_to_string(full).map_err(|e| FidelityError::Io(full.clone(), e))?;
        for (idx, line) in text.lines().enumerate() {
            if line.contains("Classical.choice") {
                classical_choice_count += 1;
                violations.push(RatchetViolation {
                    path: rel.clone(),
                    line: idx + 1,
                    detail: "forbidden non-constructive `Classical.choice` axiom".into(),
                });
            }
        }
    }

    for declared in grounded_map.keys() {
        if !disk_lex_paths.contains(declared) {
            ungrounded_lex_module_count += 1;
            violations.push(RatchetViolation {
                path: declared.clone(),
                line: 0,
                detail: "declared stratum_module does not exist on disk".into(),
            });
        }
    }

    // 3. Audit `pure_zero_one_lex_modules` for zero literals `> 1`:
    let mut domain_lex_magic_literal_count = 0usize;
    for mod_rel in &cfg.pure_zero_one_lex_modules {
        let mod_full = repo.join(mod_rel);
        let src = std::fs::read_to_string(&mod_full)
            .map_err(|e| FidelityError::Io(mod_full.clone(), e))?;
        let found = scan_lex_magic_literals(&src);
        domain_lex_magic_literal_count += found.len();
        for (line, tok) in found {
            violations.push(RatchetViolation {
                path: mod_rel.clone(),
                line,
                detail: format!("domain LexLean literal outside {{0, 1}}: `{tok}`"),
            });
        }
    }

    // 4. Audit shipped Rust engine sources `crates/verdant-core/src/**/*.rs`:
    let rust_root = repo.join(&cfg.engine_rust_root);
    let mut rust_files = Vec::new();
    collect_files_with_suffix(&rust_root, ".rs", &mut rust_files)?;
    rust_files.sort();
    if rust_files.is_empty() {
        return Err(FidelityError::Register(format!(
            "engine_rust_root `{}` contains no `.rs` files",
            cfg.engine_rust_root
        )));
    }

    let mut engine_rust_magic_literal_count = 0usize;
    for full in &rust_files {
        let rel = rel_slash(repo, full);
        let src = std::fs::read_to_string(full).map_err(|e| FidelityError::Io(full.clone(), e))?;
        let found = scan_rust_magic_literals(&src);
        engine_rust_magic_literal_count += found.len();
        for (line, tok) in found {
            violations.push(RatchetViolation {
                path: rel.clone(),
                line,
                detail: format!("magic numeric literal `{tok}` in shipped engine source"),
            });
        }
    }

    let report = RatchetReport {
        ontology_root_literal_gt_one_count: ont_lits.len(),
        ontology_root_string_field_count,
        classical_choice_count,
        ungrounded_lex_module_count,
        domain_lex_magic_literal_count,
        engine_rust_magic_literal_count,
        grounded_lex_module_count: disk_lex_paths.len(),
        audited_rust_file_count: rust_files.len(),
    };

    if report.ontology_root_literal_gt_one_count > c.max_ontology_root_literal_gt_one_count
        || report.ontology_root_string_field_count > c.max_ontology_root_string_field_count
        || report.classical_choice_count > c.max_classical_choice_count
        || report.ungrounded_lex_module_count > c.max_ungrounded_lex_module_count
        || report.domain_lex_magic_literal_count > c.max_domain_lex_magic_literal_count
        || report.engine_rust_magic_literal_count > c.max_engine_rust_magic_literal_count
        || !violations.is_empty()
    {
        let mut msg = String::from("zero-constants parametric ratchet refused:\n");
        for v in violations.iter().take(16) {
            if v.line > 0 {
                msg.push_str(&format!("  - {}:{}: {}\n", v.path, v.line, v.detail));
            } else {
                msg.push_str(&format!("  - {}: {}\n", v.path, v.detail));
            }
        }
        return Err(FidelityError::False(msg.trim_end().to_string()));
    }

    Ok(report)
}

fn rel_slash(repo: &Path, full: &Path) -> String {
    full.strip_prefix(repo)
        .unwrap_or(full)
        .to_string_lossy()
        .replace('\\', "/")
}

fn collect_files_with_suffix(
    dir: &Path,
    suffix: &str,
    out: &mut Vec<PathBuf>,
) -> Result<(), FidelityError> {
    let entries = std::fs::read_dir(dir).map_err(|e| FidelityError::Io(dir.to_path_buf(), e))?;
    for entry in entries {
        let entry = entry.map_err(|e| FidelityError::Io(dir.to_path_buf(), e))?;
        let path = entry.path();
        if path.is_dir() {
            collect_files_with_suffix(&path, suffix, out)?;
        } else if path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.ends_with(suffix))
        {
            out.push(path);
        }
    }
    Ok(())
}

/// Scans `\begin{semanticmodule}...\end{semanticmodule}` of a LexLean source and returns
/// `(line_number, token)` for every numeric literal outside `{"0", "1"}`.
pub fn scan_lex_magic_literals(source: &str) -> Vec<(usize, String)> {
    let open_tag = "\\begin{semanticmodule}";
    let close_tag = "\\end{semanticmodule}";
    let Some(start_idx) = source.find(open_tag) else {
        return Vec::new();
    };
    let body_start = start_idx + open_tag.len();
    let body_end = source[body_start..]
        .find(close_tag)
        .map_or(source.len(), |off| body_start + off);

    let mut sanitized = vec![b' '; source.len()];
    for (idx, &b) in source.as_bytes().iter().enumerate() {
        if b == b'\n' {
            sanitized[idx] = b'\n';
        } else if idx >= body_start && idx < body_end {
            sanitized[idx] = b;
        }
    }

    // Blank out `"..."` string literals inside `semanticmodule`:
    let mut i = body_start;
    while i < body_end {
        if sanitized[i] == b'"' {
            sanitized[i] = b' ';
            i += 1;
            while i < body_end {
                let c = sanitized[i];
                if c == b'\\' && i + 1 < body_end {
                    if sanitized[i] != b'\n' {
                        sanitized[i] = b' ';
                    }
                    if sanitized[i + 1] != b'\n' {
                        sanitized[i + 1] = b' ';
                    }
                    i += 2;
                    continue;
                }
                if c != b'\n' {
                    sanitized[i] = b' ';
                }
                i += 1;
                if c == b'"' {
                    break;
                }
            }
            continue;
        }
        i += 1;
    }

    let mut out = Vec::new();
    let mut line = 1usize;
    let mut pos = 0usize;
    while pos < sanitized.len() {
        let c = sanitized[pos];
        if c == b'\n' {
            line += 1;
            pos += 1;
            continue;
        }
        if c.is_ascii_alphabetic() || c == b'_' {
            while pos < sanitized.len()
                && (sanitized[pos].is_ascii_alphanumeric() || sanitized[pos] == b'_')
            {
                pos += 1;
            }
            continue;
        }
        if c.is_ascii_digit() {
            let start = pos;
            while pos < sanitized.len()
                && (sanitized[pos].is_ascii_alphanumeric() || sanitized[pos] == b'_')
            {
                pos += 1;
            }
            let tok = String::from_utf8_lossy(&sanitized[start..pos]).into_owned();
            if tok != "0" && tok != "1" {
                out.push((line, tok));
            }
            continue;
        }
        pos += 1;
    }
    out
}

/// Scans a shipped Rust source file (excluding comments, string/char literals, and `#[cfg(test)]`
/// items) and returns `(line_number, token)` for every float literal outside `{0.0, 1.0}` and
/// every integer literal `> 1`.
pub fn scan_rust_magic_literals(source: &str) -> Vec<(usize, String)> {
    let mut buf = strip_rust_comments_and_strings(source.as_bytes());
    blank_cfg_test_items(&mut buf);

    let mut out = Vec::new();
    let mut line = 1usize;
    let mut i = 0usize;
    while i < buf.len() {
        let c = buf[i];
        if c == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if c.is_ascii_alphabetic() || c == b'_' {
            while i < buf.len() && (buf[i].is_ascii_alphanumeric() || buf[i] == b'_') {
                i += 1;
            }
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            let is_radix = c == b'0'
                && i + 1 < buf.len()
                && matches!(buf[i + 1], b'x' | b'X' | b'b' | b'B' | b'o' | b'O');
            if is_radix {
                i += 2;
                while i < buf.len() && (buf[i].is_ascii_alphanumeric() || buf[i] == b'_') {
                    i += 1;
                }
                let tok = String::from_utf8_lossy(&buf[start..i]).into_owned();
                if !is_allowed_radix_zero_or_one(&tok) {
                    out.push((line, tok));
                }
                continue;
            }

            while i < buf.len() && (buf[i].is_ascii_digit() || buf[i] == b'_') {
                i += 1;
            }
            let mut is_float = false;
            if i + 1 < buf.len() && buf[i] == b'.' && buf[i + 1].is_ascii_digit() {
                is_float = true;
                i += 1;
                while i < buf.len() && (buf[i].is_ascii_digit() || buf[i] == b'_') {
                    i += 1;
                }
            }
            if i < buf.len() && (buf[i] == b'e' || buf[i] == b'E') {
                let next_ok = i + 1 < buf.len()
                    && (buf[i + 1].is_ascii_digit() || buf[i + 1] == b'+' || buf[i + 1] == b'-');
                if next_ok {
                    is_float = true;
                    i += 1;
                    if i < buf.len() && (buf[i] == b'+' || buf[i] == b'-') {
                        i += 1;
                    }
                    while i < buf.len() && (buf[i].is_ascii_digit() || buf[i] == b'_') {
                        i += 1;
                    }
                }
            }
            // Optional Rust numeric suffix (`_u64`, `f64`, `usize`, ...):
            while i < buf.len() && (buf[i].is_ascii_alphanumeric() || buf[i] == b'_') {
                i += 1;
            }
            let tok = String::from_utf8_lossy(&buf[start..i]).into_owned();
            if is_float {
                if !is_allowed_float_zero_or_one(&tok) {
                    out.push((line, tok));
                }
            } else if !is_allowed_decimal_zero_or_one(&tok) {
                out.push((line, tok));
            }
            continue;
        }
        i += 1;
    }
    out
}

const INT_OR_FLOAT_SUFFIXES: &[&str] = &[
    "u128", "i128", "usize", "isize", "u64", "i64", "u32", "i32", "u16", "i16", "u8", "i8", "f64",
    "f32",
];

fn strip_numeric_suffix(raw: &str) -> &str {
    for suf in INT_OR_FLOAT_SUFFIXES {
        if let Some(stem) = raw.strip_suffix(suf) {
            return stem.trim_end_matches('_');
        }
    }
    raw
}

fn is_allowed_float_zero_or_one(tok: &str) -> bool {
    let stem: String = strip_numeric_suffix(tok)
        .chars()
        .filter(|&c| c != '_')
        .collect();
    stem == "0.0" || stem == "1.0"
}

fn is_allowed_decimal_zero_or_one(tok: &str) -> bool {
    let stem: String = strip_numeric_suffix(tok)
        .chars()
        .filter(|&c| c != '_')
        .collect();
    stem == "0" || stem == "1"
}

fn is_allowed_radix_zero_or_one(tok: &str) -> bool {
    let stem = strip_numeric_suffix(tok);
    if stem.len() < 3 {
        return false;
    }
    let radix = match stem.as_bytes()[1] {
        b'x' | b'X' => 16,
        b'b' | b'B' => 2,
        b'o' | b'O' => 8,
        _ => return false,
    };
    let digits: String = stem[2..].chars().filter(|&c| c != '_').collect();
    u128::from_str_radix(&digits, radix).is_ok_and(|v| v <= 1)
}

fn strip_rust_comments_and_strings(bytes: &[u8]) -> Vec<u8> {
    let mut out = vec![b' '; bytes.len()];
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'\n' {
            out[idx] = b'\n';
        }
    }

    let mut i = 0usize;
    while i < bytes.len() {
        // Line comment `//`:
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        // Nested block comment `/* ... */`:
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
            let mut depth = 1usize;
            i += 2;
            while i < bytes.len() && depth > 0 {
                if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        // Raw string `r#"..."#` or byte raw string `br#"..."#`:
        if let Some(after_raw) = skip_raw_string(bytes, i) {
            i = after_raw;
            continue;
        }
        // Byte string `b"..."`:
        if i + 1 < bytes.len() && bytes[i] == b'b' && bytes[i + 1] == b'"' {
            i = skip_quoted_string(bytes, i + 1);
            continue;
        }
        // Normal string `"..."`:
        if bytes[i] == b'"' {
            i = skip_quoted_string(bytes, i);
            continue;
        }
        // Byte char literal `b'x'`:
        if i + 1 < bytes.len() && bytes[i] == b'b' && bytes[i + 1] == b'\'' {
            if let Some(after_ch) = skip_char_literal(bytes, i + 1) {
                i = after_ch;
                continue;
            }
        }
        // Char literal `'x'` (distinguished from lifetime `'a`):
        if bytes[i] == b'\'' {
            if let Some(after_ch) = skip_char_literal(bytes, i) {
                i = after_ch;
                continue;
            }
        }

        out[i] = bytes[i];
        i += 1;
    }
    out
}

fn skip_quoted_string(bytes: &[u8], quote_at: usize) -> usize {
    let mut i = quote_at + 1;
    while i < bytes.len() {
        if bytes[i] == b'\\' {
            i += 2;
            continue;
        }
        if bytes[i] == b'"' {
            return i + 1;
        }
        i += 1;
    }
    bytes.len()
}

fn skip_raw_string(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    if i < bytes.len() && bytes[i] == b'b' {
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'r' {
        return None;
    }
    if start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_') {
        return None;
    }
    i += 1;
    let mut hashes = 0usize;
    while i < bytes.len() && bytes[i] == b'#' {
        hashes += 1;
        i += 1;
    }
    if i >= bytes.len() || bytes[i] != b'"' {
        return None;
    }
    i += 1;
    while i < bytes.len() {
        if bytes[i] == b'"'
            && i + hashes < bytes.len()
            && bytes[i + 1..=i + hashes].iter().all(|&b| b == b'#')
        {
            return Some(i + 1 + hashes);
        }
        i += 1;
    }
    Some(bytes.len())
}

fn skip_char_literal(bytes: &[u8], quote_at: usize) -> Option<usize> {
    let i = quote_at + 1;
    if i >= bytes.len() {
        return None;
    }
    if bytes[i] == b'\\' {
        let mut j = i + 2;
        while j < bytes.len() && j <= i + 10 {
            if bytes[j] == b'\'' {
                return Some(j + 1);
            }
            if bytes[j] == b'\n' {
                return None;
            }
            j += 1;
        }
        return None;
    }
    // Single UTF-8 scalar followed immediately by `'`:
    let rest = std::str::from_utf8(&bytes[i..]).ok()?;
    let ch = rest.chars().next()?;
    let close = i + ch.len_utf8();
    if close < bytes.len() && bytes[close] == b'\'' {
        Some(close + 1)
    } else {
        None
    }
}

fn blank_cfg_test_items(buf: &mut [u8]) {
    let needle = b"#[cfg(test)]";
    let mut i = 0usize;
    while i + needle.len() <= buf.len() {
        if &buf[i..i + needle.len()] == needle {
            let attr_start = i;
            let mut j = i + needle.len();
            while j < buf.len() && buf[j] != b'{' && buf[j] != b';' {
                j += 1;
            }
            let end = if j < buf.len() && buf[j] == b'{' {
                let mut depth = 1usize;
                let mut k = j + 1;
                while k < buf.len() && depth > 0 {
                    if buf[k] == b'{' {
                        depth += 1;
                    } else if buf[k] == b'}' {
                        depth -= 1;
                    }
                    k += 1;
                }
                k
            } else {
                (j + 1).min(buf.len())
            };
            for slot in &mut buf[attr_start..end] {
                if *slot != b'\n' {
                    *slot = b' ';
                }
            }
            i = end;
            continue;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{scan_lex_magic_literals, scan_rust_magic_literals};

    #[test]
    fn clean_zero_one_rust_passes_and_magic_constants_are_caught() {
        let clean = r#"
            // Comment mentioning 0.161 and 42 and 5126
            const HEADER: &[u8; 4] = b"glTF"; // wait: `4` in array length would be caught!
        "#;
        assert_eq!(scan_rust_magic_literals(clean), vec![(3, "4".to_string())]);

        let purely_parametric = r#"
            /// Sabine RT60 without magic 0.161 constant.
            pub fn example(seed: u64) -> f64 {
                let two = 1.0 + 1.0;
                let zero_u = 0_u64;
                let one_u = 1_usize;
                let msg = "WGSL 60fps 1920x1080 0.161";
                let ch = '\x07';
                if seed == zero_u || one_u == 0 {
                    0.0_f64
                } else {
                    1.0 / two
                }
            }

            #[cfg(test)]
            mod tests {
                #[test]
                fn allows_test_expectations() {
                    assert_eq!(2 + 2, 4);
                    let tol = 1e-12;
                    assert!(tol < 0.5);
                }
            }
        "#;
        assert!(scan_rust_magic_literals(purely_parametric).is_empty());

        let planted = "pub fn bad() -> f64 { 0.161 * 60 as f64 }";
        assert_eq!(
            scan_rust_magic_literals(planted),
            vec![(1, "0.161".to_string()), (1, "60".to_string())]
        );
    }

    #[test]
    fn lex_semantic_module_catches_literals_gt_one() {
        let clean_lex = "\\begin{lexlean}{M}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\begin{semanticmodule}\ndefinition{\n  name: two,\n  parameters: [],\n  result: Nat,\n  body: (1 + 1)\n}\n\\end{semanticmodule}\n\\end{lexlean}\n";
        assert!(scan_lex_magic_literals(clean_lex).is_empty());

        let dirty_lex = "\\begin{lexlean}{M}\n\\useglossary{lexlean.std.nat@1.2.0}\n\\begin{semanticmodule}\ndefinition{\n  name: magic,\n  parameters: [],\n  result: Nat,\n  body: (42 + 1)\n}\n\\end{semanticmodule}\n\\end{lexlean}\n";
        assert_eq!(
            scan_lex_magic_literals(dirty_lex),
            vec![(8, "42".to_string())]
        );
    }
}
