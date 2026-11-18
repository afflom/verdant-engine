"""Materialize the verified upstream language route for a local Verdant instance."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import gzip
import hashlib
import json
import subprocess
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parent
ME = "fe7ec040132bcad07dcfafb0fa4dc2b07cd86c78"
ENGINE = "c21176cf24c51d9a0de07f649cefe2ee4b522093"
MODEL = "smollm2-135m-instruct"
VENDOR = ROOT / "vendor"
FILES = ["site/models/" + f for f in ("models.mjs", "models.lock.json", "model-worker.mjs")]
FILES += ["site/r4/" + f + ".mjs" for f in ("fingerprint", "grant", "pieces", "store", "decide")]
FILES += ["site/senses/guest.mjs", "site/senses/guest/senses.wasm"]
FILES += ["app/specs/models/spec.json", "LICENSE-APACHE", "LICENSE-MIT"]


def checkout(repo, revision, paths, target):
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.run(["git", "init", "-q", tmp], check=True)
        git = ["git", "-C", tmp]
        subprocess.run(git + ["fetch", "-q", "--depth=1", "https://github.com/" + repo, revision], check=True)
        for path in paths:
            out = target / path
            out.parent.mkdir(parents=True, exist_ok=True)
            out.write_bytes(subprocess.check_output(git + ["show", revision + ":" + path]))


def materialize(piece):
    digest = piece["fingerprint"].removeprefix("sha256-")
    path = VENDOR / "site/models/objects" / (digest + ".gz")
    def valid(data):
        raw = gzip.decompress(data)
        return len(raw) == piece["bytes"] and hashlib.sha256(raw).hexdigest() == digest
    if path.exists() and valid(path.read_bytes()):
        return
    url = "https://afflom.github.io/me/models/objects/" + path.name
    for attempt in range(3):
        try:
            with urllib.request.urlopen(url, timeout=120) as response:
                data = response.read()
            if not valid(data):
                raise ValueError("Object failed integrity check: " + digest)
            with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as tmp:
                tmp.write(data)
            Path(tmp.name).replace(path)
            return
        except OSError:
            if attempt == 2:
                raise


if __name__ == "__main__":
    checkout("afflom/me", ME, FILES, VENDOR)
    checkout("afflom/verdant-engine", ENGINE, ["site/verdant-uorc.wasm"], VENDOR / "engine")
    lock = json.loads((VENDOR / "site/models/models.lock.json").read_text())
    model = next(m for m in lock["models"] if m["id"] == MODEL)
    files = model["files"] + [f for r in lock["runtimes"] if r["id"] in model["runtimes"] for f in r["files"]]
    pieces = {p["fingerprint"]: p for f in files for p in f["pieces"]}
    (VENDOR / "site/models/objects").mkdir(exist_ok=True)
    with ThreadPoolExecutor(max_workers=4) as pool:
        list(pool.map(materialize, pieces.values()))
    print(f"Verified {len(pieces)} objects for {model['source']}", flush=True)
