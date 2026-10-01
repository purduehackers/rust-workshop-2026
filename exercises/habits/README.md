# Habit Tracker

A habit tracker that runs in your terminal, built on
[ratatui](https://ratatui.rs). Each habit is a row of the last 21 days. You
mark days done, watch streaks grow, and the best habit is named at the top.

## Before you start

Build it once so the dependencies are downloaded:

```
cargo run
```

The program stops right away with a message like
`not yet implemented: part 1` and a line number. That is expected: it points
at the first part for you to write.

Keys once it runs: `n` new habit, arrows move, space marks a day done, `q` quits.
The first run starts from `sample-habits.txt`. Your changes are saved to
`habits.txt` when you quit.

## Where to write code

```
src/main.rs        <- yours. `cargo run` runs this.
solutions/main.rs  <- the finished version. `cargo run --bin habits-solution`.
sample-habits.txt  <- three habits to start from
```

Everything you write goes above the "Given" banner in `src/main.rs`. Below
it is the drawing, key handling, loading and saving. Read it to see how your
methods get called, but nothing there needs changing except one match arm in
part 7.

## What to write

Each part replaces one `todo!()`. Run the program after every part: it
stops at the next part that is missing, with the line number.

1. `Draw for Habit`: one row. The padded name, every day via `draw_all`, then
   the done count and streak.
2. `Draw for Day`: one cell. Green for done, red for skipped, gray for empty.
3. `Habit::done_count`: how many days are done.
4. `Habit::streak`: done days in a row, counting back from today.
5. `App::best`: the habit with the most done days.
6. `App::add`: a new habit with 21 empty days. Press `n` to try it.
7. An `x` key that marks a day skipped. Write a `skip` method on `Habit` next
   to the given `toggle_done`, and a match arm in `handle_key` next to the
   space arm.

Notice which methods take `&self` and which take `&mut self`. The ones that
only read take `&self`. The ones that change a habit take `&mut self`, and
the compiler will refuse a change inside a `&self` method.

When all seven are in, check:

- every row shows green, red and gray cells
- the streak at the end of each row is right (count it)
- the title names the habit with the most done days
- `x` on a cell turns it red, `x` again clears it

## Make it yours

- A `d` key that deletes the selected habit. `Vec::remove` takes an index.
- A total row under the grid: how many days were done across all habits.
- Show the streak in gold when it is 7 or more. `"text".yellow()` works like `.green()`.
- Longer history: change `DAYS` and see what else has to change.

## Stuck?

```
cargo run --bin habits-solution
```

runs the finished version so you can see where you are heading.
`solutions/README.md` explains the parts people get stuck on. Read it when
stuck, not before.

## If something goes wrong

- The terminal looks broken after a crash: type `reset` and press Enter.
- You want to start over with the sample data: delete `habits.txt`.
- `cargo run` says `could not find Cargo.toml`: you are not in this folder.
  `cd` into `exercises/habits` first.
