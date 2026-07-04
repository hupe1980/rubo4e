#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <schema-version>" >&2
    exit 1
fi

version="$1"
archive_tag="${version}"
series="${archive_tag#v}"

TMPZIP=$(mktemp /tmp/bo4e_schemas_XXXXXX.zip)
TMPDIR=$(mktemp -d /tmp/bo4e_schemas_XXXXXX)
cleanup() {
    rm -rf "${TMPZIP}" "${TMPDIR}"
}
trap cleanup EXIT

curl -fL "https://github.com/bo4e/BO4E-Schemas/archive/refs/tags/${archive_tag}.zip" -o "${TMPZIP}"
unzip -q "${TMPZIP}" -d "${TMPDIR}"

DEST="generator/schemas/${archive_tag}"
mkdir -p "${DEST}"
cp -r "${TMPDIR}/BO4E-Schemas-${series}/src/bo4e_schemas/." "${DEST}/"

echo "Downloaded ${archive_tag}: $(find "${DEST}" -name '*.json' | wc -l | tr -d ' ') JSON files → ${DEST}/"
