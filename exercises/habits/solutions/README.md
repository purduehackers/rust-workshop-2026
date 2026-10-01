# Solution

`solutions/main.rs` is the starter with all seven parts written. Run it with:

```
cargo run --bin habits-solution
```

To continue from it instead of the starter, copy it over your file:

```
cp solutions/main.rs src/main.rs
```

## Where people get stuck

- **Part 1, building the row.** `draw_all(&self.days)` gives a `Vec` of spans.
  `extend` appends a whole `Vec`, `push` appends one item. Build the counts
  with `format!` and wrap the String in `Span::from`.
- **Part 2, the match.** Each arm returns a `Span`, so all three arms must
  produce the same type. `"■".green()` and `"·".dark_gray()` both do.
- **Part 3, comparing days.** `for day in &self.days` gives a `&Day`. Compare
  with `*day == Day::Done` to look through the reference, or match on `day`.
- **Part 4, the streak.** Walk backwards with `self.days.iter().rev()` and
  `break` at the first day that is not done. Breaking is what makes it a
  streak instead of a count.
- **Part 5, returning a reference.** `best` returns `&Habit`, a reference to
  a habit that lives in `self.habits`. Keep a `&Habit` variable and replace
  it when a habit with more done days comes along. No cloning needed.
- **Part 6, `&mut self`.** `push` changes the list, so `add` needs `&mut
  self`. The signature is given; if you had to write it, `&self` would fail
  with "cannot borrow as mutable".
- **Part 7, the arm.** Copy the space arm, change the key to `'x'`, and call
  your `skip` method. `skip` is `toggle_done` with the variants swapped.
