#!/usr/bin/env bash
# Dev helper: run horizon-mixins plan against a pristine, stamped Horizon checkout.
# usage: tools/plan.sh <horizon checkout> [mods dir]
set -euo pipefail
UP="$1"; MODS="${2:-mixins}"
RUST="/g/Games/HorizonBuild/rust/1.99.1-x86_64-pc-windows-gnu"
export PATH="$RUST/bin:$RUST/mingw64/bin:$PATH"
TOOL=$(ls -d /g/Games/HorizonBuild/native/tools/*/bin/horizon-mixins.exe | tail -2 | head -1)
"$TOOL" plan --workspace "$UP" --mods "$MODS" --out "${PLAN_OUT:-plan.json}"
