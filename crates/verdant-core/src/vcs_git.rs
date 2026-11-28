//! **Git history hosting for VerdantVcs (`vcs_git.rs`)**: holds a repository's complete Git
//! history inside Verdant, so Verdant hosts it and GitHub is only a mirror. It writes the history
//! back out with every object id unchanged, for the `githubOriginMirror` carrier tier
//! (`AUTH-GIT-OBJECT-FORMAT`).
//!
//! The Git object and pack formats are the authority, and nothing is re-identified:
//! - An object id is the SHA-1 of `<type> SP <size> NUL <payload>` (gitformat-loose(5)).
//! - A commit names one `tree` and zero or more `parent` lines, so merges keep every parent.
//! - An annotated tag names one `object`.
//! - A tree entry is `<mode> SP <name> NUL <raw id>`. Mode `160000` is a gitlink, whose commit
//!   lives in another repository (gitformat-index(5)).
//! - History is carried as one version 2 pack: `PACK`, the version, the object count, then the
//!   entries, with delta entries resolved against an earlier base, and a SHA-1 trailer
//!   (gitformat-pack(5)).
//!
//! Every object is re-hashed and the closure of every ref is checked on import, on each recovery
//! from Stitch shards, and before export. Export writes the same pack, its version 2 `.idx`,
//! `packed-refs` and `HEAD` (gitrepository-layout(5)).

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::Read;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

use crate::digest::sha256_hex;
use crate::error::StitchCustodyError;

/// The four Git object types (gitformat-loose(5)).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GitObjectKind {
    /// File content.
    Blob,
    /// Directory listing.
    Tree,
    /// Revision with a tree and zero or more parents.
    Commit,
    /// Annotated tag.
    Tag,
}

impl GitObjectKind {
    /// The type word Git writes in an object header.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Blob => "blob",
            Self::Tree => "tree",
            Self::Commit => "commit",
            Self::Tag => "tag",
        }
    }
}

/// What [`VerdantGitHistory::verify`] found.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitClosureReport {
    /// Objects in the pack, each re-hashed.
    pub objects: usize,
    /// Commits in the pack.
    pub commits: usize,
    /// Commits with two or more parents.
    pub merge_commits: usize,
    /// Gitlink (submodule) entries, whose commits live in another repository.
    pub gitlinks: usize,
    /// Refs checked.
    pub refs: usize,
}

/// A repository's complete Git history: one pack plus its refs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerdantGitHistory {
    /// A version 2 Git pack holding every object reachable from `refs`.
    pub pack: Vec<u8>,
    /// Refs (`refs/heads/main`, `refs/tags/v1.0.0`, ...) to object ids.
    pub refs: BTreeMap<String, String>,
    /// The ref `HEAD` points at.
    pub head: String,
}

#[derive(Clone, Debug)]
struct PackEntry {
    offset: usize,
    end: usize,
    kind: GitObjectKind,
}

type PackIndex = (BTreeMap<String, PackEntry>, BTreeMap<String, Vec<String>>, GitClosureReport);

fn refuse(detail: String) -> StitchCustodyError {
    StitchCustodyError::PayloadDigestMismatch { detail }
}

fn truncated() -> StitchCustodyError {
    refuse("pack is truncated".to_string())
}

const TWO: usize = 1 + 1;
const FOUR: usize = TWO * TWO;
const EIGHT: usize = FOUR + FOUR;
const SEVEN: usize = FOUR + TWO + 1;
const SEVEN_BITS: u8 = (1u8 << SEVEN) - 1;
const MORE: u8 = 1u8 << SEVEN;

fn id_len() -> usize {
    <Sha1 as Digest>::output_size()
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * TWO);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn unhex(id: &str) -> Result<Vec<u8>, StitchCustodyError> {
    if id.len() != id_len() * TWO {
        return Err(refuse(format!("`{id}` is not a Git object id")));
    }
    (0..id.len())
        .step_by(TWO)
        .map(|i| {
            u8::from_str_radix(&id[i..i + TWO], 1 << FOUR)
                .map_err(|_| refuse(format!("`{id}` is not a Git object id")))
        })
        .collect()
}

fn be_u32(bytes: &[u8], at: usize) -> Result<u32, StitchCustodyError> {
    let b = bytes.get(at..at + FOUR).ok_or_else(truncated)?;
    Ok(u32::from_be_bytes([b[0], b[1], b[TWO], b[TWO + 1]]))
}

/// The Git object id of `payload` as a `kind` object.
#[must_use]
pub fn git_object_id(kind: GitObjectKind, payload: &[u8]) -> String {
    let mut h = Sha1::new();
    h.update(format!("{} {}\0", kind.word(), payload.len()).as_bytes());
    h.update(payload);
    hex(&h.finalize())
}

/// Parses tree entries into `(mode, name, id)`.
pub fn parse_tree(payload: &[u8]) -> Result<Vec<(String, Vec<u8>, String)>, StitchCustodyError> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < payload.len() {
        let sp = payload[i..]
            .iter()
            .position(|&c| c == b' ')
            .ok_or_else(|| refuse("tree entry has no mode".to_string()))?;
        let mode = String::from_utf8_lossy(&payload[i..i + sp]).into_owned();
        let name_start = i + sp + 1;
        let nul = payload[name_start..]
            .iter()
            .position(|&c| c == 0)
            .ok_or_else(|| refuse("tree entry has no name terminator".to_string()))?;
        let name = payload[name_start..name_start + nul].to_vec();
        let id_start = name_start + nul + 1;
        let id_end = id_start + id_len();
        if id_end > payload.len() {
            return Err(refuse("tree entry id is truncated".to_string()));
        }
        out.push((mode, name, hex(&payload[id_start..id_end])));
        i = id_end;
    }
    Ok(out)
}

/// The `tree` and `parent` ids a commit names, or the `object` id a tag names, in order.
fn header_links(kind: GitObjectKind, payload: &[u8]) -> Vec<String> {
    let end = payload
        .windows(TWO)
        .position(|w| w == b"\n\n")
        .unwrap_or(payload.len());
    let head = String::from_utf8_lossy(&payload[..end]);
    let keys: &[&str] = match kind {
        GitObjectKind::Commit => &["tree ", "parent "],
        GitObjectKind::Tag => &["object "],
        _ => &[],
    };
    head.lines()
        .filter_map(|l| keys.iter().find_map(|k| l.strip_prefix(k)))
        .map(|id| id.trim().to_string())
        .collect()
}

/// Inflates one zlib stream starting at `at`; returns the bytes and where the stream ended.
fn inflate_at(pack: &[u8], at: usize, size: usize) -> Result<(Vec<u8>, usize), StitchCustodyError> {
    let mut z = flate2::bufread::ZlibDecoder::new(pack.get(at..).ok_or_else(truncated)?);
    let mut out = Vec::with_capacity(size);
    z.read_to_end(&mut out)
        .map_err(|e| refuse(format!("pack entry at {at} does not inflate: {e}")))?;
    if out.len() != size {
        return Err(refuse(format!("pack entry at {at} inflates to {} bytes, not {size}", out.len())));
    }
    let consumed = usize::try_from(z.total_in()).map_err(|_| truncated())?;
    Ok((out, at + consumed))
}

/// Reads a delta's size (gitformat-pack(5): little-endian groups of seven bits).
fn delta_varint(d: &[u8], i: &mut usize) -> Result<usize, StitchCustodyError> {
    let mut v = 0usize;
    let mut shift = 0usize;
    loop {
        let b = *d.get(*i).ok_or_else(|| refuse("delta header is truncated".to_string()))?;
        *i += 1;
        v |= usize::from(b & SEVEN_BITS) << shift;
        shift += SEVEN;
        if b & MORE == 0 {
            return Ok(v);
        }
    }
}

/// Applies a Git delta to `base` (gitformat-pack(5) "Deltified representation").
fn apply_delta(base: &[u8], delta: &[u8]) -> Result<Vec<u8>, StitchCustodyError> {
    let bad = |what: &str| refuse(format!("delta {what}"));
    let mut i = 0usize;
    let src = delta_varint(delta, &mut i)?;
    let dst = delta_varint(delta, &mut i)?;
    if src != base.len() {
        return Err(refuse(format!("delta wants a {src}-byte base, has {}", base.len())));
    }
    let mut out = Vec::with_capacity(dst);
    while i < delta.len() {
        let op = delta[i];
        i += 1;
        if op & MORE != 0 {
            let mut off = 0usize;
            let mut len = 0usize;
            for bit in 0..FOUR {
                if op & (1 << bit) != 0 {
                    off |= usize::from(*delta.get(i).ok_or_else(|| bad("copy is truncated"))?) << (bit * EIGHT);
                    i += 1;
                }
            }
            for bit in 0..(TWO + 1) {
                if op & (1 << (FOUR + bit)) != 0 {
                    len |= usize::from(*delta.get(i).ok_or_else(|| bad("copy is truncated"))?) << (bit * EIGHT);
                    i += 1;
                }
            }
            if len == 0 {
                len = 1 << (FOUR * FOUR);
            }
            out.extend_from_slice(base.get(off..off + len).ok_or_else(|| bad("copies outside its base"))?);
        } else if op != 0 {
            let n = usize::from(op);
            out.extend_from_slice(delta.get(i..i + n).ok_or_else(|| bad("insert is truncated"))?);
            i += n;
        } else {
            return Err(bad("has the reserved opcode 0"));
        }
    }
    if out.len() != dst {
        return Err(refuse(format!("delta made {} bytes, promised {dst}", out.len())));
    }
    Ok(out)
}

/// Pack entry types (gitformat-pack(5)): 1 commit, 2 tree, 3 blob, 4 tag, 6 ofs-delta, 7 ref-delta.
enum RawKind {
    Whole(GitObjectKind),
    OfsDelta,
    RefDelta,
}

fn raw_kind(code: u8) -> Result<RawKind, StitchCustodyError> {
    let t = usize::from(code);
    Ok(match t {
        1 => RawKind::Whole(GitObjectKind::Commit),
        TWO => RawKind::Whole(GitObjectKind::Tree),
        FOUR => RawKind::Whole(GitObjectKind::Tag),
        SEVEN => RawKind::RefDelta,
        _ if t == TWO + 1 => RawKind::Whole(GitObjectKind::Blob),
        _ if t == FOUR + TWO => RawKind::OfsDelta,
        _ => return Err(refuse(format!("pack has unknown entry type {t}"))),
    })
}

struct EntryHead {
    kind: RawKind,
    size: usize,
    base_offset: Option<usize>,
    base_id: Option<String>,
    data: usize,
}

fn entry_head(pack: &[u8], at: usize) -> Result<EntryHead, StitchCustodyError> {
    let mut i = at;
    let mut b = *pack.get(i).ok_or_else(truncated)?;
    i += 1;
    let kind = raw_kind((b >> FOUR) & ((1 << (TWO + 1)) - 1))?;
    let mut size = usize::from(b & ((1 << FOUR) - 1));
    let mut shift = FOUR;
    while b & MORE != 0 {
        b = *pack.get(i).ok_or_else(truncated)?;
        i += 1;
        size |= usize::from(b & SEVEN_BITS) << shift;
        shift += SEVEN;
    }
    let (mut base_offset, mut base_id) = (None, None);
    match kind {
        RawKind::OfsDelta => {
            let mut c = *pack.get(i).ok_or_else(truncated)?;
            i += 1;
            let mut back = usize::from(c & SEVEN_BITS);
            while c & MORE != 0 {
                c = *pack.get(i).ok_or_else(truncated)?;
                i += 1;
                back = ((back + 1) << SEVEN) | usize::from(c & SEVEN_BITS);
            }
            base_offset = Some(
                at.checked_sub(back)
                    .ok_or_else(|| refuse(format!("delta at {at} points before the pack")))?,
            );
        }
        RawKind::RefDelta => {
            base_id = Some(hex(pack.get(i..i + id_len()).ok_or_else(truncated)?));
            i += id_len();
        }
        RawKind::Whole(_) => {}
    }
    Ok(EntryHead { kind, size, base_offset, base_id, data: i })
}

impl VerdantGitHistory {
    /// Takes a version 2 pack (`git pack-objects --stdout`), the refs
    /// (`git for-each-ref --format='%(objectname) %(refname)' refs/heads refs/tags refs/pull`,
    /// keeping branches, tags and pull request heads `refs/pull/<n>/head`, whose commits a merge
    /// may have squashed away) and the ref `HEAD` names, and refuses unless every object re-hashes and every ref's history is complete.
    pub fn import(pack: Vec<u8>, for_each_ref: &str, head: &str) -> Result<(Self, GitClosureReport), StitchCustodyError> {
        let mut refs = BTreeMap::new();
        for line in for_each_ref.lines().filter(|l| !l.trim().is_empty()) {
            let (id, name) = line
                .split_once(' ')
                .ok_or_else(|| refuse(format!("ref line `{line}` is not `<id> <refname>`")))?;
            let _ = unhex(id)?;
            let pull_head = name
                .strip_prefix("refs/pull/")
                .and_then(|r| r.strip_suffix("/head"))
                .is_some_and(|n| !n.is_empty() && n.bytes().all(|c| c.is_ascii_digit()));
            if !name.starts_with("refs/heads/") && !name.starts_with("refs/tags/") && !pull_head {
                return Err(refuse(format!("ref `{name}` is not a branch, tag or pull request head")));
            }
            refs.insert(name.to_string(), id.to_string());
        }
        let h = Self { pack, refs, head: head.trim().to_string() };
        let report = h.verify()?;
        Ok((h, report))
    }

    /// Resolves every pack entry in order, re-hashing each object, and checks the trailer. A
    /// delta base is cached only until the last delta that uses it, so memory follows the live
    /// bases rather than the whole history.
    fn index(&self) -> Result<PackIndex, StitchCustodyError> {
        let p = &self.pack;
        let trailer = p.len().checked_sub(id_len()).ok_or_else(truncated)?;
        if p.get(..FOUR) != Some(b"PACK".as_slice()) || be_u32(p, FOUR)? as usize != TWO {
            return Err(refuse("not a version 2 Git pack".to_string()));
        }
        let mut sum = Sha1::new();
        sum.update(&p[..trailer]);
        if sum.finalize().as_slice() != &p[trailer..] {
            return Err(refuse("pack trailer does not match its SHA-1".to_string()));
        }
        let count = be_u32(p, EIGHT)? as usize;
        let mut heads = Vec::with_capacity(count);
        let mut at = FOUR * (TWO + 1);
        for _ in 0..count {
            let eh = entry_head(p, at)?;
            let (_, end) = inflate_at(p, eh.data, eh.size)?;
            heads.push((at, end, eh));
            at = end;
        }
        if at != trailer {
            return Err(refuse("pack has bytes between its last entry and its trailer".to_string()));
        }
        let mut uses: HashMap<usize, usize> = HashMap::new();
        for (_, _, eh) in &heads {
            if let Some(b) = eh.base_offset {
                *uses.entry(b).or_default() += 1;
            }
        }
        let mut cache: HashMap<usize, (GitObjectKind, Rc<Vec<u8>>)> = HashMap::new();
        let mut entries: BTreeMap<String, PackEntry> = BTreeMap::new();
        let mut links = BTreeMap::new();
        let mut report = GitClosureReport::default();
        for (offset, end, eh) in heads {
            let (raw, _) = inflate_at(p, eh.data, eh.size)?;
            let (kind, payload) = match eh.kind {
                RawKind::Whole(k) => (k, raw),
                RawKind::OfsDelta | RawKind::RefDelta => {
                    let (bk, base) = match (eh.base_offset, &eh.base_id) {
                        (Some(b), _) => {
                            let hit = cache
                                .get(&b)
                                .cloned()
                                .ok_or_else(|| refuse(format!("delta at {offset} has no base at {b}")))?;
                            if let Some(n) = uses.get_mut(&b) {
                                *n -= 1;
                                if *n == 0 {
                                    cache.remove(&b);
                                }
                            }
                            hit
                        }
                        (None, Some(id)) => {
                            let (k, b) = self.object_from(&entries, id)?;
                            (k, Rc::new(b))
                        }
                        _ => return Err(refuse("delta without a base".to_string())),
                    };
                    (bk, apply_delta(&base, &raw)?)
                }
            };
            let id = git_object_id(kind, &payload);
            match kind {
                GitObjectKind::Commit => {
                    let l = header_links(kind, &payload);
                    report.commits += 1;
                    if l.len() > TWO {
                        report.merge_commits += 1;
                    }
                    links.insert(id.clone(), l);
                }
                GitObjectKind::Tag => {
                    links.insert(id.clone(), header_links(kind, &payload));
                }
                GitObjectKind::Tree => {
                    let mut l = Vec::new();
                    for (mode, _, child) in parse_tree(&payload)? {
                        if mode == "160000" {
                            report.gitlinks += 1;
                        } else {
                            l.push(child);
                        }
                    }
                    links.insert(id.clone(), l);
                }
                GitObjectKind::Blob => {}
            }
            if uses.get(&offset).copied().unwrap_or(0) > 0 {
                cache.insert(offset, (kind, Rc::new(payload)));
            }
            entries.insert(id, PackEntry { offset, end, kind });
        }
        report.objects = entries.len();
        Ok((entries, links, report))
    }

    /// Reads one object by id, resolving its delta chain from the pack.
    fn object_from(&self, entries: &BTreeMap<String, PackEntry>, id: &str) -> Result<(GitObjectKind, Vec<u8>), StitchCustodyError> {
        let e = entries
            .get(id)
            .ok_or_else(|| refuse(format!("object {id} is not in the history")))?;
        let mut chain: Vec<Vec<u8>> = Vec::new();
        let mut at = e.offset;
        loop {
            let eh = entry_head(&self.pack, at)?;
            let (raw, _) = inflate_at(&self.pack, eh.data, eh.size)?;
            match eh.kind {
                RawKind::Whole(k) => {
                    let mut obj = raw;
                    for d in chain.iter().rev() {
                        obj = apply_delta(&obj, d)?;
                    }
                    return Ok((k, obj));
                }
                RawKind::OfsDelta | RawKind::RefDelta => {
                    chain.push(raw);
                    at = match (eh.base_offset, eh.base_id) {
                        (Some(b), _) => b,
                        (None, Some(bid)) => entries
                            .get(&bid)
                            .ok_or_else(|| refuse(format!("delta base {bid} is not in the history")))?
                            .offset,
                        _ => return Err(refuse("delta without a base".to_string())),
                    };
                }
            }
        }
    }

    /// Re-hashes every object and refuses unless every ref's whole history is present.
    pub fn verify(&self) -> Result<GitClosureReport, StitchCustodyError> {
        Ok(self.verified()?.1)
    }

    fn verified(&self) -> Result<(BTreeMap<String, PackEntry>, GitClosureReport), StitchCustodyError> {
        if !self.refs.contains_key(&self.head) {
            return Err(refuse(format!("HEAD names `{}`, which is not a ref of this history", self.head)));
        }
        let (entries, links, mut report) = self.index()?;
        let mut seen = BTreeSet::new();
        let mut stack: Vec<String> = self.refs.values().cloned().collect();
        while let Some(id) = stack.pop() {
            if !seen.insert(id.clone()) {
                continue;
            }
            if !entries.contains_key(&id) {
                return Err(refuse(format!("history is missing reachable object {id}")));
            }
            if let Some(l) = links.get(&id) {
                stack.extend(l.iter().cloned());
            }
        }
        report.refs = self.refs.len();
        Ok((entries, report))
    }

    /// The parents of commit `id`, in order.
    pub fn commit_parents(&self, id: &str) -> Result<Vec<String>, StitchCustodyError> {
        let (entries, _) = self.verified()?;
        let (kind, c) = self.object_from(&entries, id)?;
        if kind != GitObjectKind::Commit {
            return Err(refuse(format!("{id} is not a commit")));
        }
        Ok(header_links(kind, &c).into_iter().skip(1).collect())
    }

    /// Extracts the files under `prefix` of commit `commit` into `out`, read straight from the
    /// hosted pack (a sparse checkout with no Git process). Returns the number of files written.
    pub fn extract(&self, commit: &str, prefix: &str, out: &std::path::Path) -> Result<usize, StitchCustodyError> {
        let (entries, _) = self.verified()?;
        let (kind, c) = self.object_from(&entries, commit)?;
        if kind != GitObjectKind::Commit {
            return Err(refuse(format!("{commit} is not a commit")));
        }
        let tree = header_links(kind, &c)
            .into_iter()
            .next()
            .ok_or_else(|| refuse(format!("commit {commit} names no tree")))?;
        let prefix = prefix.trim_matches('/');
        let mut written = 0usize;
        let mut stack = vec![(tree, String::new())];
        while let Some((tid, path)) = stack.pop() {
            let (_, t) = self.object_from(&entries, &tid)?;
            for (mode, name, id) in parse_tree(&t)? {
                let name = String::from_utf8(name).map_err(|_| refuse("tree entry name is not UTF-8".to_string()))?;
                if name.contains('/') || name == ".." || name == "." || name == ".git" {
                    return Err(refuse(format!("unsafe tree entry name `{name}`")));
                }
                let full = if path.is_empty() { name } else { format!("{path}/{name}") };
                let inside = prefix.is_empty() || full == prefix || full.starts_with(&format!("{prefix}/"));
                let above = prefix.starts_with(&format!("{full}/"));
                if mode == "40000" {
                    if inside || above {
                        stack.push((id, full));
                    }
                } else if inside && mode != "160000" {
                    let (_, bytes) = self.object_from(&entries, &id)?;
                    let dest = out.join(&full);
                    let io = |e: std::io::Error| refuse(format!("write {}: {e}", dest.display()));
                    if let Some(parent) = dest.parent() {
                        std::fs::create_dir_all(parent).map_err(io)?;
                    }
                    std::fs::write(&dest, &bytes).map_err(io)?;
                    #[cfg(unix)]
                    if mode == "100755" {
                        use std::os::unix::fs::PermissionsExt;
                        let exec_bits = (1u32 << (TWO * (TWO + 1))) | (1u32 << (TWO + 1)) | 1;
                        let mut perm = std::fs::metadata(&dest).map_err(io)?.permissions();
                        perm.set_mode(perm.mode() | exec_bits);
                        std::fs::set_permissions(&dest, perm).map_err(io)?;
                    }
                    written += 1;
                }
            }
        }
        Ok(written)
    }

    /// Writes the history into `git_dir` as a bare repository: the same pack, its version 2
    /// `.idx`, `packed-refs` and `HEAD`. Every object id written equals the imported id.
    pub fn write_bare_repository(&self, git_dir: &std::path::Path) -> Result<usize, StitchCustodyError> {
        let (entries, _) = self.verified()?;
        let io = |e: std::io::Error| refuse(format!("write {}: {e}", git_dir.display()));
        for sub in ["objects/info", "objects/pack", "refs/heads", "refs/tags", "info"] {
            std::fs::create_dir_all(git_dir.join(sub)).map_err(io)?;
        }
        std::fs::write(git_dir.join("config"), "[core]\n\trepositoryformatversion = 0\n\tbare = true\n").map_err(io)?;
        std::fs::write(git_dir.join("HEAD"), format!("ref: {}\n", self.head)).map_err(io)?;
        let trailer = &self.pack[self.pack.len() - id_len()..];
        let name = hex(trailer);
        // gitformat-pack(5) version 2 index: magic, version, 256 fan-out counts, sorted ids, the
        // CRC-32 of each packed entry, 31-bit offsets with a 64-bit table above 2^31, the pack
        // checksum, then the index's own SHA-1.
        let mut idx = vec![SEVEN_BITS | MORE, b't', b'O', b'c'];
        idx.extend_from_slice(&u32::try_from(TWO).unwrap_or(0).to_be_bytes());
        let ids: Vec<(Vec<u8>, &PackEntry)> = entries
            .iter()
            .map(|(id, e)| unhex(id).map(|b| (b, e)))
            .collect::<Result<_, _>>()?;
        let mut fan = vec![0u32; 1usize << EIGHT];
        for (b, _) in &ids {
            for slot in fan.iter_mut().skip(usize::from(b[0])) {
                *slot += 1;
            }
        }
        for f in &fan {
            idx.extend_from_slice(&f.to_be_bytes());
        }
        for (b, _) in &ids {
            idx.extend_from_slice(b);
        }
        for (_, e) in &ids {
            let mut crc = flate2::Crc::new();
            crc.update(&self.pack[e.offset..e.end]);
            idx.extend_from_slice(&crc.sum().to_be_bytes());
        }
        let high = FOUR * EIGHT - 1;
        let mut large = Vec::new();
        for (_, e) in &ids {
            if e.offset < (1usize << high) {
                idx.extend_from_slice(&u32::try_from(e.offset).unwrap_or(0).to_be_bytes());
            } else {
                let slot = u32::try_from(large.len()).unwrap_or(0) | (1u32 << high);
                idx.extend_from_slice(&slot.to_be_bytes());
                large.push(u64::try_from(e.offset).unwrap_or(0));
            }
        }
        for o in large {
            idx.extend_from_slice(&o.to_be_bytes());
        }
        idx.extend_from_slice(trailer);
        let mut s = Sha1::new();
        s.update(&idx);
        idx.extend_from_slice(&s.finalize());
        let pack_dir = git_dir.join("objects/pack");
        std::fs::write(pack_dir.join(format!("pack-{name}.pack")), &self.pack).map_err(io)?;
        std::fs::write(pack_dir.join(format!("pack-{name}.idx")), &idx).map_err(io)?;
        let mut packed = String::from("# pack-refs with: peeled fully-peeled sorted \n");
        for (rname, id) in &self.refs {
            packed.push_str(&format!("{id} {rname}\n"));
            let mut target = id.clone();
            while entries.get(&target).map(|e| e.kind) == Some(GitObjectKind::Tag) {
                let (k, t) = self.object_from(&entries, &target)?;
                target = header_links(k, &t).into_iter().next().unwrap_or_default();
            }
            if &target != id {
                packed.push_str(&format!("^{target}\n"));
            }
        }
        std::fs::write(git_dir.join("packed-refs"), packed).map_err(io)?;
        Ok(entries.len())
    }

    /// The capsule: `VDNT_GIT_HIST_V1`, an 8-byte big-endian metadata length, the metadata (refs
    /// and HEAD) as JSON, then the pack.
    pub fn encode_capsule(&self) -> Result<Vec<u8>, StitchCustodyError> {
        let meta = serde_json::to_vec(&serde_json::json!({ "head": self.head, "refs": self.refs }))
            .map_err(|e| refuse(format!("serialize history metadata: {e}")))?;
        let mut out = Vec::with_capacity(CAPSULE_MAGIC.len() + EIGHT + meta.len() + self.pack.len());
        out.extend_from_slice(CAPSULE_MAGIC);
        out.extend_from_slice(&u64::try_from(meta.len()).unwrap_or(0).to_be_bytes());
        out.extend_from_slice(&meta);
        out.extend_from_slice(&self.pack);
        Ok(out)
    }

    /// Restores a history from its capsule and verifies it.
    pub fn decode_capsule(bytes: &[u8]) -> Result<(Self, GitClosureReport), StitchCustodyError> {
        let m = CAPSULE_MAGIC.len();
        if bytes.get(..m) != Some(CAPSULE_MAGIC.as_slice()) || bytes.len() < m + EIGHT {
            return Err(refuse("not a Verdant Git history capsule".to_string()));
        }
        let mut lb = [0u8; EIGHT];
        lb.copy_from_slice(&bytes[m..m + EIGHT]);
        let len = usize::try_from(u64::from_be_bytes(lb)).map_err(|_| truncated())?;
        let meta_end = m + EIGHT + len;
        let meta: serde_json::Value = serde_json::from_slice(bytes.get(m + EIGHT..meta_end).ok_or_else(truncated)?)
            .map_err(|e| refuse(format!("parse history metadata: {e}")))?;
        let refs: BTreeMap<String, String> = serde_json::from_value(meta.get("refs").cloned().unwrap_or_default())
            .map_err(|e| refuse(format!("parse history refs: {e}")))?;
        let head = meta.get("head").and_then(|h| h.as_str()).unwrap_or_default().to_string();
        let h = Self { pack: bytes[meta_end..].to_vec(), refs, head };
        let report = h.verify()?;
        Ok((h, report))
    }
}

const CAPSULE_MAGIC: &[u8; 1 << FOUR] = b"VDNT_GIT_HIST_V1";

/// The address of a Verdant-hosted Git history: the SHA-256 of its capsule, the Reed-Solomon
/// layout of its Stitch shards, and the refs it holds. Carriers are untrusted transport
/// (`principle6ZeroTrustVerifiedHydration`): a reader takes any `k` shards from any relay and
/// accepts the history only if it re-hashes to `capsule_kappa`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHostingPin {
    /// Format identifier (`verdant/git-history-pin/1`).
    pub spec: String,
    /// Repository name the history was imported as (`owner/name`).
    pub repository: String,
    /// SHA-256 hex of the history capsule.
    pub capsule_kappa: String,
    /// Exact capsule byte length.
    pub capsule_byte_len: usize,
    /// Data shards needed to recover.
    pub data_shards_k: u8,
    /// Shards produced.
    pub total_shards_n: u8,
    /// Shard file names, `shard-<index>.stch`.
    pub shard_files: Vec<String>,
    /// The ref `HEAD` names.
    pub head: String,
    /// Every ref and the object id it names.
    pub refs: BTreeMap<String, String>,
    /// Closure counts verified before hosting.
    pub closure: GitClosureReport,
    /// Relay base URLs that carry the shards (any of them, in any order).
    pub stitch_relays: Vec<String>,
}

impl GitHostingPin {
    /// Pretty JSON form of the pin.
    pub fn to_json(&self) -> Result<String, StitchCustodyError> {
        serde_json::to_string_pretty(self).map_err(|e| refuse(format!("serialize pin: {e}")))
    }

    /// Parses a pin.
    pub fn from_json(raw: &str) -> Result<Self, StitchCustodyError> {
        serde_json::from_str(raw).map_err(|e| refuse(format!("parse pin: {e}")))
    }
}

impl VerdantGitHistory {
    /// Encodes, shards (`k`-of-`n` Reed-Solomon) and pins this history for Stitch hosting.
    pub fn host(
        &self,
        repository: &str,
        data_shards_k: u8,
        total_shards_n: u8,
        stitch_relays: Vec<String>,
    ) -> Result<(Vec<crate::network::StitchRsShard>, GitHostingPin), StitchCustodyError> {
        let closure = self.verify()?;
        let capsule = self.encode_capsule()?;
        let shards = crate::network::ReedSolomonStitchCodec::shard_k_of_n(&capsule, data_shards_k, total_shards_n)?;
        let pin = GitHostingPin {
            spec: "verdant/git-history-pin/1".to_string(),
            repository: repository.to_string(),
            capsule_kappa: sha256_hex(&capsule),
            capsule_byte_len: capsule.len(),
            data_shards_k,
            total_shards_n,
            shard_files: shards.iter().map(|s| format!("shard-{}.stch", s.index)).collect(),
            head: self.head.clone(),
            refs: self.refs.clone(),
            closure,
            stitch_relays,
        };
        Ok((shards, pin))
    }

    /// Recovers a hosted history from any `k` of its shards and refuses it unless the capsule
    /// re-hashes to the pin, every object re-hashes, and refs and closure equal the pin's.
    pub fn recover(shards: &[crate::network::StitchRsShard], pin: &GitHostingPin) -> Result<Self, StitchCustodyError> {
        let capsule = crate::network::ReedSolomonStitchCodec::recover_k_of_n(shards)?;
        let got = sha256_hex(&capsule);
        if got != pin.capsule_kappa {
            return Err(refuse(format!("recovered capsule {got} is not the pinned {}", pin.capsule_kappa)));
        }
        let (h, closure) = Self::decode_capsule(&capsule)?;
        if h.refs != pin.refs || h.head != pin.head || closure != pin.closure {
            return Err(refuse("recovered refs or closure differ from the pin".to_string()));
        }
        Ok(h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn empty_blob_and_tree_ids_match_git() {
        assert_eq!(git_object_id(GitObjectKind::Blob, b""), "e69de29bb2d1d6434b8b29ae775ad8c2e48c5391");
        assert_eq!(git_object_id(GitObjectKind::Tree, b""), "4b825dc642cb6eb9a060e54bf8d69288fbee4904");
    }

    fn git(dir: &std::path::Path, args: &[&str]) -> Vec<u8> {
        let o = Command::new("git").current_dir(dir).args(args).output().expect("git");
        assert!(o.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&o.stderr));
        o.stdout
    }

    /// A repository with a merge, a tag and deltas, packed by Git itself.
    fn sample() -> (tempdir::Dir, VerdantGitHistory) {
        let d = tempdir::Dir::new();
        let p = d.path();
        git(p, &["init", "-q", "-b", "main"]);
        git(p, &["config", "user.email", "t@example.org"]);
        git(p, &["config", "user.name", "T"]);
        let big: String = (0..400).map(|i| format!("line {i}\n")).collect();
        std::fs::write(p.join("a.txt"), &big).unwrap();
        git(p, &["add", "."]);
        git(p, &["commit", "-qm", "one"]);
        git(p, &["checkout", "-qb", "side"]);
        std::fs::write(p.join("a.txt"), format!("{big}side\n")).unwrap();
        git(p, &["commit", "-qam", "side"]);
        git(p, &["checkout", "-q", "main"]);
        std::fs::write(p.join("b.txt"), "b\n").unwrap();
        git(p, &["add", "."]);
        git(p, &["commit", "-qm", "two"]);
        git(p, &["merge", "-q", "--no-edit", "side"]);
        git(p, &["tag", "-am", "v1", "v1"]);
        let refs = String::from_utf8(git(p, &["for-each-ref", "--format=%(objectname) %(refname)", "refs/heads", "refs/tags"])).unwrap();
        let list = git(p, &["rev-list", "--objects", "--all"]);
        let ids: String = String::from_utf8(list).unwrap().lines().map(|l| format!("{}\n", l.split(' ').next().unwrap())).collect();
        let mut child = Command::new("git")
            .current_dir(p)
            .args(["pack-objects", "--stdout", "--delta-base-offset", "-q"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        use std::io::Write;
        child.stdin.take().unwrap().write_all(ids.as_bytes()).unwrap();
        let pack = child.wait_with_output().unwrap().stdout;
        let (h, _) = VerdantGitHistory::import(pack, &refs, "refs/heads/main").unwrap();
        (d, h)
    }

    mod tempdir {
        pub struct Dir(std::path::PathBuf);
        impl Dir {
            pub fn new() -> Self {
                use std::sync::atomic::{AtomicUsize, Ordering};
                static N: AtomicUsize = AtomicUsize::new(0);
                let p = std::env::temp_dir().join(format!("vgit-{}-{}", std::process::id(), N.fetch_add(1, Ordering::SeqCst)));
                std::fs::create_dir_all(&p).unwrap();
                Self(p)
            }
            pub fn path(&self) -> &std::path::Path {
                &self.0
            }
        }
        impl Drop for Dir {
            fn drop(&mut self) {
                let _ = std::fs::remove_dir_all(&self.0);
            }
        }
    }

    #[test]
    fn merges_keep_every_parent() {
        let (_d, h) = sample();
        let r = h.verify().unwrap();
        assert_eq!((r.commits, r.merge_commits, r.refs), (4, 1, 3));
        assert_eq!(h.commit_parents(&h.refs["refs/heads/main"]).unwrap().len(), 2);
    }

    #[test]
    fn export_is_a_valid_repository_with_the_same_ids() {
        let (src, h) = sample();
        let out = tempdir::Dir::new();
        h.write_bare_repository(out.path()).unwrap();
        git(out.path(), &["fsck", "--full", "--strict"]);
        let a = git(src.path(), &["for-each-ref", "--format=%(objectname) %(refname)", "refs/heads", "refs/tags"]);
        let b = git(out.path(), &["for-each-ref", "--format=%(objectname) %(refname)"]);
        assert_eq!(a, b);
        assert_eq!(git(src.path(), &["rev-list", "--all"]), git(out.path(), &["rev-list", "--all"]));
    }

    #[test]
    fn sparse_extract_reads_one_directory() {
        let (_d, h) = sample();
        let out = tempdir::Dir::new();
        let n = h.extract(&h.refs["refs/heads/main"], "b.txt", out.path()).unwrap();
        assert_eq!(n, 1);
        assert_eq!(std::fs::read(out.path().join("b.txt")).unwrap(), b"b\n");
    }

    #[test]
    fn tampered_pack_and_missing_ref_are_refused() {
        let (_d, h) = sample();
        let mut bad = h.clone();
        let mid = bad.pack.len() / 2;
        bad.pack[mid] ^= 1;
        assert!(bad.verify().is_err());
        let mut gone = h.clone();
        gone.refs.insert("refs/heads/ghost".into(), "1".repeat(40));
        assert!(gone.verify().is_err());
    }

    #[test]
    fn hosted_history_recovers_from_any_four_of_eight() {
        let (_d, h) = sample();
        let (shards, pin) = h.host("t/t", 4, 8, vec![]).unwrap();
        let back = VerdantGitHistory::recover(&shards[4..], &pin).unwrap();
        assert_eq!(back, h);
        let mut wrong = pin.clone();
        wrong.capsule_kappa = "0".repeat(64);
        assert!(VerdantGitHistory::recover(&shards[..4], &wrong).is_err());
    }
}
