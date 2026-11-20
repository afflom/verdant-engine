//! `prism-fidelity`: score a Prism project, enforce the permanent zero-constants
//! parametric ratchet, and keep its fidelity register.
//!
//! ```text
//! prism-fidelity --project app --repo . --register fidelity.toml
//!                [--ratchet model/parametric-ratchet.toml]
//!                [--cite model/provenance.toml app/src/Provenance.lex.tex]
//!                [--measures model/measures.toml] [--lean app/src/Fidelity.lex.tex]
//!                [--json scorecard.json] [--check]
//! ```

use std::path::PathBuf;
use std::process::ExitCode;

use prism_fidelity::{file_kappa, lean, ratchet, scorecard, Register};

fn main() -> ExitCode {
    let mut project = PathBuf::from("app");
    let mut repo = PathBuf::from(".");
    let mut register = PathBuf::from("fidelity.toml");
    let mut ratchet_cfg: Option<PathBuf> = Some(PathBuf::from("model/parametric-ratchet.toml"));
    let mut lean_out: Option<PathBuf> = None;
    let mut json_out: Option<PathBuf> = None;
    let mut check = false;
    let mut cite: Option<(PathBuf, PathBuf)> = None;
    let mut measures: Option<PathBuf> = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        let mut path = |d: PathBuf| args.next().map(PathBuf::from).unwrap_or(d);
        match arg.as_str() {
            "--project" => project = path(project),
            "--repo" => repo = path(repo),
            "--register" => register = path(register),
            "--ratchet" => ratchet_cfg = Some(path(PathBuf::from("model/parametric-ratchet.toml"))),
            "--no-ratchet" => ratchet_cfg = None,
            "--lean" => lean_out = Some(path(PathBuf::new())),
            "--json" => json_out = Some(path(PathBuf::new())),
            "--check" => check = true,
            "--measures" => measures = Some(path(PathBuf::new())),
            "--cite" => {
                let list = path(PathBuf::new());
                let out = path(PathBuf::new());
                cite = Some((list, out));
            }
            other => {
                eprintln!("prism-fidelity: unknown argument {other}");
                return ExitCode::from(64);
            }
        }
    }
    if let Some((list, out)) = &cite {
        if let Err(why) = run_cite(&project, &repo, list, out, check) {
            eprintln!("prism-fidelity: {why}");
            return ExitCode::FAILURE;
        }
    }
    let passes = if check { 1 } else { 2 };
    let mut outcome = Ok(String::new());
    for pass in 0..passes {
        let outputs = Outputs {
            register: register.clone(),
            lean: lean_out.clone(),
            json: json_out.clone(),
            measures: measures.clone(),
        };
        outcome = run(pass + 1 < passes, &project, &repo, outputs, check);
        if outcome.is_err() {
            break;
        }
    }
    if let (Ok(_), Some(ratchet_path)) = (&outcome, &ratchet_cfg) {
        match ratchet::audit_parametric_ratchet(&repo, ratchet_path) {
            Ok(rep) => {
                let frontier_note = if rep.frontier_total_functions > 0 {
                    format!(
                        "; uorust frontier {}/{} lifted ({} residue, {} uncertified)",
                        rep.frontier_lifted_functions,
                        rep.frontier_total_functions,
                        rep.frontier_residue_functions,
                        rep.frontier_uncertified_functions
                    )
                } else {
                    String::new()
                };
                if rep.self_hosted_lifted_module_count > 0 {
                    println!(
                        "parametric-ratchet: 0 magic constants across {} LexLean strata modules and {} shipped Rust engine files; self-hosting ratchet 0 unlifted across {} lifted modules (blank frame {} B <= {} B){}",
                        rep.grounded_lex_module_count,
                        rep.audited_rust_file_count,
                        rep.self_hosted_lifted_module_count,
                        rep.blank_frame_total_bytes,
                        rep.blank_frame_max_bytes,
                        frontier_note
                    );
                } else {
                    println!(
                        "parametric-ratchet: 0 magic constants across {} LexLean strata modules and {} shipped Rust engine files{}",
                        rep.grounded_lex_module_count, rep.audited_rust_file_count, frontier_note
                    );
                }
            }
            Err(why) => {
                eprintln!("prism-fidelity: {why}");
                return ExitCode::FAILURE;
            }
        }
    }
    match outcome {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(why) => {
            eprintln!("prism-fidelity: {why}");
            ExitCode::FAILURE
        }
    }
}

fn run_cite(
    project: &std::path::Path,
    repo: &std::path::Path,
    list: &std::path::Path,
    out: &std::path::Path,
    check: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(list).map_err(|e| format!("{}: {e}", list.display()))?;
    let (module, derived) = prism_fidelity::cite::module(project, repo, &text)?;
    if check {
        if std::fs::read_to_string(out).unwrap_or_default() != module {
            return Err(format!(
                "{} is not what the sources cite: run prism-fidelity without --check",
                out.display()
            )
            .into());
        }
    } else {
        std::fs::write(out, module).map_err(|e| format!("{}: {e}", out.display()))?;
    }
    println!("{derived} records cited from their primary sources");
    Ok(())
}

struct Outputs {
    register: PathBuf,
    lean: Option<PathBuf>,
    json: Option<PathBuf>,
    measures: Option<PathBuf>,
}

fn run(
    rewriting: bool,
    project: &std::path::Path,
    repo: &std::path::Path,
    files: Outputs,
    check: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let Outputs {
        register,
        lean: lean_out,
        json: json_out,
        measures,
    } = files;
    let register = register.as_path();
    let specs = match &measures {
        Some(p) => prism_fidelity::measures(
            &std::fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))?,
        )?,
        None => Vec::new(),
    };
    let mut measured = Vec::new();
    for m in specs {
        let bytes =
            std::fs::read(repo.join(&m.source)).map_err(|e| format!("{}: {e}", m.source))?;
        let kappa = file_kappa(&m.source, &bytes);
        measured.push((m, kappa));
    }
    let ignore: &[&str] = if rewriting {
        &[prism_fidelity::FIDELITY_MODULE]
    } else {
        &[]
    };
    let score = prism_fidelity::score_ignoring(project, repo, ignore)?;
    let previous = match std::fs::read_to_string(register) {
        Ok(text) => Register::parse(&text)?,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound && !check => Register::default(),
        Err(e) => return Err(format!("{}: {e}", register.display()).into()),
    };
    let next = if rewriting {
        Register::next(&Register::default(), &score)?
    } else {
        Register::next(&previous, &score)?
    };
    let text = next.render();
    let location = register
        .strip_prefix(repo)
        .unwrap_or(register)
        .to_string_lossy()
        .replace('\\', "/");
    let kappa = file_kappa(&location, text.as_bytes());
    let mut outputs = Vec::new();
    if !rewriting {
        outputs.push((register.to_path_buf(), text));
    }
    if let Some(p) = lean_out {
        outputs.push((p, lean::module(&score, &location, &kappa, &measured)));
    }
    if let Some(p) = json_out.filter(|_| !rewriting) {
        outputs.push((p, scorecard(&score)));
    }
    for (path, text) in &outputs {
        if check {
            let committed = std::fs::read_to_string(path).unwrap_or_default();
            if &committed != text {
                return Err(format!(
                    "{} is not what the model scores to: run prism-fidelity without --check",
                    path.display()
                )
                .into());
            }
        } else {
            std::fs::write(path, text).map_err(|e| format!("{}: {e}", path.display()))?;
        }
    }
    Ok(prism_fidelity::summary(&score))
}
