#import "@preview/larrow:1.2.0": *
#import "@preview/touying:0.7.4": *
#import themes.metropolis: *

#import "@preview/cetz:0.5.2"
#import "@preview/cetz-venn:0.2.0"

#let cetz-canvas = touying-reducer.with(
  reduce: cetz.canvas,
  cover: cetz.draw.hide.with(bounds: true),
)

#let b(x, c) = text(weight: if c { "bold" } else { "regular" })[#x]

#show: metropolis-theme.with(
  align: start,
  // config-common(new-section-slide-fn: none),
  config-info(
    title: [Introduction to the Rust Programming Language],
    subtitle: [#datetime(year: 2026, month: 10, day: 1).display() — A Purdue Hackers workshop],
    author: [Arhan Chaudhary],
    institution: [https://github.com/ArhanChaudhary],
    contact: [arhan.ch\@gmail.com],
  ),
)

#show raw.where(block: true): block.with(
  fill: luma(240),
  stroke: 0.5pt + luma(200),
  inset: 1em,
  radius: 0.3em,
  width: 100%,
)
#show raw.where(block: true): set text(size: 1.2em)

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
        // not-abc-stroke: none,
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
      lbl("v.b", dx: -1em, dy: -3.5em)[Assembly]
      lbl("v.b", dx: 1.75em, dy: 3em)[Zig]
      lbl("v.b", dx: 1.25em, dy: -1em)[C]
      lbl("v.ac", dx: -1em, dy: 1em)[Typescript]
      lbl("v.ac", dx: -1.2em, dy: -1em)[Swift]
      lbl("v.a", dx: -1.25em, dy: -0.5em)[Python]
      lbl("v.a", dx: -2em, dy: 2em)[Go]
      lbl("v.a", dx: 0.75em, dy: -3em)[Java]

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
          name: [Data~Race~Free],
          anchor: "north",
          line-anchor: "north-west",
          content-anchor: "south-east",
          angle: 270deg,
          offset: (158deg, 15em),
          note: box(width: 10em)[#set align(right); No unsynchronized writes by threads to shared memory.],
          rel: (0.1, 0.1),
        ),
      )

      hide(bounds: true, {
        for ax in axes {
          let bk = "bounds-" + ax.key
          content(("v." + ax.key, -70%, "v.abc"), b([#ax.name], true), anchor: ax.anchor, name: bk)
          content((rel: ax.offset, to: bk), ax.note)
        }
      })

      for (i, ax) in axes.enumerate() {
        let on = if self.subslide == 1 { true } else { self.subslide == i + 2 }
        let k = ax.key + "-on"
        let c = ax.key + "-content"

        content(("v." + ax.key, -70%, "v.abc"), b([#ax.name], on), name: k, anchor: ax.anchor)

        circle((ax.angle, D), radius: R, fill: none, stroke: if on { black } else { gray })

        if on and self.subslide != 1 {
          content((rel: ax.offset, to: k), ax.note, name: c)
          line(
            (rel: (-0.05, -0.05), to: (name: k, anchor: ax.line-anchor)),
            (rel: ax.rel, to: (name: c, anchor: ax.content-anchor)),
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

  #image(width: 40%, "qr.png")
]

= Hello World <touying:hidden>

```rust
fn main() {
    println!("Hello World!");
}
```

#image(width: 20%, "ferris-gesture.png")

= Ownership <touying:hidden>

=== How do programming languages manage memory?
#pause
- Python, Java, Typescript: automatically with overhead
#pause
- C, C++: you must manage memory yourself
#pause
What does Rust do? #pause It picks a third option:

- Rust: the compiler enforces the rules of ownership! #pause
  - The rules manage memory without overhead #pause
  - If any rules are violated, your program won't compile #pause

---

#slide[
  Let's focus on a very common data structure: strings

  #only("1-2")[
    ```rust
    fn main() {
        let ph: &str = "Purdue Hackers";
        println!("{ph} is awesome!");
    }
    ```
  ]

  #only(2)["Purdue Hackers" is hardcoded into the program]
  #only("3-4")[
    ```rust
    fn main() {
        let idk: &str = utils::read_string();
        println!("{idk} is awesome!");
    }
    ```
  ]
  #only(4)[
    Rust program ->
  ]
  // #arrow-label()
]



#focus-slide[Thank you for listening!]

#show: appendix
#set text(size: 24pt)

= Appendix

== References

#bibliography("bib.yaml")
