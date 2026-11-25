#!/bin/sh
set -eu

SCRIPT_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
SRC="$SCRIPT_DIR/verdant_neural.om"
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

TMP_SRC="$OUT_DIR/verdant_neural.build.om"
trap 'rm -f "$TMP_SRC"' EXIT

case "$OS/$ARCH" in
  Darwin/arm64)
    OUT_BIN="$OUT_DIR/verdant-neural-macos"
    { printf 'target a64macos\n'; cat "$SRC"; } > "$TMP_SRC"
    ;;
  Linux/aarch64)
    OUT_BIN="$OUT_DIR/verdant-neural-linux-aarch64"
    cp "$SRC" "$TMP_SRC"
    ;;
  Linux/x86_64)
    OUT_BIN="$OUT_DIR/verdant-neural-linux-x86_64"
    { printf 'target x86linux\n'; cat "$SRC"; } > "$TMP_SRC"
    ;;
  *)
    echo "error: unsupported platform $OS/$ARCH" >&2
    exit 1
    ;;
esac

TMP_BIN="$OUT_BIN.tmp"
rm -f "$TMP_BIN"
"$OMC_BIN" < "$TMP_SRC" > "$TMP_BIN"
chmod +x "$TMP_BIN"

if [ "$OS/$ARCH" = "Darwin/arm64" ]; then
  codesign -s - -f "$TMP_BIN" >/dev/null 2>&1
fi

mv -f "$TMP_BIN" "$OUT_BIN"
ln -sf "$(basename "$OUT_BIN")" "$OUT_DIR/verdant-neural"

if command -v shasum >/dev/null 2>&1; then
  OMC_SHA="$(shasum -a 256 "$OMC_BIN" | awk '{print $1}')"
  BIN_SHA="$(shasum -a 256 "$OUT_BIN" | awk '{print $1}')"
else
  OMC_SHA="$(sha256sum "$OMC_BIN" | awk '{print $1}')"
  BIN_SHA="$(sha256sum "$OUT_BIN" | awk '{print $1}')"
fi

echo "BUILT $OUT_BIN"
echo "  omc_sha256=$OMC_SHA"
echo "  bin_sha256=$BIN_SHA"
