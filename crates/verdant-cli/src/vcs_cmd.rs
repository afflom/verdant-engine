//! `verdant vcs ...`: host a Git repository's full history on Stitch and read it back
//! (`vcs_cmd.rs`, `AUTH-GIT-OBJECT-FORMAT`).

use super::*;
use std::collections::BTreeMap;
use verdant_core::vcs_git::{GitHostingPin, VerdantGitHistory};

const VCS_USAGE: &str = "verdant vcs git-host --pack <history.pack> --refs <for-each-ref.txt> --head <refname> --repository <owner/name> --out <dir> [--relay <url>]...\n\
verdant vcs git-fetch --pin <pin.json> --shards <dir> --git-dir <bare.git>\n\
verdant vcs git-extract --pin <pin.json> --shards <dir> --commit <id> [--path <prefix>] --out <dir>";

fn io_err(path: &std::path::Path, e: &std::io::Error) -> CliCommandError {
    CliCommandError::IoFailure { path: path.display().to_string(), reason: e.to_string() }
}

fn rejected(command: &str, e: impl std::fmt::Display) -> CliCommandError {
    CliCommandError::ValidationRejected { command: command.to_string(), diagnostic: e.to_string() }
}

pub(crate) fn run_vcs_command(args: &[String]) -> Result<String, CliCommandError> {
    let two = 1usize + 1usize;
    let mut opts: BTreeMap<String, String> = BTreeMap::new();
    let mut relays = Vec::new();
    let mut i = two;
    while i + 1 < args.len() {
        if args[i] == "--relay" {
            relays.push(args[i + 1].trim_end_matches('/').to_string());
        } else {
            opts.insert(args[i].trim_start_matches("--").to_string(), args[i + 1].clone());
        }
        i += two;
    }
    let need = |k: &str| {
        opts.get(k).cloned().ok_or_else(|| CliCommandError::InvalidUsage { usage: VCS_USAGE.to_string() })
    };
    match args.get(1).map(String::as_str) {
        Some("git-host") => {
            let pack_p = std::path::PathBuf::from(need("pack")?);
            let refs_p = std::path::PathBuf::from(need("refs")?);
            let out = std::path::PathBuf::from(need("out")?);
            let pack = std::fs::read(&pack_p).map_err(|e| io_err(&pack_p, &e))?;
            let refs = std::fs::read_to_string(&refs_p).map_err(|e| io_err(&refs_p, &e))?;
            let (h, report) =
                VerdantGitHistory::import(pack, &refs, &need("head")?).map_err(|e| rejected("vcs git-host", e))?;
            let objects = report.objects;
            let k = 1u8 << two;
            let n = k << 1;
            let (shards, pin) =
                h.host(&need("repository")?, k, n, relays).map_err(|e| rejected("vcs git-host", e))?;
            std::fs::create_dir_all(&out).map_err(|e| io_err(&out, &e))?;
            for s in &shards {
                let p = out.join(format!("shard-{}.stch", s.index));
                std::fs::write(&p, s.to_binary_stch2()).map_err(|e| io_err(&p, &e))?;
            }
            let pin_p = out.join("pin.json");
            let json = pin.to_json().map_err(|e| rejected("vcs git-host", e))?;
            std::fs::write(&pin_p, json + "\n").map_err(|e| io_err(&pin_p, &e))?;
            Ok(format!(
                "GIT HISTORY HOSTED: repository={} objects={objects} commits={} merges={} refs={} capsule={} bytes={} shards={k}of{n}",
                pin.repository, pin.closure.commits, pin.closure.merge_commits, pin.closure.refs, pin.capsule_kappa, pin.capsule_byte_len
            ))
        }
        Some("git-fetch") => {
            let git_dir = std::path::PathBuf::from(need("git-dir")?);
            let (pin, shards, h) = recover(&need("pin")?, &need("shards")?, "vcs git-fetch")?;
            let written = h.write_bare_repository(&git_dir).map_err(|e| rejected("vcs git-fetch", e))?;
            Ok(format!(
                "GIT HISTORY FETCHED: repository={} capsule={} shards_used={} objects={written} head={} git_dir={}",
                pin.repository, pin.capsule_kappa, shards.len(), h.head, git_dir.display()
            ))
        }
        Some("git-extract") => {
            let out = std::path::PathBuf::from(need("out")?);
            let commit = need("commit")?;
            let path = opts.get("path").cloned().unwrap_or_default();
            let (pin, _, h) = recover(&need("pin")?, &need("shards")?, "vcs git-extract")?;
            let n = h.extract(&commit, &path, &out).map_err(|e| rejected("vcs git-extract", e))?;
            Ok(format!(
                "GIT TREE EXTRACTED: repository={} capsule={} commit={commit} path={path} files={n} out={}",
                pin.repository, pin.capsule_kappa, out.display()
            ))
        }
        _ => Err(CliCommandError::InvalidUsage { usage: VCS_USAGE.to_string() }),
    }
}

/// Reads the pin and whichever of its shards are present, and recovers the verified history.
fn recover(
    pin_path: &str,
    shard_dir: &str,
    command: &str,
) -> Result<(GitHostingPin, Vec<verdant_core::network::StitchRsShard>, VerdantGitHistory), CliCommandError> {
    let pin_p = std::path::PathBuf::from(pin_path);
    let raw = std::fs::read_to_string(&pin_p).map_err(|e| io_err(&pin_p, &e))?;
    let pin = GitHostingPin::from_json(&raw).map_err(|e| rejected(command, e))?;
    let dir = std::path::PathBuf::from(shard_dir);
    let mut shards = Vec::new();
    for f in &pin.shard_files {
        if let Ok(bytes) = std::fs::read(dir.join(f)) {
            if let Ok(s) = verdant_core::network::StitchRsShard::from_binary_stch2(&bytes) {
                shards.push(s);
            }
        }
        if shards.len() == usize::from(pin.data_shards_k) {
            break;
        }
    }
    let h = VerdantGitHistory::recover(&shards, &pin).map_err(|e| rejected(command, e))?;
    Ok((pin, shards, h))
}
