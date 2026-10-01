# Solution

The finished tracker, runnable on its own:

```
cargo run --bin expenses-solution -- add coffee 4 food
cargo run --bin expenses-solution -- list
cargo run --bin expenses-solution -- spent food
sh demo.sh expenses-solution | diff - expected.txt    # prints nothing
```

Both programs read and write the same `expenses.txt`, so delete it when
switching between them.

## Notes on the harder parts

- **Part 4, `biggest`**: the method returns `&Expense`, a reference into the
  list. Start with a reference to the first expense and replace it whenever
  a later one is larger. No copying, no cloning.
- **Part 5, `remove`**: you cannot remove from the list while a `for` loop is
  borrowing it. Find the index first, then remove after the loop. The index
  is an `Option<usize>`: an enum that is `Some(index)` or `None`, which
  `match` handles like any other enum.
- **Part 6, `spent`**: three changes in three places. The new `Cmd` variant
  gets its category parsed by Clap for free. `{category:?}` prints the
  variant name because `Category` derives `Debug`.
