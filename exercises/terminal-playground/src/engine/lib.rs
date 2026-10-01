//! A small terminal engine: it owns the terminal, the frame loop, input and
//! rendering, so `main.rs` only has to describe what to update and what to draw.
//!
//! Nothing in this directory needs editing during the workshop.

mod canvas;
mod input;
mod render;
mod terminal;

use std::time::{Duration, Instant};

pub use canvas::Canvas;
pub use crossterm::style::Color;
pub use input::{Frame, Key, Mouse};

/// Something the engine can run. Implement this for your own struct.
pub trait App {
    /// Called once per frame, before drawing. Change your state here.
    fn update(&mut self, frame: &Frame);

    /// Called once per frame, after updating. Only reads state; the canvas is
    /// blank when this is called.
    fn draw(&self, canvas: &mut Canvas);
}

/// Engine settings. `Config::default()` is fine for almost everything.
pub struct Config {
    /// Frames per second the engine aims for.
    pub fps: u32,
    /// Whether pressing Esc ends the program. Ctrl+C always does.
    pub quit_on_esc: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config { fps: 30, quit_on_esc: true }
    }
}

/// Current terminal size as (columns, rows). Useful for building a grid or
/// placing things before `run` starts.
pub fn screen_size() -> (i32, i32) {
    terminal::size()
}

/// Runs the app with default settings until it quits.
pub fn run(app: impl App) {
    run_with(app, Config::default());
}

/// Runs the app with custom settings until it quits.
pub fn run_with(mut app: impl App, config: Config) {
    let _guard = match terminal::enter() {
        Ok(guard) => guard,
        Err(e) => {
            eprintln!("This program needs a real terminal window, not an editor's output pane.");
            eprintln!("Open a terminal, go to this folder and run `cargo run` there. ({e})");
            std::process::exit(1);
        }
    };

    let (mut width, mut height) = terminal::size();
    let mut canvas = Canvas::new(width, height);
    let mut renderer = render::Renderer::new();
    let mut input = input::InputState::new(config.quit_on_esc);

    let frame_time = Duration::from_secs_f64(1.0 / config.fps.max(1) as f64);
    let start = Instant::now();
    let mut last = start;
    let mut tick = 0;

    loop {
        let collected = input.collect(last + frame_time);
        if collected.quit {
            break;
        }
        if let Some((w, h)) = collected.resized {
            width = w;
            height = h;
            canvas.resize(width, height);
            renderer.invalidate();
        }

        let now = Instant::now();
        let frame = Frame::new(
            width,
            height,
            tick,
            (now - start).as_secs_f32(),
            (now - last).as_secs_f32(),
            collected.keys,
            input.mouse(),
        );
        last = now;
        tick += 1;

        app.update(&frame);
        if frame.quit_requested() {
            break;
        }

        canvas.clear();
        app.draw(&mut canvas);
        renderer
            .present(&canvas)
            .expect("could not write to the terminal");
    }
}
