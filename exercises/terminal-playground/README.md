# Terminal Playground

A tiny engine that turns your terminal into a canvas. You write two functions:
what changes every frame, and what to draw. The engine handles the rest.

## Before you start

Build it once, from inside this folder:

```
cargo run
```

The first run downloads and compiles the dependencies, which takes a minute.
If you see bouncing dots, you are ready. Press Esc to quit.

Windows users: use Windows Terminal, not the old `cmd` window.

## Where to write code

```
src/main.rs      <- yours. `cargo run` runs this.
src/engine/      <- the engine. You never need to open it.
solutions/       <- finished examples. `cargo run --bin balls`, `cargo run --bin snake`.
```

`main.rs` has a struct holding your state, and an `impl App` with two methods:

```rust
fn update(&mut self, frame: &Frame)   // change state: move things, react to keys
fn draw(&self, canvas: &mut Canvas)   // paint the current state
```

`update` gets `&mut self` because it changes things. `draw` gets `&self`
because it only looks. The compiler holds you to that.

## Warm-up: three walls

Break the starter on purpose. Each error message is the lesson. Fix each one
before moving on.

1. In `move_balls`, change `for ball in &mut self.balls` to `for ball in self.balls`.
   Read the first error (the others follow from it). Why can't you take the
   balls out of `self`?
2. In `draw`, add `ball.x += 1.0;` inside the loop. Read the error. Why is
   `draw` not allowed to change anything?
3. In `handle_input`, replace the space-bar block with:

   ```rust
   if frame.pressed(' ') {
       let ball = Ball::new(10.0, 10.0);
       self.balls.push(ball);
       self.balls.push(ball);
   }
   ```

   Read the error. Where did `ball` go after the first push?

## Make the starter yours

`main.rs` is split into small methods: move, bounce, collide, handle input,
draw. Each has `Try:` comments with one-line changes that look good right
away. Start with gravity, then colours, then write `collide_balls`, which is
left empty for you. Once the balls bounce off each other you have written a
real physics simulation.

## Now build something

Pick one, or invent your own. Each comes with a suggested first change so you
are never staring at a blank screen.

| Idea | Difficulty | First change |
|---|---|---|
| Bouncing logo | easy | Draw a word with `canvas.text` instead of a dot |
| Starfield | easy | Stars move left; wrap to the right edge when they leave |
| Matrix rain | easy | One falling column of random letters, then many |
| Fireworks | medium | A particle has a lifetime; remove dead ones with `retain` |
| Clock | medium | Draw big digits from `frame.time` with `canvas.rect` |
| Pong | medium | Two paddles on Up/Down and W/S, one ball |
| Snake | medium | A `Vec<(i32, i32)>` body; push the head, pop the tail |
| Conway's Life | medium | A grid of `bool`; count neighbours; next grid from old grid |
| Falling sand | medium | A grid of an enum; sand moves down if the cell below is empty |
| Typing test | medium | Show a sentence; compare typed keys with `frame.keys()` |
| Paint program | medium | Mouse paints into a grid; number keys change colour |
| Spinning donut | hard | Search for "donut.c" and port it |

## Stuck?

The `solutions/` folder has finished code that runs as is:

```
cargo run --bin balls    # the starter with every Try: applied, collisions included
cargo run --bin snake    # a complete game, to see how a grid-based idea fits the same two methods
```

Run them to see where you are heading. Read them when stuck, not before.

## Engine reference

Everything below is available in `main.rs` through `use engine::{...}`.

### Frame (read in `update`)

| | |
|---|---|
| `frame.width`, `frame.height` | screen size in columns and rows |
| `frame.tick` | frames since start |
| `frame.time` | seconds since start |
| `frame.dt` | seconds since the last frame |
| `frame.pressed(' ')`, `frame.pressed(Key::Up)` | was this key pressed this frame |
| `frame.keys()` | every key pressed this frame, in order |
| `frame.mouse.x`, `frame.mouse.y`, `frame.mouse.held` | mouse position and button state |
| `frame.quit()` | end the program after this frame |
| `engine::screen_size()` | (columns, rows), for use before `run` starts |

Keys: `Key::Char(c)`, `Up`, `Down`, `Left`, `Right`, `Enter`, `Backspace`, `Tab`, `Esc`.
Esc and Ctrl+C quit by default. To use Esc yourself, start the engine with
`engine::run_with(app, Config { quit_on_esc: false, ..Config::default() })`.

### Canvas (write in `draw`)

| | |
|---|---|
| `canvas.width()`, `canvas.height()` | screen size |
| `canvas.put(x, y, '●', Color::Cyan)` | one character |
| `canvas.put_bg(x, y, ' ', Color::White, Color::Blue)` | with a background colour |
| `canvas.text(x, y, "hello", Color::White)` | a string |
| `canvas.rect(x, y, w, h, '█', Color::Red)` | a filled rectangle |
| `canvas.fill('.', Color::DarkGrey)` | the whole screen |
| `canvas.background(Color::Black)` | background of the whole screen |

Drawing outside the screen is ignored, so no bounds checks needed. The canvas
is wiped before every `draw`.

Colours: `Color::Red`, `Green`, `Blue`, `Cyan`, `Magenta`, `Yellow`, `White`,
`Black`, `DarkGrey`, `Grey`, and any RGB with `Color::Rgb { r: 255, g: 100, b: 0 }`.

Use characters that take one column: letters, `●`, `█`, `▓`, `▒`, `░`, `·`,
box-drawing lines. Emoji take two columns and will smear.

### Randomness

```rust
rand::random_range(0..10)        // an integer from 0 to 9
rand::random_range(-1.0..1.0)    // a float
rand::random::<bool>()           // a coin flip
```

## If something goes wrong

- Your program panicked and the terminal looks fine: good, that is the engine
  cleaning up. Read the message and the line number.
- The terminal looks broken after a crash: type `reset` and press Enter.
- Everything flickers or is slow: lower the frame rate with
  `engine::run_with(app, Config { fps: 15, ..Config::default() })`.
