use crossterm::style::Color;

/// One character cell on screen.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) struct Cell {
    pub ch: char,
    pub fg: Color,
    pub bg: Color,
}

impl Cell {
    const BLANK: Cell = Cell { ch: ' ', fg: Color::Reset, bg: Color::Reset };
}

/// The screen for one frame. Coordinates are columns and rows, with (0, 0) at
/// the top left. Drawing outside the screen is silently ignored, so you never
/// have to bounds-check.
///
/// Each cell holds one character, so use characters that take one column
/// (letters, `●`, `█`, box-drawing symbols). Emoji take two columns and will
/// look wrong.
pub struct Canvas {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}

impl Canvas {
    pub(crate) fn new(width: i32, height: i32) -> Canvas {
        let mut canvas = Canvas { width: 0, height: 0, cells: Vec::new() };
        canvas.resize(width, height);
        canvas
    }

    pub(crate) fn resize(&mut self, width: i32, height: i32) {
        self.width = width.max(0);
        self.height = height.max(0);
        self.cells = vec![Cell::BLANK; (self.width * self.height) as usize];
    }

    pub(crate) fn clear(&mut self) {
        self.cells.fill(Cell::BLANK);
    }

    pub(crate) fn cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Number of columns.
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Number of rows.
    pub fn height(&self) -> i32 {
        self.height
    }

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width + x) as usize)
        }
    }

    /// Draws one character.
    pub fn put(&mut self, x: i32, y: i32, ch: char, color: Color) {
        if let Some(i) = self.index(x, y) {
            self.cells[i].ch = ch;
            self.cells[i].fg = color;
        }
    }

    /// Draws one character with an explicit background color.
    pub fn put_bg(&mut self, x: i32, y: i32, ch: char, fg: Color, bg: Color) {
        if let Some(i) = self.index(x, y) {
            self.cells[i] = Cell { ch, fg, bg };
        }
    }

    /// Draws a string starting at (x, y), left to right.
    pub fn text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        for (offset, ch) in text.chars().enumerate() {
            self.put(x + offset as i32, y, ch, color);
        }
    }

    /// Fills a rectangle with one character.
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, ch: char, color: Color) {
        for row in y..y + h {
            for col in x..x + w {
                self.put(col, row, ch, color);
            }
        }
    }

    /// Fills the whole screen with one character.
    pub fn fill(&mut self, ch: char, color: Color) {
        self.rect(0, 0, self.width, self.height, ch, color);
    }

    /// Paints the background of the whole screen.
    pub fn background(&mut self, color: Color) {
        for cell in &mut self.cells {
            cell.bg = color;
        }
    }
}
