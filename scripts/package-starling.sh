#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${1:-${ROOT}/utilitymod-1.1.0.hnmod}"
STAGE="$(mktemp -d)"
trap 'rm -rf "${STAGE}"' EXIT

command -v zip >/dev/null 2>&1 || {
  echo "Error: zip is required to package the .hnmod archive." >&2
  exit 1
}

test -f "${ROOT}/horizon-native.json"
test -f "${ROOT}/mixins/utilitymod/horizon-mixin.toml"

# Starling native bundles require horizon-mixin.toml and helper .rs files
# at the bundle root, not nested under the repository's mixins/ directory.
cp "${ROOT}/horizon-native.json" "${STAGE}/"
cp -R "${ROOT}/mixins/utilitymod/." "${STAGE}/"

mkdir -p "$(dirname "${OUT}")"
rm -f "${OUT}"
(
  cd "${STAGE}"
  zip -q -r "${OUT}" .
)
echo "Created ${OUT}"
