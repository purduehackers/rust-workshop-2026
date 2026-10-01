# Expense Tracker

A command line expense tracker. You write the methods; Clap turns the
command line into Rust values for you.

## Before you start

Build once, so the dependencies are downloaded:

```
cargo run -- --help
```

Then try the commands. Expenses are saved in `expenses.txt` between runs.

```
cargo run -- add coffee 4 food
cargo run -- add rent 800 rent
cargo run -- list
cargo run -- biggest
cargo run -- remove coffee
```

Categories are `food`, `rent` or `travel`. Names with spaces go in quotes:
`cargo run -- add "bus ticket" 3 travel`.

## Where to write code

```
src/main.rs        <- yours. `cargo run` runs this.
solutions/main.rs  <- the finished version. `cargo run --bin expenses-solution`.
demo.sh            <- replays a session so you can check your output.
expected.txt       <- what the session should print.
```

Everything you write is a method with a `todo!()` body. The program compiles
from the start, and `add` already works, so `cargo run -- add coffee 4 food`
does something on your first run. A command that reaches a `todo!()` stops
with a message naming the file and line: that is the part to do next.

## What to write

Fill in the parts in order. Each one is a few lines. `add` is written for
you and shows the shape.

1. `describe` for `Expenses`: every line, then the total
2. `describe` for `Expense`: print one expense on one line
3. `total`: sum the amounts with a `for` loop
4. `biggest`: return a reference to the largest expense
5. `remove`: find by name, remove, report
6. A `spent` command: a new `Cmd` variant, a match arm, and a method

`cargo run -- list` walks you through parts 1 to 3: each run stops at the
next one. Before writing a method, ask which it needs: `&mut self` if it
changes the list, `&self` if it only reads it.

## Check your work

```
sh demo.sh | diff - expected.txt
```

No output means every line matches. Otherwise `<` lines are yours and `>`
lines are expected. `demo.sh` deletes `expenses.txt` first, so your own
entries are gone after running it.

## Make it yours

- A `--sort` flag on `list` that orders by amount. Clap reads flags from
  fields marked `#[arg(long)]`.
- A budget: `cargo run -- budget 500` saves a limit, and `add` warns when the
  total goes over it.
- A `report` command printing one line per category with its total, so the
  `match` on `Category` earns its keep.
- An `edit` command that changes the amount of an existing expense.

## Stuck?

The finished version runs on its own:

```
cargo run --bin expenses-solution -- list
sh demo.sh expenses-solution | diff - expected.txt
```

Read `solutions/main.rs` when stuck on a part, not before. The notes in
`solutions/README.md` explain the three parts people get stuck on most.

## If something goes wrong

- `error: unrecognized subcommand`: run `cargo run -- --help` to see the
  commands. Arguments after `cargo run` need the `--` first.
- `invalid value 'Food' for '<CATEGORY>'`: categories are lowercase on the
  command line.
- `bad line in expenses.txt`: the file was edited by hand into a shape the
  program does not understand. Delete it and start again.
- The diff shows differences you cannot see: trailing spaces. Compare the
  exact strings in the part's comment with your `println!`.
