use std::io::{self, stdout};
use std::panic;
use std::sync::atomic::{AtomicBool, Ordering};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size as terminal_size, EnterAlternateScreen,
    LeaveAlternateScreen,
};

static RESTORED: AtomicBool = AtomicBool::new(true);

/// Restores the terminal when dropped. A panic also restores it first, so the
/// panic message is printed on a normal screen instead of a garbled one.
pub(crate) struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        restore();
    }
}

pub(crate) fn enter() -> io::Result<Guard> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture, Hide)?;
    RESTORED.store(false, Ordering::SeqCst);

    let default_hook = panic::take_hook();
    panic::set_hook(Box::new(move |info| {
        restore();
        default_hook(info);
    }));

    Ok(Guard)
}

/// Current terminal size as (columns, rows).
pub(crate) fn size() -> (i32, i32) {
    match terminal_size() {
        Ok((w, h)) => (w as i32, h as i32),
        Err(_) => (80, 24),
    }
}

fn restore() {
    if RESTORED.swap(true, Ordering::SeqCst) {
        return;
    }
    let _ = execute!(stdout(), Show, DisableMouseCapture, LeaveAlternateScreen);
    let _ = disable_raw_mode();
}
