//! The proof `just app` keeps (`lexlean/attestation/4` + Holo v6 self-carrying proof record),
//! and the read-time check a run makes before reusing a kept proof (`M29`).

use std::path::Path;
use std::process::Command;

use crate::Fail;
use serde_json::Value;

const KEPT: &str = ".prism-proof";
const EVIDENCE: [&str; 2] = ["attestation.json", "build-manifest.json"];
const INPUT_PATHS: [&str; 5] = [
    "app",
    "template.lock",
    "Justfile",
    "rust-toolchain.toml",
    "scripts/build-holo-v6.mjs",
];

struct Kept {
    attestation: String,
    lexlean: Vec<u8>,
    build_manifest: Vec<u8>,
    inputs: String,
}

struct Observed {
    inputs: String,
    source_id: String,
    semantic_id: String,
    arch: String,
    workspace: Vec<(String, String)>,
}

pub(crate) fn blake3_hex(bytes: &[u8]) -> String {
    const IV: [u32; 8] = [
        0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A, 0x510E527F, 0x9B05688C, 0x1F83D9AB,
        0x5BE0CD19,
    ];
    const PERM: [usize; 16] = [2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8];
    macro_rules! g {
        ($s:expr, $a:expr, $b:expr, $c:expr, $d:expr, $mx:expr, $my:expr) => {{
            $s[$a] = $s[$a].wrapping_add($s[$b]).wrapping_add($mx);
            $s[$d] = ($s[$d] ^ $s[$a]).rotate_right(16);
            $s[$c] = $s[$c].wrapping_add($s[$d]);
            $s[$b] = ($s[$b] ^ $s[$c]).rotate_right(12);
            $s[$a] = $s[$a].wrapping_add($s[$b]).wrapping_add($my);
            $s[$d] = ($s[$d] ^ $s[$a]).rotate_right(8);
            $s[$c] = $s[$c].wrapping_add($s[$d]);
            $s[$b] = ($s[$b] ^ $s[$c]).rotate_right(7);
        }};
    }
    macro_rules! compress {
        ($cv:expr, $bw:expr, $counter:expr, $blen:expr, $flags:expr) => {{
            let cv_r: &[u32; 8] = $cv;
            let mut s = [0_u32; 16];
            s[..8].copy_from_slice(cv_r);
            s[8..12].copy_from_slice(&IV[..4]);
            let ctr: u64 = $counter;
            s[12] = ctr as u32;
            s[13] = (ctr >> 32) as u32;
            s[14] = $blen;
            s[15] = $flags;
            let mut m: [u32; 16] = $bw;
            for r in 0..7 {
                g!(s, 0, 4, 8, 12, m[0], m[1]);
                g!(s, 1, 5, 9, 13, m[2], m[3]);
                g!(s, 2, 6, 10, 14, m[4], m[5]);
                g!(s, 3, 7, 11, 15, m[6], m[7]);
                g!(s, 0, 5, 10, 15, m[8], m[9]);
                g!(s, 1, 6, 11, 12, m[10], m[11]);
                g!(s, 2, 7, 8, 13, m[12], m[13]);
                g!(s, 3, 4, 9, 14, m[14], m[15]);
                if r < 6 {
                    let mut next = [0_u32; 16];
                    for j in 0..16 {
                        next[j] = m[PERM[j]];
                    }
                    m = next;
                }
            }
            for i in 0..8 {
                s[i] ^= s[i + 8];
                s[i + 8] ^= cv_r[i];
            }
            s
        }};
    }
    macro_rules! words {
        ($slice:expr) => {{
            let sl: &[u8] = $slice;
            let mut buf = [0_u8; 64];
            buf[..sl.len()].copy_from_slice(sl);
            let mut wout = [0_u32; 16];
            for i in 0..16 {
                wout[i] = u32::from_le_bytes(buf[i * 4..(i + 1) * 4].try_into().unwrap());
            }
            wout
        }};
    }
    macro_rules! first8 {
        ($s:expr) => {{
            let sv = $s;
            let mut f8 = [0_u32; 8];
            f8.copy_from_slice(&sv[..8]);
            f8
        }};
    }
    macro_rules! chunk_out {
        ($chunk:expr, $counter:expr) => {{
            let ch: &[u8] = $chunk;
            let ctr_c: u64 = $counter;
            let mut cv = IV;
            if ch.is_empty() {
                (cv, [0_u32; 16], ctr_c, 0_u32, 3_u32)
            } else {
                let mut pos = 0usize;
                let mut bi = 0usize;
                while pos + 64 < ch.len() {
                    let w = words!(&ch[pos..pos + 64]);
                    cv = first8!(compress!(&cv, w, ctr_c, 64, if bi == 0 { 1 } else { 0 }));
                    pos += 64;
                    bi += 1;
                }
                let tail = &ch[pos..];
                (
                    cv,
                    words!(tail),
                    ctr_c,
                    tail.len() as u32,
                    2 | if bi == 0 { 1 } else { 0 },
                )
            }
        }};
    }
    let root_node = if bytes.len() <= 1024 {
        chunk_out!(bytes, 0)
    } else {
        let mut stack: Vec<[u32; 8]> = Vec::new();
        let mut pos = 0usize;
        let mut cnt = 0_u64;
        while pos + 1024 < bytes.len() {
            let (c_cv, c_w, c_cnt, c_len, c_flg) = chunk_out!(&bytes[pos..pos + 1024], cnt);
            let mut new_cv = first8!(compress!(&c_cv, c_w, c_cnt, c_len, c_flg));
            let mut tot = cnt + 1;
            while (tot & 1) == 0 {
                if let Some(left) = stack.pop() {
                    let mut pw = [0_u32; 16];
                    pw[..8].copy_from_slice(&left);
                    pw[8..].copy_from_slice(&new_cv);
                    new_cv = first8!(compress!(&IV, pw, 0, 64, 4));
                }
                tot >>= 1;
            }
            stack.push(new_cv);
            pos += 1024;
            cnt += 1;
        }
        let mut out = chunk_out!(&bytes[pos..], cnt);
        while let Some(left) = stack.pop() {
            let rc = first8!(compress!(&out.0, out.1, out.2, out.3, out.4));
            let mut pw = [0_u32; 16];
            pw[..8].copy_from_slice(&left);
            pw[8..].copy_from_slice(&rc);
            out = (IV, pw, 0, 64, 4);
        }
        out
    };
    let s = compress!(&root_node.0, root_node.1, 0, root_node.3, root_node.4 | 8);
    let mut out = [0_u8; 32];
    for i in 0..8 {
        out[i * 4..(i + 1) * 4].copy_from_slice(&s[i].to_le_bytes());
    }
    let mut hex = String::with_capacity(64);
    for b in out {
        use std::fmt::Write as _;
        let _ = write!(hex, "{b:02x}");
    }
    hex
}

fn binds(kept: &Kept, observed: &Observed) -> Result<(), String> {
    if kept.attestation.len() != 64
        || !kept
            .attestation
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err("the kept attestation ID is not a 64-hex BLAKE3 digest".into());
    }
    macro_rules! text_at {
        ($v:expr, $ptr:expr) => {
            $v.pointer($ptr)
                .and_then(Value::as_str)
                .ok_or_else(|| format!("the attestation has no {}", $ptr))?
        };
    }
    macro_rules! same_val {
        ($what:expr, $k:expr, $o:expr) => {
            if $k != $o {
                return Err(format!(
                    "{} differs: the kept proof has {}, this run has {}",
                    $what, $k, $o
                ));
            }
        };
    }
    let lexlean = serde_json::from_slice::<Value>(&kept.lexlean)
        .map_err(|error| format!("attestation.json: {error}"))?;
    same_val!(
        "the attestation spec",
        text_at!(&lexlean, "/spec"),
        "lexlean/attestation/4"
    );
    same_val!(
        "the LexLean status",
        text_at!(&lexlean, "/status"),
        "verified"
    );
    same_val!(
        "the attestation ID",
        text_at!(&lexlean, "/attestation_id"),
        &format!("blake3:{}", kept.attestation)
    );
    same_val!(
        "the build manifest kappa",
        text_at!(&lexlean, "/build_manifest/kappa"),
        &format!("blake3:{}", blake3_hex(&kept.build_manifest))
    );
    same_val!(
        "the source ID",
        text_at!(&lexlean, "/source_id"),
        &observed.source_id
    );
    same_val!(
        "the semantic ID",
        text_at!(&lexlean, "/semantic_id"),
        &observed.semantic_id
    );
    same_val!(
        "the architecture",
        text_at!(&lexlean, "/host/arch"),
        &observed.arch
    );
    let rows = lexlean
        .get("lake_workspace")
        .and_then(Value::as_array)
        .ok_or("the attestation names no Lake workspace")?;
    if rows.len() != observed.workspace.len() {
        return Err("the Lake workspace differs".into());
    }
    for (row, (path, kappa)) in rows.iter().zip(&observed.workspace) {
        same_val!("a Lake workspace path", text_at!(row, "/path"), path);
        same_val!(&format!("app/{path}"), text_at!(row, "/kappa"), kappa);
    }
    let recorded = kept
        .inputs
        .strip_prefix(&format!("attestation {}\n", kept.attestation))
        .ok_or("the kept inputs name another attestation")?;
    if recorded != observed.inputs {
        return Err("the proof inputs differ from the kept proof's".into());
    }
    Ok(())
}

fn run(root: &Path, dir: &str, program: &str, args: &[&str]) -> Result<String, Fail> {
    let output = Command::new(program)
        .args(args)
        .current_dir(root.join(dir))
        .output()
        .map_err(|error| format!("{program}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(String::from_utf8(output.stdout)?)
}

fn inputs(root: &Path) -> Result<String, Fail> {
    let devcontainer =
        std::fs::read_to_string(root.join(".devcontainer/devcontainer.json")).unwrap_or_default();
    let lean_tc = std::fs::read_to_string(root.join("app/lean-toolchain")).unwrap_or_default();
    let mut out = format!(
        "devcontainer {}\nlean_toolchain {}lexlean {}arch {}\n",
        blake3_hex(devcontainer.as_bytes()),
        lean_tc,
        run(root, ".", "lexlean", &["--version"])?,
        std::env::consts::ARCH,
    );
    let mut args = vec!["ls-files", "-z", "--"];
    args.extend(INPUT_PATHS);
    let files = run(root, ".", "git", &args)?;
    for path in files.split('\0').filter(|path| !path.is_empty()) {
        let bytes = std::fs::read(root.join(path)).map_err(|error| format!("{path}: {error}"))?;
        out.push_str(&format!("{}  {path}\n", blake3_hex(&bytes)));
    }
    let mut args = vec!["status", "--porcelain=v1", "--untracked-files=all", "--"];
    args.extend(INPUT_PATHS);
    out.push_str(&run(root, ".", "git", &args)?);
    Ok(out)
}

fn observe(root: &Path, lexlean: &Value) -> Result<Observed, Fail> {
    let check: Value = serde_json::from_str(&run(
        root,
        "app",
        "lexlean",
        &["--diagnostic-format", "json", "check"],
    )?)?;
    if check.get("success") != Some(&Value::Bool(true)) {
        return Err("lexlean check did not succeed".into());
    }
    let rows = lexlean
        .get("lake_workspace")
        .and_then(Value::as_array)
        .ok_or("the attestation names no Lake workspace")?;
    let mut workspace = Vec::new();
    for row in rows {
        let path = row
            .pointer("/path")
            .and_then(Value::as_str)
            .ok_or("the attestation has no /path")?;
        if path.is_empty() || path.starts_with('/') || path.split('/').any(|part| part == "..") {
            return Err(format!("the attestation names a Lake file outside app/: {path}").into());
        }
        let bytes = std::fs::read(root.join("app").join(path))
            .map_err(|error| format!("app/{path}: {error}"))?;
        workspace.push((path.to_owned(), format!("blake3:{}", blake3_hex(&bytes))));
    }
    let source_id = check
        .pointer("/source_id")
        .and_then(Value::as_str)
        .ok_or("the attestation has no /source_id")?
        .to_owned();
    let semantic_id = check
        .pointer("/semantic_id")
        .and_then(Value::as_str)
        .ok_or("the attestation has no /semantic_id")?
        .to_owned();
    Ok(Observed {
        inputs: inputs(root)?,
        source_id,
        semantic_id,
        arch: std::env::consts::ARCH.to_owned(),
        workspace,
    })
}

pub fn reuse(root: &Path) -> Result<(), Fail> {
    let read_file = |path: &str| {
        std::fs::read(root.join(KEPT).join(path)).map_err(|error| format!("{KEPT}/{path}: {error}"))
    };
    let res = (|| -> Result<String, Fail> {
        let kept = Kept {
            attestation: String::from_utf8(read_file("attestation")?)?
                .trim_end()
                .to_owned(),
            lexlean: read_file("verified/attestation.json")?,
            build_manifest: read_file("verified/build-manifest.json")?,
            inputs: String::from_utf8(read_file("inputs")?)?,
        };
        let lexlean: Value = serde_json::from_slice(&kept.lexlean)?;
        binds(&kept, &observe(root, &lexlean)?)?;
        Ok(kept.attestation)
    })();
    match res {
        Ok(attestation) => {
            println!(
                "app: reused attestation blake3:{attestation}: it resolves and binds these exact proof inputs"
            );
            Ok(())
        }
        Err(error) => {
            println!("app: proving in full, the kept proof is not reusable: {error}");
            Err(error)
        }
    }
}

pub fn keep(root: &Path, attestation: &str) -> Result<(), Fail> {
    let raw_id = attestation.strip_prefix("blake3:").unwrap_or(attestation);
    if raw_id.len() != 64
        || !raw_id
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(
            format!("lexlean verify printed no 64-hex attestation ID: {attestation:?}").into(),
        );
    }
    let published = root.join("app/.lexlean/verified").join(raw_id);
    let kept = root.join(KEPT);
    if kept.exists() {
        std::fs::remove_dir_all(&kept)?;
    }
    std::fs::create_dir_all(kept.join("verified"))?;
    for name in EVIDENCE {
        std::fs::copy(published.join(name), kept.join("verified").join(name))
            .map_err(|error| format!("{}: {error}", published.join(name).display()))?;
    }
    std::fs::write(
        kept.join("inputs"),
        format!("attestation {raw_id}\n{}", inputs(root)?),
    )?;
    std::fs::write(kept.join("attestation"), format!("{raw_id}\n"))?;
    let readback = Kept {
        attestation: raw_id.to_owned(),
        lexlean: std::fs::read(kept.join("verified/attestation.json"))?,
        build_manifest: std::fs::read(kept.join("verified/build-manifest.json"))?,
        inputs: std::fs::read_to_string(kept.join("inputs"))?,
    };
    let lexlean: Value = serde_json::from_slice(&readback.lexlean)?;
    binds(&readback, &observe(root, &lexlean)?)
        .map_err(|error| format!("the proof just kept does not read back: {error}"))?;
    println!("app: kept attestation blake3:{raw_id} with the inputs it binds");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{binds, blake3_hex, Kept, Observed};
    use serde_json::json;

    const SOURCE: &str = "blake3:c573a7a43bdb1d0561da769cbd12fb9a9dea517fdb004165d5086d292d82a92c";
    const SEMANTIC: &str =
        "blake3:fbad6e6ba85b0d26526724c760b0080c4a274f9c113c6776357dfd636d3ee02d";
    const TOOLCHAIN_KAPPA: &str =
        "blake3:8e3538e0ab5f81a3ee04927d8838c8c674e0e112838b4b3ce87ec218143276af";
    const INPUTS: &str =
        "devcontainer abcd\nlean_toolchain leanprover/lean4:v4.32.2\nlexlean 0.4.5\narch x86_64\n";

    fn seal() -> Kept {
        let build_manifest = b"{\"modules\":[]}".to_vec();
        let attestation = "1".repeat(64);
        let lexlean = serde_json::to_vec(&json!({
            "attestation_id": format!("blake3:{attestation}"),
            "build_id": format!("blake3:{}", "2".repeat(64)),
            "build_manifest": {
                "byte_length": build_manifest.len(),
                "kappa": format!("blake3:{}", blake3_hex(&build_manifest)),
            },
            "host": {"arch": "x86_64", "os": "linux"},
            "lake_workspace": [{"path": "lean-toolchain", "kappa": TOOLCHAIN_KAPPA}],
            "semantic_id": SEMANTIC,
            "source_id": SOURCE,
            "spec": "lexlean/attestation/4",
            "status": "verified",
        }))
        .unwrap();
        Kept {
            inputs: format!("attestation {attestation}\n{INPUTS}"),
            attestation,
            lexlean,
            build_manifest,
        }
    }

    fn observed() -> Observed {
        Observed {
            inputs: INPUTS.to_owned(),
            source_id: SOURCE.to_owned(),
            semantic_id: SEMANTIC.to_owned(),
            arch: "x86_64".to_owned(),
            workspace: vec![("lean-toolchain".to_owned(), TOOLCHAIN_KAPPA.to_owned())],
        }
    }

    #[test]
    fn a_proof_of_these_exact_inputs_is_reused() {
        assert_eq!(binds(&seal(), &observed()), Ok(()));
    }

    #[test]
    fn a_kept_proof_that_does_not_bind_these_inputs_is_proved_again() {
        let mut bad_id = seal();
        bad_id.attestation = "f".repeat(64);
        assert!(binds(&bad_id, &observed()).is_err());

        let mut bad_manifest = seal();
        bad_manifest.build_manifest.push(b' ');
        assert!(binds(&bad_manifest, &observed()).is_err());

        let mut changed_sem = observed();
        changed_sem.semantic_id = format!("blake3:{}", "5".repeat(64));
        assert!(binds(&seal(), &changed_sem).is_err());

        let mut changed_arch = observed();
        changed_arch.arch = "aarch64".to_owned();
        assert!(binds(&seal(), &changed_arch).is_err());
    }

    #[test]
    fn a_lake_file_outside_the_application_is_never_read() {
        let s = seal();
        let base: serde_json::Value = serde_json::from_slice(&s.lexlean).unwrap();
        let check_ws = |val: &serde_json::Value| -> Result<(), String> {
            let rows = val["lake_workspace"].as_array().ok_or("missing")?;
            for row in rows {
                let p = row["path"].as_str().ok_or("missing path")?;
                if p.is_empty() || p.starts_with('/') || p.split('/').any(|part| part == "..") {
                    return Err(p.to_owned());
                }
            }
            Ok(())
        };
        assert!(check_ws(&base).is_ok());
        for path in ["../Justfile", "/etc/passwd", "a/../../b", ""] {
            let mut escape = base.clone();
            escape["lake_workspace"][0]["path"] = json!(path);
            assert!(check_ws(&escape).is_err(), "{path}");
        }
    }
}
