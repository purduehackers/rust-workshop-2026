use std::cell::Cell;
use std::io;
use std::time::Instant;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

/// A key press. `Key::Char` covers letters, digits, space and punctuation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Char(char),
    Up,
    Down,
    Left,
    Right,
    Enter,
    Backspace,
    Tab,
    Esc,
}

impl From<char> for Key {
    fn from(c: char) -> Key {
        Key::Char(c)
    }
}

/// Where the mouse is and whether a button is held down.
///
/// Terminals only report the position while a button is held or, in some
/// terminals, while it moves. Before any mouse event `x` and `y` are 0.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub held: bool,
}

/// Everything an app can know about the current frame.
pub struct Frame {
    /// Screen width in columns.
    pub width: i32,
    /// Screen height in rows.
    pub height: i32,
    /// Frames since the program started.
    pub tick: u64,
    /// Seconds since the program started.
    pub time: f32,
    /// Seconds since the previous frame.
    pub dt: f32,
    /// Mouse state.
    pub mouse: Mouse,
    keys: Vec<Key>,
    quit: Cell<bool>,
}

impl Frame {
    pub(crate) fn new(
        width: i32,
        height: i32,
        tick: u64,
        time: f32,
        dt: f32,
        keys: Vec<Key>,
        mouse: Mouse,
    ) -> Frame {
        Frame { width, height, tick, time, dt, mouse, keys, quit: Cell::new(false) }
    }

    /// True if the key was pressed during this frame.
    /// Accepts a `char` or a `Key`: `frame.pressed(' ')`, `frame.pressed(Key::Up)`.
    pub fn pressed(&self, key: impl Into<Key>) -> bool {
        self.keys.contains(&key.into())
    }

    /// All keys pressed during this frame, in order.
    pub fn keys(&self) -> &[Key] {
        &self.keys
    }

    /// Ends the program after this frame.
    pub fn quit(&self) {
        self.quit.set(true);
    }

    pub(crate) fn quit_requested(&self) -> bool {
        self.quit.get()
    }
}

/// Events gathered while waiting for the next frame.
pub(crate) struct Collected {
    pub keys: Vec<Key>,
    pub resized: Option<(i32, i32)>,
    pub quit: bool,
}

pub(crate) struct InputState {
    mouse: Mouse,
    quit_on_esc: bool,
}

impl InputState {
    pub fn new(quit_on_esc: bool) -> InputState {
        InputState { mouse: Mouse::default(), quit_on_esc }
    }

    pub fn mouse(&self) -> Mouse {
        self.mouse
    }

    /// Reads terminal events until `deadline`, so this doubles as the frame timer.
    pub fn collect(&mut self, deadline: Instant) -> Collected {
        let mut collected = Collected { keys: Vec::new(), resized: None, quit: false };
        loop {
            let now = Instant::now();
            if now >= deadline {
                break;
            }
            match event::poll(deadline - now) {
                Ok(true) => {}
                Ok(false) => break,
                Err(_) => {
                    collected.quit = true;
                    break;
                }
            }
            match event::read() {
                Ok(event) => self.handle(event, &mut collected),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(_) => {
                    collected.quit = true;
                    break;
                }
            }
        }
        collected
    }

    fn handle(&mut self, event: Event, collected: &mut Collected) {
        match event {
            Event::Key(key) => {
                if key.kind == KeyEventKind::Release {
                    return;
                }
                let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
                let mapped = match key.code {
                    KeyCode::Char('c') if ctrl => {
                        collected.quit = true;
                        return;
                    }
                    KeyCode::Esc if self.quit_on_esc => {
                        collected.quit = true;
                        return;
                    }
                    KeyCode::Char(c) => Key::Char(c),
                    KeyCode::Up => Key::Up,
                    KeyCode::Down => Key::Down,
                    KeyCode::Left => Key::Left,
                    KeyCode::Right => Key::Right,
                    KeyCode::Enter => Key::Enter,
                    KeyCode::Backspace => Key::Backspace,
                    KeyCode::Tab => Key::Tab,
                    KeyCode::Esc => Key::Esc,
                    _ => return,
                };
                collected.keys.push(mapped);
            }
            Event::Mouse(mouse) => {
                self.mouse.x = mouse.column as i32;
                self.mouse.y = mouse.row as i32;
                match mouse.kind {
                    MouseEventKind::Down(_) => self.mouse.held = true,
                    MouseEventKind::Up(_) => self.mouse.held = false,
                    _ => {}
                }
            }
            Event::Resize(w, h) => collected.resized = Some((w as i32, h as i32)),
            _ => {}
        }
    }
}
