use std::io::{self, BufWriter, Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};

use super::canvas::{Canvas, Cell};

/// Writes a canvas to the terminal, sending only the cells that changed since
/// the previous frame. A full redraw happens after a resize.
pub(crate) struct Renderer {
    out: BufWriter<Stdout>,
    previous: Vec<Cell>,
    full_redraw: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer { out: BufWriter::new(io::stdout()), previous: Vec::new(), full_redraw: true }
    }

    pub fn invalidate(&mut self) {
        self.full_redraw = true;
    }

    pub fn present(&mut self, canvas: &Canvas) -> io::Result<()> {
        let cells = canvas.cells();
        let width = canvas.width();
        if self.previous.len() != cells.len() {
            self.full_redraw = true;
        }
        if self.full_redraw {
            queue!(self.out, Clear(ClearType::All))?;
        }

        let mut cursor: Option<(i32, i32)> = None;
        let mut fg = None;
        let mut bg = None;

        for (i, cell) in cells.iter().enumerate() {
            if !self.full_redraw && self.previous[i] == *cell {
                continue;
            }
            let x = i as i32 % width;
            let y = i as i32 / width;
            if cursor != Some((x, y)) {
                queue!(self.out, MoveTo(x as u16, y as u16))?;
            }
            if fg != Some(cell.fg) {
                queue!(self.out, SetForegroundColor(cell.fg))?;
                fg = Some(cell.fg);
            }
            if bg != Some(cell.bg) {
                queue!(self.out, SetBackgroundColor(cell.bg))?;
                bg = Some(cell.bg);
            }
            queue!(self.out, Print(cell.ch))?;
            cursor = Some((x + 1, y));
        }

        queue!(self.out, SetForegroundColor(Color::Reset), SetBackgroundColor(Color::Reset))?;
        self.out.flush()?;

        self.previous.clear();
        self.previous.extend_from_slice(cells);
        self.full_redraw = false;
        Ok(())
    }
}
