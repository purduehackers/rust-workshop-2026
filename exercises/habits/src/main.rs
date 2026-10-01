//! Habit tracker.
//!
//! Each habit is a row of the last 21 days; the rightmost cell is today.
//!
//!   n        add a habit (type its name, Enter to confirm)
//!   arrows   move between habits and days
//!   space    mark the selected day done (again to clear it)
//!   q        quit
//!
//! Habits are saved to habits.txt when you quit. The first run starts from
//! sample-habits.txt.
//!
//! Parts 1 to 6 replace a `todo!()` each, top to bottom. Part 7 adds a key.
//! Run the program after every part: it stops at the first part you have not
//! written yet and prints the line number.
//!
//! Run this program by entering the following command into your terminal
//! from this directory:
//!
//! cargo run

use std::fs;

use ratatui::Frame;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

/// How many days each habit remembers.
const DAYS: usize = 21;
const FILE: &str = "habits.txt";
const SAMPLE: &str = "sample-habits.txt";

/// One day of one habit.
///
/// `derive` gives `Day` abilities for free: it can be copied and compared with `==`.
#[derive(Clone, Copy, PartialEq)]
enum Day {
    Empty,
    Done,
    Skipped,
}

/// A habit and its last `DAYS` days. The last day is today.
struct Habit {
    name: String,
    days: Vec<Day>,
}

/// Everything on screen.
struct App {
    habits: Vec<Habit>,
    /// Which habit is highlighted.
    selected: usize,
    /// Which day is highlighted.
    day: usize,
    /// True while a new habit name is being typed.
    typing: bool,
    input: String,
}

/// Anything that can be drawn as a row of text pieces.
///
/// A `Span` is one piece of styled text. `'static` means the span owns its
/// text instead of borrowing it from somewhere else.
trait Draw {
    fn draw(&self) -> Vec<Span<'static>>;

    /// Implementing `draw` gives you `line` for free.
    fn line(&self) -> Line<'static> {
        Line::from(self.draw())
    }
}

// Given. Draws every item in one row. Works for anything that implements Draw.
#[allow(dead_code)] // Unused until part 1.
fn draw_all<T: Draw>(items: &[T]) -> Vec<Span<'static>> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    for item in items {
        spans.extend(item.draw());
    }
    spans
}

// 1. One row: the name padded to 12 characters plus a space, then every day
//    drawn with `draw_all`, then "  <done> done, <streak> day streak".
//    Hint: `format!("{:<12} ", self.name)` pads the name.
//    Hint: `Span::from(text)` turns a String into a plain Span.
impl Draw for Habit {
    fn draw(&self) -> Vec<Span<'static>> {
        todo!("part 1: draw one habit row")
    }
}

// 2. One cell: Done is a green "■", Skipped a red "■", Empty a dark gray "·".
//    Hint: `"■".green()` is a green Span.
impl Draw for Day {
    fn draw(&self) -> Vec<Span<'static>> {
        todo!("part 2: draw one day")
    }
}

impl Habit {
    // 3. How many days are Done.
    fn done_count(&self) -> u32 {
        todo!("part 3: count the Done days")
    }

    // 4. How many days in a row are Done, counting back from today.
    //    Hint: `self.days.iter().rev()` walks the days from today backwards.
    fn streak(&self) -> u32 {
        todo!("part 4: count the streak")
    }
}

impl App {
    // 5. The habit with the most Done days. You may assume there is at least one.
    fn best(&self) -> &Habit {
        todo!("part 5: find the best habit")
    }

    // 6. Adds a habit with `DAYS` empty days to the end of the list.
    //    Hint: `vec![Day::Empty; DAYS]` makes the days.
    fn add(&mut self, name: String) {
        todo!("part 6: add a habit called {name}")
    }
}

// 7. Add an `x` key that marks the selected day Skipped, or Empty if it is
//    already Skipped. You need a `skip` method on Habit, shaped like the given
//    `toggle_done`, and a match arm in `handle_key` shaped like the space arm.

// ---------------------------------------------------------------------------
// Given. Everything below already works. Read it to see how your code gets
// called. The only thing to add here is the match arm for part 7.
// ---------------------------------------------------------------------------

impl Habit {
    /// Marks a day Done, or Empty if it is already Done.
    fn toggle_done(&mut self, day: usize) {
        self.days[day] = match self.days[day] {
            Day::Done => Day::Empty,
            _ => Day::Done,
        };
    }
}

impl App {
    /// Draws the whole screen.
    fn draw(&self, frame: &mut Frame) {
        let [header, grid, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let mut lines: Vec<Line<'static>> = Vec::new();
        for (index, habit) in self.habits.iter().enumerate() {
            let mut line: Line<'static> = habit.line();
            if index == self.selected {
                line = line.reversed();
            }
            lines.push(line);
        }
        // A caret under the selected day. Column 13 is the first day.
        let spaces: String = " ".repeat(13 + self.day);
        let ago: usize = DAYS - 1 - self.day;
        lines.push(Line::from(format!("{spaces}^ {ago} days ago")));
        frame.render_widget(Paragraph::new(lines).block(Block::bordered()), grid);

        let mut title: String = format!("{} habits", self.habits.len());
        if !self.habits.is_empty() {
            let best: &Habit = self.best();
            let done: u32 = best.done_count();
            let streak: u32 = best.streak();
            title.push_str(&format!(
                "  ·  best: {} ({done} done, {streak} day streak)",
                best.name
            ));
        }
        frame.render_widget(Paragraph::new(title).bold(), header);

        let help: String = if self.typing {
            format!("name: {}_  (Enter to add, Esc to cancel)", self.input)
        } else {
            String::from("n new habit · arrows move · space done · q quit")
        };
        frame.render_widget(Paragraph::new(help), footer);
    }

    /// Handles one key press. Returns false when it is time to quit.
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        if self.typing {
            match key.code {
                KeyCode::Enter => {
                    let name: String = self.input.clone();
                    self.input.clear();
                    if !name.is_empty() {
                        self.add(name);
                    }
                    self.typing = false;
                }
                KeyCode::Esc => self.typing = false,
                KeyCode::Backspace => {
                    self.input.pop();
                }
                KeyCode::Char(c) => self.input.push(c),
                _ => {}
            }
            return true;
        }

        match key.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('n') => self.typing = true,
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected + 1 < self.habits.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Left => {
                if self.day > 0 {
                    self.day -= 1;
                }
            }
            KeyCode::Right => {
                if self.day + 1 < DAYS {
                    self.day += 1;
                }
            }
            // The `if` after the pattern is a guard: the arm only runs when it holds.
            KeyCode::Char(' ') if !self.habits.is_empty() => {
                self.habits[self.selected].toggle_done(self.day);
            }
            _ => {}
        }
        true
    }

    /// Reads habits.txt, or sample-habits.txt if there is no habits.txt yet.
    /// One habit per line: the name, a space, then one letter per day.
    fn load() -> App {
        // `read_to_string` gives `Ok(text)` or `Err(why)`. On an error we fall
        // back to the sample, and to an empty String if that is missing too.
        let text: String = match fs::read_to_string(FILE) {
            Ok(text) => text,
            Err(_) => fs::read_to_string(SAMPLE).unwrap_or_default(),
        };

        let mut habits: Vec<Habit> = Vec::new();
        for line in text.lines() {
            // Split at the last space. `unwrap` crashes if there is none,
            // which only happens if the file was edited by hand.
            let (name, letters) = line.rsplit_once(' ').unwrap();
            let mut days: Vec<Day> = Vec::new();
            for letter in letters.chars() {
                let day: Day = match letter {
                    'd' => Day::Done,
                    'x' => Day::Skipped,
                    _ => Day::Empty,
                };
                days.push(day);
            }
            // Pad or cut to exactly DAYS days.
            days.resize(DAYS, Day::Empty);
            habits.push(Habit {
                name: name.to_string(),
                days,
            });
        }

        App {
            habits,
            selected: 0,
            day: DAYS - 1,
            typing: false,
            input: String::new(),
        }
    }

    /// Writes habits.txt in the format `load` reads.
    fn save(&self) {
        let mut text: String = String::new();
        for habit in &self.habits {
            text.push_str(&habit.name);
            text.push(' ');
            for day in &habit.days {
                let letter: char = match day {
                    Day::Done => 'd',
                    Day::Skipped => 'x',
                    Day::Empty => '-',
                };
                text.push(letter);
            }
            text.push('\n');
        }
        fs::write(FILE, text).unwrap();
    }
}

/// Draw, wait for a key, repeat.
fn main() {
    let mut app: App = App::load();

    let mut terminal = ratatui::init();
    loop {
        terminal.draw(|frame| app.draw(frame)).unwrap();
        // `if let` runs the block only when the event is a key event.
        if let Event::Key(key) = event::read().unwrap() {
            let quit: bool = key.kind == KeyEventKind::Press && !app.handle_key(key);
            if quit {
                break;
            }
        }
    }
    ratatui::restore();

    app.save();
}
