#!/bin/sh
# Replays a session. Check your work with:   sh demo.sh | diff - expected.txt
# Replay the finished version with:          sh demo.sh expenses-solution
BIN=${1:-expenses}
run() { cargo run -q --bin "$BIN" -- "$@"; }

rm -f expenses.txt
run add coffee 4 food
run add rent 800 rent
run add "bus ticket" 3 travel
run add lunch 12 food
run list
run biggest
run spent food
run remove coffee
run remove coffee
run spent food
run list
