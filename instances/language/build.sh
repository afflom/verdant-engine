#!/usr/bin/env bash
# Run from a checkout of verdant-engine. Docker supplies the repository's locked SDK.
set -euo pipefail
cd "$(dirname "$0")"
here=$PWD
repo=$(git rev-parse --show-toplevel)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT
checkout() {
  git init -q "$3"
  git -C "$3" fetch -q --depth=1 "https://github.com/$1" "$2"
  git -C "$3" checkout -q --detach FETCH_HEAD
  test "$(git -C "$3" rev-parse HEAD)" = "$2"
}
checkout afflom/prism-devrel ace153e37c7c29646f3efa6e273c3c4f8c9fb8bc "$work/generator"
checkout UOR-Foundation/template 0851bb41138041f0f75118135fd8a342109c2d73 "$work/project"
node "$work/generator/builder/cli.mjs" check prompt.json
node "$work/generator/builder/cli.mjs" build prompt.json --out "$work/project" --last-modified 2026-11-18T02:00:00Z
sdk=$(python3 -c 'import json,sys;print(json.load(open(sys.argv[1]))["sdk_image"])' "$repo/prismpm.lock")
[[ $sdk =~ @sha256:[0-9a-f]{64}$ ]]
mkdir -p "$work/scratch/home/.docker" vendor
docker run --rm --user "$(id -u):$(id -g)" --group-add "$(stat -c '%g' /var/run/docker.sock)" \
  -v "$work/project:$work/project" -v "$work/scratch:/tmp" -v /var/run/docker.sock:/var/run/docker.sock \
  -w "$work/project/app" -e HOME=/tmp/home -e PRISMPM_EPHEMERAL_HOME=1 "$sdk" \
  sh -ec 'lexlean lock && lexlean fmt --all && lexlean lock && prismpm verify'
shopt -s nullglob
holos=("$work"/project/app/.prism/build/*/*.holo)
test "${#holos[@]}" = 1
cp "${holos[0]}" "$here/vendor/verdant-language.holo"
cp -r "$work/project/try/vendor/prism-code" "$work/prism-code"
rm -rf "$here/vendor/prism-code"
mv "$work/prism-code" "$here/vendor/prism-code"
