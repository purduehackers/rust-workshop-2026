# Exercises

Work for after the workshop. Each folder has its own README and its own
solutions. Start with the README. The three programs each have a finished
version you can run right away, so you can see the goal before you start.

| Project | What you build | Library | Shape |
|---|---|---|---|
| [`challenge-problems/`](challenge-problems/) | 52 short exercises in the style of Rust By Practice, one idea each | none | Guided: make each file compile and print `Success!` with `cargo run --bin <n>` |
| [`habits/`](habits/) | A habit tracker with a grid of the last 21 days | ratatui | Guided: fill in seven numbered parts |
| [`expenses/`](expenses/) | A command line expense tracker with subcommands | clap | Guided: fill in six numbered parts |
| [`terminal-playground/`](terminal-playground/) | Animations, games and simulations in the terminal | crossterm, behind a small engine | Open-ended: a starter plus ideas |

Suggested order: `challenge-problems` to review the workshop one idea at a
time, then `habits` or `expenses` for a guided program, then
`terminal-playground` when you want to build something of your own.

## Running the solutions

Every exercise ships with finished code you can run without copying anything
over your own work. All commands are run from inside the exercise's folder.

### `challenge-problems/`

Solutions live under `solutions/`, one file per exercise in the same layout
as the exercises, named `<chapter>-<exercise>`:

```
cd solutions
cargo run --bin 03-ownership-2
```

To run every solution at once, and confirm every exercise still fails as
intended:

```
./check.sh
```

### `habits/`

```
cargo run --bin habits-solution
```

Loads `sample-habits.txt` on the first run. Press `q` to quit.

### `expenses/`

```
cargo run --bin expenses-solution -- --help
cargo run --bin expenses-solution -- add coffee 4 food
cargo run --bin expenses-solution -- list
```

Or replay a whole session and compare it against the expected output. No
diff output means it matches:

```
sh demo.sh expenses-solution | diff - expected.txt
```

### `terminal-playground/`

Two finished programs on the same engine. Press Esc to quit either.

```
cargo run --bin balls    # the starter with every hint applied
cargo run --bin snake    # a complete game
```

### Everything in one go

From this folder, this runs or checks every solution that does not need a
keyboard, which is useful before the day to make sure all of it still builds:

```
(cd challenge-problems && ./check.sh)
(cd expenses && sh demo.sh expenses-solution | diff - expected.txt)
(cd habits && cargo build --bin habits-solution)
(cd terminal-playground && cargo build --bin balls --bin snake)
```
