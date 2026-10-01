#import "@preview/larrow:1.2.0": *
#import "@preview/touying:0.7.4": *
#import themes.metropolis: *
#import "@preview/cetz:0.5.2"
#import "@preview/cetz-venn:0.2.0"
#import "@preview/codly:1.3.0": *
#import "@preview/codly-languages:0.1.1": *
#show: codly-init

#show link: underline

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
    let meta-parts = parts.at(1, default: "1").split(" ")
    let min-i = int(meta-parts.at(0))
    if self.subslide >= min-i {
      keep.push(parts.at(0))
    } else {
      let newlines = int(meta-parts.at(1, default: 1))
      for _ in range(newlines) {
        keep.push("")
      }
    }
  }
  raw(keep.join("\n"), block: code.block, lang: code.lang)
}

#let ownership-example = ```rs
fn main() {
    let a: String = utils::read_string();
    let a2: String = a;
    println!("{a} is awesome!");
}
```

#let red(n) = (n, color.red.lighten(70%))
#let green(n) = (n, color.green.lighten(60%))
#let yellow(n) = (n, color.yellow.lighten(60%))
#let hl(g: (), r: (), h: ()) = codly(
  highlighted-lines: g.map(green) + r.map(red) + h.map(yellow),
)

#let err(n, body) = text(font: "DejaVu Sans Mono")[
  #text(rgb(205, 75, 62), weight: "bold")[error[E#n]]:
  #show raw: set text(size: 1.2em)
  #body
]

#let cetz-canvas = touying-reducer.with(
  reduce: cetz.canvas,
  cover: cetz.draw.hide.with(bounds: true),
)

#let b(x, c: true) = text(weight: if c { "bold" } else { "regular" })[#x]

#let label-arrow = label-arrow.with(
  tip: (symbol: ">", fill: black),
  stroke: 2pt,
)

#let codly-hl = local.with(
  highlight-inset: 0pt,
  header: box(height: 0.5em),
  footer: box(height: 0.5em),
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
  config-common(
    new-section-slide-fn: none,
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

#pause

#align(horizon)[
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

= How is Rust empowering?

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

= Setting Up

#slide(align: center + horizon)[
  === https://code.purduehackers.com/new/purduehackers/rust-workshop-2026
]

= Hello World

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

= Variables

#slide(repeat: 4, self => {
  if self.subslide == 1 {
    ```rs
    fn main() {
        let power = 1000;
        println!("The power level is: {power}");


    }
    ```
  } else if self.subslide == 2 {
    codly-hl(
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 1, start: 11, end: 11), (line: 6, start: 1)),
    )[
      ```rs
      fn main() { // this creates a scope
          let power = 1000;
          println!("The power level is: {power}");


      }
      ```
    ]
    [Variables are only valid inside scopes (curly braces)]
  } else {
    ```rs
    fn main() {
        let power = 1000;
        println!("The power level is: {power}");
        power = 2000;
        println!("The power level is now: {power}");
    }
    ```
    if self.subslide == 4 {
      err([0384], [cannot assign twice to immutable variable \`power\`])

      align(center + bottom)[#image(height: 1fr, "ferris-confused.svg")]
    }
  }
})

---

#codly-hl(
  highlight-outset: (x: 0em, y: 0.3em),
  highlights: (
    (line: 2, start: 8, end: 12, fill: color.green.lighten(60%)),
  ),
  highlight-fill: color => color,
)[
  ```rs
  fn main() {
      let mut power = 1000;
      println!("The power level is: {power}");
      power = 2000;
      println!("The power level is now: {power}");
  }
  ```
]

Variables are _immutable_ (cannot change) by default; _mutability_ (can change) is opt-in

#align(center + bottom)[#image(height: 1fr, "ferris-happy.png")]

---

#slide(repeat: 5, self => {
  if self.subslide == 5 {
    codly-hl(
      highlight-outset: (x: 0em, y: 0.3em),
      highlights: (
        (line: 4, start: 8, end: 12, fill: color.green.lighten(60%)),
      ),
      highlight-fill: color => color,
      highlighted-lines: ((10, color.green.lighten(60%)),),
    )[
      ```rs
      fn data_types() {
          // Every variable has a type
          // `integer` has the integer type `i32`
          let mut integer = 5;
          // `integer2` is optionally annotated explicitly
          let integer2: i32 = 5;
          let true_or_false: bool = true;
          let decimal: f64 = 4.5;

          integer = decimal; // different types
      }```
    ]
    text(font: "DejaVu Sans Mono")[
      #err([0308], [mismatched types])
    ]
  } else {
    animated-code(self)[```rs
    fn data_types() {
        // Every variable has a type
        // `integer` has the integer type `i32`
        let integer = 5;
        // `integer2` is optionally annotated explicitly //> 2
        let integer2: i32 = 5; //> 2
        let true_or_false: bool = true; //> 3
        let decimal: f64 = 4.5; //> 4


    }```]
  }
})

= Control Flow

#slide(repeat: 3, self => {
  animated-code(self)[```rs
  fn main() {
      // Reads a user input number from the terminal
      let integer: i32 = utils::read_number();
      if integer == 42 { //> 2
          println!("42 is the meaning of life"); //> 2
      } //> 2
      let abs = if integer < 0 { //> 3
          -integer //> 3
      } else { //> 3
          integer //> 3
      }; //> 3
  }```]
})

---

#slide(repeat: 2, self => {
  animated-code(self)[```rs
  fn main() {
      // Reads a user input number from the terminal
      let integer: i32 = utils::read_number();
      // If `integer` is positive, let's //> 2
      // subtract one until `integer` is zero. //> 2
      while integer != 0 { //> 2
          println!("Count down: {integer}"); //> 2
          integer -= 1; //> 2
      } //> 2
  }```]
})


= Ownership

#pause

#align(horizon + center)[
  === Ownership can be confusing!
  Don't be afraid to ask questions!
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
  - Memory unsafe $=>$ the rules are violated $=>$ your program won't compile

---



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

  #codly-hl(
    highlight-outset: (x: 0.25em, y: 0.3em),
    highlights: ((line: 2, start: 5, end: 5), (line: 7, start: 5, end: 5)),
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
    #codly-hl(
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 2, start: 5, end: 5), (line: 6, start: 5, end: 5)),
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
    #codly-hl(
      highlight-outset: (x: 0.25em, y: 0.3em),
      highlights: ((line: 1, start: 11, end: 11), (line: 5, start: 1, end: 1)),
    )[#ownership-example]
  ][#ownership-example]

  jump(3)

  place(center, dx: -4em, dy: 0.25em)[
    #grid(
      columns: 2,
      rows: 2,
      column-gutter: 8em,
      row-gutter: 1.5em,
      a-table(two: false),
      grid.cell(rowspan: 2)[
        \
        \
        #alternatives(start: 3)[
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
      "3-4": label-arrow(
        <first>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em - 0.25em),
        from-offset: (4.95em - 4em, 0.3em - 0.25em),
        bend: 31,
      ),
      "5-": [
        #label-arrow(
          <first>,
          <second>,
          to-offset: (1.5em - 4em, -2.8em - 0.25em),
          from-offset: (4.95em - 4em, 0.3em - 0.25em),
          bend: 31,
          stroke: color.red + 2pt,
          tip: (fill: color.red, symbol: ">"),
        )
        #place(dx: 16em, dy: -12em)[#text(fill: color.red)[#b[Invalid?]]] ],
    ))
    #alternatives(start: 3)[
      #label-arrow(<third>, <second>, to-offset: (1.5em - 4em, -2.8em - 0.25em), from-offset: (-1.3em, 3em), bend: -60)
    ][
      #label-arrow(
        <third>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em - 0.25em),
        from-offset: (-1.3em, 3em),
        bend: -60,
        stroke: color.red + 2pt,
        tip: (fill: color.red, symbol: ">"),
      )
      #place(dx: 16em, dy: -6.5em)[#text(fill: color.red)[#b[Invalid]]]
    ]
  ]
  uncover("6-")[
    #place(horizon + right, dy: 5em, dx: 1em)[
      #align(center)[ #b[This is a memory safety bug! \ This leads to undefined behavior!] ]
    ]
  ]
}

---

#ownership-example

#err([0382], [borrow of moved value: \``a`\`])

#align(center + bottom)[#image(height: 1fr, "ferris-happy.png")]

---

=== What are the rules of ownership?

#let rule-one = [
  1) Each value in Rust has an owner
]

#let rule-two = [
  2) There can only be one owner at a time
]

#let rule-three = [
  3) When the owner goes out of scope, the value will be cleaned up
]

#{
  pause
  rule-one
  pause
  linebreak()
  rule-two
  pause
  linebreak()
  rule-three
}

---

#text(size: 0.75em)[
  #ownership-example
]

How does this program violate the rules of ownership?

---

#{
  set text(size: 0.75em)
  alternatives[
    #hl(h: (2,))
    #ownership-example
  ][
    #hl(h: (3,))
    #ownership-example
  ][
    #hl(h: (5,))
    #ownership-example
  ][
    #hl(r: (4,))
    #ownership-example
  ]

  place(right + top, dx: -0.5em, dy: 0.5em)[
    #set text(weight: "bold", size: 1.05em)
    #let rule-box(color, body) = box(
      inset: 0.75em,
      fill: color.lighten(70%),
      stroke: color.lighten(5%) + 1pt,
      radius: 1mm,
      body,
    )
    #alternatives[
      #rule-box(blue)[#rule-one]
    ][
      #rule-box(blue)[#rule-two]
    ][
      #rule-box(blue)[
        3) When the owner goes out of scope, \
        the value will be cleaned up
      ]
    ][
      #rule-box(color.red)[Variable used after value was moved]
    ]
  ]

  place(center, dx: -4em, dy: 0.25em)[
    #grid(
      columns: 2,
      rows: 2,
      column-gutter: 8em,
      row-gutter: 1.5em,
      alternatives[
        #a-table(two: false)
      ][
        #set table.cell(fill: luma(170))
        #a-table(two: false)
      ],
      grid.cell(rowspan: 2)[
        \
        \
        #alternatives-match((
          "1-2": heap-table,
          "3-": {
            show table.cell: it => if it.y >= 2 { hide(it) } else { it }
            set table(fill: (_, y) => if y == 1 { luma(240) } else if y >= 2 { luma(200) })
            heap-table
          },
        ))
      ],
      only("2-")[
        #a-table(two: true)
      ],
    )
    #alternatives[
      #label-arrow(
        <first>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em - 0.25em),
        from-offset: (4.95em - 4em, 0.3em - 0.25em),
        bend: 31,
      )
    ][
      #label-arrow(
        <third>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em - 0.25em),
        from-offset: (-1.3em, 3em),
        bend: -60,
      )
    ][
      #label-arrow(
        <third>,
        <second>,
        to-offset: (1.5em - 4em, -2.8em - 0.25em),
        from-offset: (-1.3em, 3em),
        bend: -60,
        stroke: color.red + 2pt,
        tip: (fill: color.red, symbol: ">"),
      )
      #place(dx: 16em, dy: -6.5em)[#text(fill: color.red)[#b[Invalid]]]
    ]
  ]
}

---

```rs
fn main() {
    let a: String = utils::read_string();
    foo(a);
    println!("{a}")
}

fn foo(s: String) {
    println!("In function: {s}");
}
```

#pause

#err([0382], [borrow of moved value: \``a`\`])

---

#codly-hl(
  highlight-outset: (x: 0em, y: 0.3em),
  highlights: (
    (line: 7, start: 18, end: 28, fill: color.green.lighten(60%)),
    (line: 3, start: 5, end: 12, fill: color.green.lighten(60%)),
  ),
  highlight-fill: color => color,
  highlighted-lines: ((9, color.green.lighten(60%)), (10, color.green.lighten(60%))),
)[
  ```rs
  fn main() {
      let a: String = utils::read_string();
      let a = foo(a);
      println!("{a}")
  }

  fn foo(s: String) -> String {
      println!("In function: {s}");
      // Return the variable `s`
      s
  }
  ```
]

#pause

This is too verbose!

= Borrowing

#codly-hl(
  highlight-outset: (x: 0em, y: 0.4em),
  highlights: (
    (line: 7, start: 11, end: 11, fill: color.green.lighten(60%)),
    (line: 3, start: 9, end: 9, fill: color.green.lighten(60%)),
  ),
  highlight-fill: color => color,
)[
  ```rs
  fn main() {
      let a: String = utils::read_string();
      foo(&a);
      println!("{a}")
  }

  fn foo(s: &String) {
      println!("In function: {s}");
  }
  ```
]

- \``&a`\` _borrows_ the value from the owner #pause
  - Immutable borrow (*Read-only*) #pause
  - What if we wanted to borrow, but write to the value?

---

#slide(repeat: 3, self => {
  codly-hl(
    highlight-outset: (x: 0em, y: 0.4em),
    highlights: (
      (line: 2, start: 8, end: 12, fill: color.green.lighten(60%)),
      (line: 7, start: 11, end: 15, fill: color.green.lighten(60%)),
      (line: 3, start: 9, end: 13, fill: color.green.lighten(60%)),
    ),
    highlight-fill: color => color,
  )[
    #animated-code(self, ```rs
    fn main() {
        let mut a: String = utils::read_string();
        foo(&mut a);
        println!("{a}")
    }

    fn foo(s: &mut String) {
        println!("In function: {s}");
        // Append an exclamation mark to the end //> 2
        s.push('!'); //> 2
    }
    ```)
  ]
  only("3-")[- \``&mut a`\` mutably borrows the value (*Read and write*)]
})

---

```rs
fn main() {
    let a: String = utils::read_string();

    let b: &String = &a;
    let c: &String = &a;
    let d: &String = &a;

    // ... a bunch of complicated code

    println!("{b} {c} {d}");
}
```

---

#codly-hl(
  highlight-outset: (x: 0em, y: 0.4em),
  highlights: (
    (line: 2, start: 8, end: 12, fill: color.green.lighten(60%)),
    (line: 6, start: 13, end: 16, fill: color.green.lighten(60%)),
    (line: 6, start: 27, end: 30, fill: color.green.lighten(60%)),
  ),
  highlight-fill: color => color,
)[
  ```rs
  fn main() {
      let mut a: String = utils::read_string();

      let b: &String = &a;
      let c: &String = &a;
      let d: &mut String = &mut a;

      // ... a bunch of complicated code

      println!("{b} {c} {d}");
  }
  ```
]

#pause

#place(bottom, dy: 0.5em)[
  #err([0502], [cannot borrow \``a`\` as mutable because it is also borrowed as immutable])
]

= Structs, Enums, and Traits

#align(center + horizon)[
  === Let's do some live coding!
]

---

= Further Learning

- _The Rust Programming Language_ book (#link("https://doc.rust-lang.org/stable/book/")[doc.rust-lang.org/stable/book])
  - How I learned Rust
  - We roughly covered the first 6 out of 21 chapters in this workshop
- Rustlings (#link("https://rustlings.rust-lang.org/")[rustlings.rust-lang.org])
  - Complementary learning exercises as you read the book

#empty-slide[
  #place(dx: 0em, dy: 0em, bottom)[#image(width: 40%, "ferris-gesture.png")]
  #place(dx: 14em, dy: -2em)[#image(width: 50%, "speech.png")]
  #text(size: 2em)[
    #alternatives[
      #place(dx: 9.75em, dy: 1em)[
        Thank you \ for listening!
      ]
    ][
      #place(dx: 9.5em, dy: 1.25em)[
        Let's do some \ exercises!
      ]
    ]
  ]
]

#show: appendix
#set text(size: 24pt)

= Appendix

#bibliography("bib.yaml")
