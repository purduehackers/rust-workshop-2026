#!/bin/sh
# Runs every solution, and confirms every exercise still fails.
# Usage: ./check.sh
cd "$(dirname "$0")" || exit
out=$(mktemp -d)
status=0

if ! RUSTFLAGS="-D warnings" cargo build -q --manifest-path solutions/Cargo.toml >"$out/log" 2>&1; then
    echo "FAIL solutions do not build"; cat "$out/log"; rm -rf "$out"; exit 1
fi
for sol in solutions/*/*.rs; do
    name=${sol#solutions/}
    bin=$(echo "$name" | sed 's|/|-|; s|\.rs$||')
    if cargo run -q --manifest-path solutions/Cargo.toml --bin "$bin" >"$out/run" 2>&1 \
        && grep -q '^Success!$' "$out/run"; then
        echo "PASS solution $name"
    else
        echo "FAIL solution $name"; cat "$out/run"; status=1
    fi
done

for ex in */src/bin/*.rs; do
    chapter=${ex%%/*}
    bin=$(basename "$ex" .rs)
    name="$chapter/$bin.rs"
    # These two work as written: one has nothing to fix, one asks for a rewrite.
    case "$name" in 04-borrowing/7.rs|11-iterators/3.rs) continue;; esac
    if cargo run -q --manifest-path "$chapter/Cargo.toml" --bin "$bin" >/dev/null 2>&1; then
        echo "FAIL exercise $name already passes"; status=1
    else
        echo "ok   exercise $name fails as intended"
    fi
done

rm -rf "$out"
exit $status
