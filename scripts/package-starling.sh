#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${1:-${ROOT}/utilitymod-1.1.0.hnmod}"

command -v zip >/dev/null 2>&1 || {
  echo "Error: zip is required to package the .hnmod archive." >&2
  exit 1
}

test -f "${ROOT}/horizon-native.json"
test -f "${ROOT}/mixins/utilitymod/horizon-mixin.toml"

mkdir -p "$(dirname "${OUT}")"
rm -f "${OUT}"
# Starling/Horizon native bundles are ZIPs containing the package manifest
# and mixins/<mod-id>/ source tree (not a content-mod .hmod layout).
(
  cd "${ROOT}"
  zip -q -r "${OUT}" horizon-native.json mixins/utilitymod
)
echo "Created ${OUT}"
