#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "$0")/.." && pwd)
TABLE="$ROOT/operation-table.lisp"

PIN=$(sed -n 's/.*(commit \. "\([0-9a-f]\{40\}\)").*/\1/p' "$TABLE" | head -1)
[ -n "$PIN" ] || { echo "operation authority pin missing" >&2; exit 1; }

TMP=$(mktemp -d)
trap 'rm -rf "$TMP"' EXIT

git -C "$TMP" init -q
git -C "$TMP" remote add origin https://github.com/juv4uk/my-lisp.git
git -C "$TMP" fetch -q --depth 1 origin "$PIN"
git -C "$TMP" checkout -q FETCH_HEAD

REG="$TMP/lib/surface/semantic-registry.lisp"
[ -f "$REG" ] || { echo "semantic registry missing at pinned authority" >&2; exit 1; }

for row in   "0002 atom"   "0003 eq"   "0004 cons"   "0005 car"   "0006 cdr"
do
  id=${row%% *}
  name=${row#* }
  grep -Eq "^  \($id \(en $name stable\)" "$REG" || {
    echo "semantic authority drift: $id no longer identifies stable $name" >&2
    exit 1
  }
  grep -Fq "(canonical-id . \"$id\")" "$TABLE" || {
    echo "operation table lost canonical id $id" >&2
    exit 1
  }
done

echo "OPERATION-AUTHORITY-GREEN my-lisp=$PIN core-five=0002..0006"
