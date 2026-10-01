# Solutions

Finished code for when you are stuck. Each one runs directly:

```
cargo run --bin balls
cargo run --bin snake
```

| Binary | What it is |
|---|---|
| `balls` | The starter with every `Try:` applied: colours, gravity, friction, collisions, trails, mouse attraction, clear key |
| `snake` | A complete Snake game: grid movement, arrow keys, growing body, game over and restart |

To continue from a solution instead of the starter, copy it over your file:

```
cp solutions/balls.rs src/main.rs
```

## Warm-up wall answers

1. `for ball in self.balls` tries to move the vector out of `self`, but
   `self` is only borrowed (`&mut self`), so there is nothing to move it into.
   Borrow it instead: `&mut self.balls`.
2. `draw` takes `&self`, a shared reference. Shared references cannot change
   anything. Changes belong in `update`, which takes `&mut self`.
3. `push(ball)` moves the ball into the vector. The variable `ball` is empty
   afterwards, so the second push has nothing to push. Create a second ball,
   or make `Ball` cloneable with `#[derive(Clone, Copy)]` and push a copy.
