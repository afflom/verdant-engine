#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
OUT_DIR="${OUT_DIR:-$SCRIPT_DIR/bin}"
mkdir -p "$OUT_DIR"

OS="$(uname -s)"
ARCH="$(uname -m)"

if [ -n "${OMC:-}" ]; then
  OMC_BIN="$OMC"
elif [ -x "$HOME/.local/bin/omc" ]; then
  OMC_BIN="$HOME/.local/bin/omc"
elif [ -x "$HOME/canon/omega-mv/seed/out/omc" ]; then
  OMC_BIN="$HOME/canon/omega-mv/seed/out/omc"
else
  echo "error: omc not found (set OMC=/path/to/omc)" >&2
  exit 1
fi

build_one() {
  src="$1"
  base="$2"
  tmp_src="$OUT_DIR/$base.build.om"

  case "$OS/$ARCH" in
    Darwin/arm64)
      out_bin="$OUT_DIR/$base-macos"
      { printf 'target a64macos\n'; cat "$src"; } > "$tmp_src"
      ;;
    Linux/aarch64)
      out_bin="$OUT_DIR/$base-linux-aarch64"
      cp "$src" "$tmp_src"
      ;;
    Linux/x86_64)
      out_bin="$OUT_DIR/$base-linux-x86_64"
      { printf 'target x86linux\n'; cat "$src"; } > "$tmp_src"
      ;;
    *)
      echo "error: unsupported platform $OS/$ARCH" >&2
      rm -f "$tmp_src"
      exit 1
      ;;
  esac

  tmp_bin="$out_bin.tmp"
  rm -f "$tmp_bin"
  "$OMC_BIN" < "$tmp_src" > "$tmp_bin"
  rm -f "$tmp_src"
  chmod +x "$tmp_bin"

  if [ "$OS/$ARCH" = "Darwin/arm64" ]; then
    codesign -s - -f "$tmp_bin" >/dev/null 2>&1
  fi

  mv -f "$tmp_bin" "$out_bin"
  ln -sf "$(basename "$out_bin")" "$OUT_DIR/$base"

  if command -v shasum >/dev/null 2>&1; then
    omc_sha="$(shasum -a 256 "$OMC_BIN" | awk '{print $1}')"
    bin_sha="$(shasum -a 256 "$out_bin" | awk '{print $1}')"
  else
    omc_sha="$(sha256sum "$OMC_BIN" | awk '{print $1}')"
    bin_sha="$(sha256sum "$out_bin" | awk '{print $1}')"
  fi

  echo "BUILT $out_bin"
  echo "  omc_sha256=$omc_sha"
  echo "  bin_sha256=$bin_sha"
}

build_one "$SCRIPT_DIR/verdant_neural.om" "verdant-neural"
build_one "$SCRIPT_DIR/verdant_slm_jit.om" "verdant-slm-jit"
