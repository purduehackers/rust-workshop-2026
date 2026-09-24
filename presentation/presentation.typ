#import "@preview/larrow:1.2.0": *
#import "@preview/touying:0.7.4": *
#import themes.metropolis: *
#import "@preview/cetz:0.5.2"
#import "@preview/cetz-venn:0.2.0"
#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#show: codly-init.with()

#codly(
  number-format: it => "",
  zebra-fill: none,
  display-name: false,
  stroke: 1pt + luma(200),
  highlight-radius: 0pt,
  inset: (y: 0.25em),
  fill: luma(240),
  radius: 0.3em,
  header-cell-args: (inset: 0pt),
  header-repeat: false,
  footer-cell-args: (inset: 0pt),
  footer-repeat: false,
)

#show raw.where(block: true): set text(size: 1.2em)
#show raw.where(block: true): it => {
  codly(header: box(height: 0.5em), footer: box(height: 0.5em))
  it
}

#let animated-code(self, code) = {
  let lines = code.text.split("\n")
  let keep = ()
  for line in lines {
    let parts = line.split(regex("\s*//>\s*"))
    let min-i = int(parts.at(1, default: 1))
    if self.subslide >= min-i {
      keep.push(parts.at(0))
    } else {
      keep.push("")
    }
  }
  raw(keep.join("\n"), block: code.block, lang: code.lang)
}

#let red(n) = (n, color.red.lighten(80%))
#let green(n) = (n, color.green.lighten(75%))
#let hl(g: (), r: ()) = codly(
  highlighted-lines: g.map(green) + r.map(red),
)

#let cetz-canvas = touying-reducer.with(
  reduce: cetz.canvas,
  cover: cetz.draw.hide.with(bounds: true),
)

#let b(x, c: true) = text(weight: if c { "bold" } else { "regular" })[#x]

#let label-arrow = label-arrow.with(
  tip: (symbol: ">", fill: black),
  stroke: 2pt,
)

#show: metropolis-theme.with(
  align: start,
  config-info(
    title: [Introduction to the Rust Programming Language],
    subtitle: [#datetime(year: 2026, month: 10, day: 1).display() — A Purdue Hackers workshop],
    author: [Arhan Chaudhary],
    institution: [https://github.com/ArhanChaudhary],
    contact: [arhan.ch\@gmail.com],
  ),
)

#text(size: 1.2em)[#title-slide(extra: place(horizon, dx: 27em, dy: -1em)[
  #figure(
    image(width: 14em, "ferris.png"),
    caption: text(size: 1.2em)[=== Ferris, the Rust mascot],
    supplement: none,
  )
])]

= What is Rust?

#slide(align: horizon)[
  Rust is a general-purpose programming language that empowers #al(<end-1>)everyone to build reliable and efficient software.
  #align(center)[— https://rust-lang.org/]

  #pause

  #place(right, dx: -10em, dy: 0em)[

    #al(<start-1>)How?
    #label-arrow(
      <start-1>,
      <end-1>,
      both-offset: (0pt, -40pt),
      from-offset: (1em, 0.6em),
      to-offset: (-2em, -0.5em),
    )
  ]
]

= How is Rust empowering? <touying:hidden>

#slide(repeat: 4, self => [
  #place(center, dy: -1.5em)[
    #cetz.canvas(length: 6em, {
      import cetz.draw: *

      let D = 3.5em
      let R = 6em

      cetz-venn.venn3(
        name: "v",
        a-distance: D,
        b-distance: D,
        c-distance: D,
        radius: R,
        a-layer: 0,
        stroke: none,
        fill: rgb(250, 250, 250, 0),
      )

      let lbl(
        pos,
        body,
        dx: 0em,
        dy: 0em,
        name: none,
        anchor: none,
      ) = content(
        pos,
        place(center, dx: dx, dy: dy, body),
        name: name,
        anchor: anchor,
      )

      lbl("v.abc")[*Rust*]
      lbl("v.abc", dx: -0.75em, dy: -1.5em)[Bend2]
      lbl("v.b", dx: -0.25em, dy: -2.5em)[Assembly]
      lbl("v.bc", dx: 1em, dy: 0.5em)[Zig]
      lbl("v.b", dx: 1.25em, dy: 3em)[C]
      lbl("v.b", dx: 0.5em, dy: 0em)[#box[C#sym.zwj+#sym.zwj+]]
      lbl("v.ac", dx: -1em, dy: 1em)[TypeScript]
      lbl("v.ac", dx: -1.2em, dy: -1em)[Swift]
      lbl("v.a", dx: -1.25em, dy: -0.5em)[Python]
      lbl("v.a", dx: -2em, dy: 2em)[Go]
      lbl("v.a", dx: 0.75em, dy: -3em)[JavaScript]
      lbl("v.c")[Fortran]

      let axes = (
        (
          key: "a",
          name: [Memory~Safe],
          anchor: "east",
          line-anchor: "south",
          content-anchor: "north",
          angle: 150deg,
          offset: (-110deg, 8em),
          note: box(width: 8em)[>80% of exploited vulnerabilities are memory safety issues. @memory-safety],
          rel: (0, 0.05),
        ),
        (
          key: "b",
          name: [Low~Level~Control],
          anchor: "west",
          line-anchor: "south",
          content-anchor: "north",
          angle: 30deg,
          offset: (-75deg, 7em),
          note: box(width: 8em)[Direct control over hardware. Minimal runtime.],
          rel: (0, 0.05),
        ),
        (
          key: "c",
          name: [Strong~Type~System],
          anchor: "north",
          line-anchor: "north-west",
          content-anchor: "south-east",
          angle: 270deg,
          offset: (158deg, 15em),
          note: box(width: 10em)[#set align(right); Enforce data requirements at compile-time],
          rel: (0.1, 0.1),
        ),
      )

      hide(bounds: true, {
        for ax in axes {
          let bk = "bounds-" + ax.key
          content(("v." + ax.key, -70%, "v.abc"), b([#ax.name]), anchor: ax.anchor, name: bk)
          content((rel: ax.offset, to: bk), ax.note)
        }
      })

      for (i, ax) in axes.enumerate() {
        let on = if self.subslide == 1 { true } else { self.subslide == i + 2 }
        let k = ax.key + "-on"
        let c = ax.key + "-content"

        content(("v." + ax.key, -70%, "v.abc"), b([#ax.name], c: on), name: k, anchor: ax.anchor)

        circle((ax.angle, D), radius: R, fill: none, stroke: if on { black } else { gray })

        if on and self.subslide != 1 {
          content((rel: ax.offset, to: k), ax.note, name: c)
          line(
            (rel: (-0.05, -0.05), to: (name: k, anchor: ax.line-anchor)),
            (rel: ax.rel, to: (name: c, anchor: ax.content-anchor)),
            stroke: 2pt,
            mark: (end: ">", fill: black),
          )
        }
      }
    })
  ]
])

= Who uses Rust?

#slide(align: horizon)[
  #set align(center)

  #grid(
    rows: 4,
    columns: 8,
    column-gutter: 1em,
    row-gutter: 1em,
    grid.cell(colspan: 8)[
      === You probably run Rust code every day
    ],
    image("linux.png"),
    image("nvidia.png"),
    image("apple.png"),
    image("google.png"),
    image("amazon.png"),
    image("mozilla.png"),
    image("meta.png"),
    image("microsoft.png"),
    grid.cell(colspan: 8)[
      #pause
      === As do AI agents
    ],
    [], [], [],
    image("claude.png"),
    image("openai.png"),
    [], [], [],
  )
]

#focus-slide[Let's learn some Rust!]

= Setting Up <touying:hidden>

#slide(align: center + horizon)[
  === https://code.purduehackers.com/new/purduehackers/rust-workshop-2026
]

= Hello World <touying:hidden>

```rust
fn main() {
    println!("Hello World!");
}
```

#place(dx: 0em, dy: 6em)[#image(width: 20%, "ferris-gesture.png")]
#place(dx: 6.5em, dy: 1em)[#image(width: 20%, "speech.png")]
#alternatives[
  #place(dx: 8em, dy: 3.25em)[
    Hello World!
  ]
][
  #place(dx: 8.75em, dy: 2.5em)[
    Let's run \ this live!
  ]
]

= Guessing Game <touying:hidden>

#text(size: 0.7em)[#grid(
  columns: 2,
  column-gutter: 1em,
  ```
  $ cargo run --bin guessing-game
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.10s
       Running `target/debug/guessing-game`
  Guess the number!
  Please input your guess.
  60
  You guessed: 60
  Too small!
  Please input your guess.
  80
  You guessed: 80
  Too big!
  Please input your guess.
  73
  You guessed: 73
  Too big!
  ```,
  ```
  Please input your guess.
  66
  You guessed: 66
  Too small!
  Please input your guess.
  70
  You guessed: 70
  Too big!
  Please input your guess.
  68
  You guessed: 68
  You win!
  ```,
)]

= Variables <touying:hidden>

```rs
fn mutability() {
    let power = 1000;
    println!("The power level is: {power}");
    power = 2000;
    println!("The power level is now: {power}");
}
```

#pause

#text(font: "DejaVu Sans Mono")[
  #text(rgb(205, 75, 62), weight: "bold")[error[E0384]]:
  cannot assign twice to immutable variable power
]

#align(center + bottom)[#image(height: 1fr, "ferris-confused.svg")]

---

#hl(g: (3,), r: (2,))
```rs
fn mutability() {
    let power = 1000;
    let mut power = 1000;
    println!("The power level is: {power}");
    power = 2000;
    println!("The power level is now: {power}");
}
```

#align(center + bottom)[#image(height: 1fr, "ferris-happy.png")]

---

#slide(repeat: 4, self => {
  if self.subslide == 4 {
    local(
      highlight-inset: 0pt,
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 5, start: 5, end: 5, fill: yellow), (line: 8, start: 5, end: 5, fill: yellow)),
      header: box(height: 0.5em),
      footer: box(height: 0.5em),
    )[
      ```rs
      fn scope() {
          let power = 5000;
          println!("The power level is: {power}");

          { // this creates a scope
              let power = power * 2; // 10000
              println!("The power level is: {power}. The power level is over 9000!!");
          }

          println!("The power level is back to: {power}");
      }
      ```
    ]
  } else {
    animated-code(self, ```rs
    fn scope() {
        let power = 5000;
        println!("The power level is: {power}");

        { //> 2
            let power = power * 2; // 10000 //> 2
            println!("The power level is: {power}. The power level is over 9000!!"); //> 2
        } //> 2

        println!("The power level is back to: {power}"); //> 3
    }
    ```)
  }
})

---

#slide(repeat: 5, self => {
  animated-code(self)[```rs
  fn data_types() {
      // Every variable has a singular, fixed type!
      // `int` has the integer type `i32`
      let mut int = 5;
      // `int2` is optionally annotated explicitly //> 2
      let int2: i32 = 5; //> 2
      let true_or_false: bool = true; //> 3
      let decimal: f64 = 4.5; //> 4

      int = decimal; // different types //> 5
  }```]
  only(5)[
    #text(font: "DejaVu Sans Mono")[
      #text(rgb(205, 75, 62), weight: "bold")[error[E0308]]:
      mismatched types
    ]
  ]
})

= Control Flow <touying:hidden>

#slide(repeat: 3, self => {
  animated-code(self)[```rs
  fn main() {
      // Reads a user input number from the terminal
      let int: i32 = utils::read_number();
      if int == 42 { //> 2
          println!("42 is the meaning of life"); //> 2
      } //> 2
      let abs = if int < 0 { //> 3
          -int //> 3
      } else { //> 3
          int //> 3
      }; //> 3
  }```]
})

---

#slide(repeat: 2, self => {
  animated-code(self)[```rs
  fn main() {
      // Reads a user input number from the terminal
      let int: i32 = utils::read_number();
      // If `x` is positive, let's //> 2
      // subtract one until `x` is zero. //> 2
      while x != 0 { //> 2
          println!("Count down: {x}"); //> 2
          x -= 1; //> 2
      } //> 2
  }```]
})


= Ownership <touying:hidden>

#pause

#align(horizon + center)[
  === Ownership can be confusing!
  Do not be afraid to ask me questions
]

---

=== How do programming languages manage their memory?

#pause
- Python, Java, Typescript: automatically with overhead
#pause
- C, C++: manually, by yourself
#pause

=== What does Rust do?

#pause

It picks a third option:

- Rust: the compiler enforces the rules of ownership! #pause
  - The rules manage memory automatically *without overhead* #pause
  - Memory unsafe $=>$ the rules are violated $=>$ your program won't compile #pause

---

#let w = 5em;
#let h_ = 7em;
#let fold = 1em;
#let icon(change) = align(center + horizon)[#box(width: w, height: h_, {
  place(polygon(
    fill: white,
    stroke: 0.5pt + black,
    (0pt, 0pt),
    (w - fold, 0pt),
    (w, fold),
    (w, h_),
    (0pt, h_),
  ))
  place(top + right, polygon(
    fill: luma(230),
    stroke: 0.5pt + black,
    (0pt, 0pt),
    (0pt, fold),
    (fold, fold),
  ))
  box(width: w, height: h_, inset: 0.2em)[
    #if change {
      [
        101100100
        100110100
        001110011
      ]
    } else {
      [
        110100000
        110010101
        000000110
      ]
    }
  ]
})]

Let's focus on a very common data structure: strings

```rust
fn main() {
    let ph: &str = "Purdue Hackers";
    println!("{ph} is awesome!");
}
```

#pause

#block["Purdue Hackers" is hardcoded into the program executable]

#grid(
  columns: 3,
  [#pause #icon(true)],
  [
    #pause
    // the rect
    #place(horizon + left, dy: 1pt, dx: -124pt + 60pt)[
      #rect(width: 2.85em, height: 1em)[]
    ]
    // top left
    #place(horizon + left, dy: -9pt, dx: -124pt + 60pt)[
      #line(angle: -21deg, length: 6.6em, stroke: gray)
    ]
    // bottom right
    #place(horizon + left, dy: 11pt, dx: -67pt + 60pt)[
      #line(angle: -3.4deg, length: 11.9em, stroke: gray)
    ]
    // bottom left
    #place(horizon + left, dy: 4.9pt, dx: -67pt + 60pt)[
      #line(angle: -6.7deg, length: 3.4em, stroke: gray)
    ]
    // top left
    #place(horizon + left, dy: -9pt, dx: -67pt + 60pt)[
      #line(angle: -11.2deg, length: 3.4em, stroke: gray)
    ]
    #place(horizon + left, dy: -1.5em, dx: 60pt)[
      #let label = rect(inset: 1em)[Purdue~Hackers]
      #context rect(inset: 1em, width: measure(label).width)[Purdue~Hackers]
    ]
  ],
  place(right + horizon, dx: 23em)[#align(center)[#box(width: 17em)[
    #pause
    Let's prove it! \ `xxd target/debug/ownership | less`
  ]]],
)

---

Let's focus on a very common data structure: strings

```rust
fn main() {
    // Reads user input from the terminal into a "String"
    let a: String = utils::read_string();
    println!("{a} is awesome!");
}
```

#pause

#align(center + horizon)[
  #cetz-canvas({
    import cetz.draw: *
    set-style(line: (mark: (end: ">", fill: black), stroke: 2pt))

    let arrow(from, arrow-range: none, range: none, ..args) = {
      let range = if range == none {
        if arrow-range == none { str(from) + "-" } else { str(from) + "-" + str(arrow-range) }
      } else {
        range
      }
      (
        only(range, line(..args)),
        only(from, line(..args, stroke: (paint: color.red), mark: (fill: color.red))),
      )
    }

    content((0, 0), [], name: "d")
    content((5, 0), icon(false), name: "a")
    arrow(2, (to: "d.east", rel: (0.5em, 0)), (to: "a.west", rel: (-0.5em, 0)), name: "l3")
    (
      only("2-", content(
        ((to: "l3.start", rel: (0, 0.25)), 50%, (to: "l3.end", rel: (0, 0.25))),
        anchor: "south",
        [Input],
      )),
    )

    (pause,)

    content((16, 0), image(width: 5em, "os.png"), name: "b")
    content("b.south", anchor: "north", [Operating System])

    (pause,)

    content((27, 0), image(width: 5em, "ram.png"), name: "c")
    content((to: "c.south", rel: (0, -0.25)), anchor: "north", [RAM])

    arrow(3, arrow-range: 5, (to: "a.east", rel: (0.5em, 0)), (to: "b.west", rel: (-0.5em, 0)), name: "l1")
    (
      only("3-5", content(
        ((to: "l1.start", rel: (0, 0.25)), 50%, (to: "l1.end", rel: (0, 0.25))),
        anchor: "south",
        [Asks for \ memory],
      )),
    )

    arrow("6-7", range: "6-", (to: "b.west", rel: (-0.5em, 0)), (to: "a.east", rel: (0.5em, 0)), name: "l1")
    (
      only("6-", content(
        ((to: "l1.start", rel: (0, -0.25)), 50%, (to: "l1.end", rel: (0, -0.25))),
        anchor: "north",
        [`0x557593ec3d60`],
      )),
    )

    arrow(4, arrow-range: 4, (to: "b.east", rel: (0.5em, 0)), (to: "c.west", rel: (-0.5em, 0)), name: "l2")
    (
      only(4, content(
        ((to: "l2.start", rel: (0, 0.25)), 50%, (to: "l2.end", rel: (0, 0.25))),
        anchor: "south",
        align(center)[Finds available \ memory],
      )),
    )

    arrow(5, (to: "c.west", rel: (-0.5em, 0)), (to: "b.east", rel: (0.5em, 0)), name: "l2")
    (
      only("5-", content(
        ((to: "l2.start", rel: (0, -0.25)), 50%, (to: "l2.end", rel: (0, -0.25))),
        anchor: "north",
        align(center)[Block of \ memory],
      )),
    )
  })
]

#jump(7)

#place(bottom + left, dy: 1em)[
  #text(fill: black.transparentize(40%))[
    Note: extremely simplified
  ]
]

---

```rs
// Reads user input from the terminal into a "String"
let a: String = utils::read_string();
```
#set table(rows: 2em, columns: 2, align: center + horizon, inset: 1em, stroke: black, fill: (x, y) => if y == 1 {
  luma(240)
})

#let a-table(two: false) = table(
  table.cell(colspan: 2, stroke: none)[#b[#if two { [ a2 ] } else { [ a ] }]],
  table.header(
    [#b[Type]],
    [#b[Value]],
  ),
  [address], [`0x557593ec3d60`#al(if two { <third> } else { <first> })],
  [length], [`5`],
)

#let heap-table = table(
  columns: 1,
  table.cell(stroke: none)[],
  table.header([#b[Value]]),
  [#al(<second>) `H`],
  [`e`],
  [`l`],
  [`l`],
  [`o`],
)

#place(left, dx: 3em, dy: 1em)[#grid(
  columns: 2,
  column-gutter: 6em,
  a-table(two: false),
  alternatives-match((
    "1-2": table(
      table.cell(colspan: 2, stroke: none)[],
      table.header(
        [#b[Memory address]],
        [#b[Value]],
      ),
      [#al(<second>)`0x557593ec3d60`],
      [`H`],
      [`0x557593ec3d61`],
      [`e`],
      [`0x557593ec3d62`],
      [`l`],
      [`0x557593ec3d63`],
      [`l`],
      [`0x557593ec3d64`],
      [`o`],
    ),
    "3": heap-table,
  )),
)]

#only("2-")[#label-arrow(<first>, <second>, to-offset: (-1.6em, 0.3em), from-offset: (1em, 0.3em))]

---

#slide(repeat: 2, self => [
  How do Strings interact with scopes?

  #local(
    highlight-inset: 0pt,
    highlight-outset: (x: 0.25em, y: 0.3em),
    highlights: ((line: 2, start: 5, end: 5, fill: yellow), (line: 7, start: 5, end: 5, fill: yellow)),
    header: box(height: 0.5em),
    footer: box(height: 0.5em),
  )[
    #animated-code(self, ```rs
    fn main() {
        {
            // `a` is created inside a scope
            let a: String = utils::read_string();
            // `a` is used inside a scope
            println!("{a} is awesome!");
        }
        // `a` has left its scope
        // What happened to the memory? //> 2
    }
    ```)]
])

---

=== Remember: Rust manages memory automatically

- Rust inserts code to free up memory back to the operating system

#align(center)[
  #grid(
    columns: 3,
    column-gutter: 4em,
    a-table(two: false),
    place(dy: 4.1em, dx: -1.5em)[#text(fill: color.red)[#b[Invalid]]],
    {
      show table.cell: it => if it.y >= 2 { hide(it) } else { it }
      set table(fill: (_, y) => if y == 1 { luma(240) } else if y >= 2 { luma(200) })
      heap-table
    },
  )
  #label-arrow(
    <first>,
    <second>,
    to-offset: (3.2em, 1.8em),
    from-offset: (3.9em, 1.8em),
    stroke: color.red + 2pt,
    tip: (
      fill: color.red,
      symbol: ">",
    ),
  )
]

---

#{
  set text(size: 0.75em)
  alternatives[
    #local(
      highlight-inset: 0pt,
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 2, start: 5, end: 5, fill: yellow), (line: 6, start: 5, end: 5, fill: yellow)),
      header: box(height: 0.5em),
      footer: box(height: 0.5em),
    )[
      ```rs
      fn main() {
          {
              let a: String = utils::read_string();
              let a2: String = a;
              println!("{a} is awesome!");
          }
      }
      ```
    ]
  ][
    #local(
      highlight-inset: 0pt,
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 1, start: 11, end: 11, fill: yellow), (line: 5, start: 1, end: 1, fill: yellow)),
      header: box(height: 0.5em),
      footer: box(height: 0.5em),
    )[
      ```rs
      fn main() {
          let a: String = utils::read_string();
          let a2: String = a;
          println!("{a} is awesome!");
      }
      ```
    ]
  ][
    ```rs
    fn main() {
        let a: String = utils::read_string();
        let a2: String = a;
        println!("{a} is awesome!");
    }
    ```
  ]

  pause

  place(center, dx: -4em)[
    #grid(
      columns: 2,
      rows: 2,
      column-gutter: 8em,
      row-gutter: 1.5em,
      a-table(two: false),
      grid.cell(rowspan: 2)[
        \
        \
        #alternatives[
          #heap-table
        ][
          #show table.cell: it => if it.y >= 2 { hide(it) } else { it }
          #set table(fill: (_, y) => if y == 1 { luma(240) } else if y >= 2 { luma(200) })
          #heap-table
        ]
      ],
      a-table(two: true),
    )
    #alternatives-match((
      "4-5": label-arrow(
        <first>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em),
        from-offset: (4.95em - 4em, 0.3em),
        bend: 31,
      ),
      "6-": [
        #label-arrow(
          <first>,
          <second>,
          to-offset: (1.5em - 4em, -2.8em),
          from-offset: (4.95em - 4em, 0.3em),
          bend: 31,
          stroke: color.red + 2pt,
          tip: (fill: color.red, symbol: ">"),
        )
        #place(dx: 16em, dy: -12em)[#text(fill: color.red)[#b[Invalid?]]] ],
    ))
    #alternatives(start: 4)[
      #label-arrow(<third>, <second>, to-offset: (1.5em - 4em, -2.8em), from-offset: (-1.3em, 3em), bend: -60)
    ][
      #label-arrow(
        <third>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em),
        from-offset: (-1.3em, 3em),
        bend: -60,
        stroke: color.red + 2pt,
        tip: (fill: color.red, symbol: ">"),
      )
      #place(dx: 16em, dy: -6.5em)[#text(fill: color.red)[#b[Invalid]]]
    ]
  ]
  uncover("7-")[
    #place(horizon + right, dy: 5em, dx: 1em)[
      #align(center)[ #b[This is a memory safety bug! \ This leads to undefined behavior!] ]
    ]
  ]
}

---

```rs
fn main() {
    let a: String = utils::read_string();
    let a2: String = a;
    println!("{a} is awesome!");
}
```

#text(font: "DejaVu Sans Mono")[
  #text(rgb(205, 75, 62), weight: "bold")[error[E0382]]:
  borrow of moved value: \`a\`
]

= Further Reading <touying:hidden>

- The Rust Programming Language
- Rustlings

#empty-slide[
  #place(dx: 0em, dy: 0em, bottom)[#image(width: 40%, "ferris-gesture.png")]
  #place(dx: 14em, dy: -2em)[#image(width: 50%, "speech.png")]
  #place(dx: 19.5em, dy: 2.25em)[
    #text(size: 2em)[
      Thank you \ for listening!
    ]
  ]
]

#show: appendix
#set text(size: 24pt)

= Appendix <touying:hidden>

#bibliography("bib.yaml")
