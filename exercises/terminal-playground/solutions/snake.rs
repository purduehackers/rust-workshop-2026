// A complete Snake game, as an example of a grid-based game on the engine.
//
// Run it with `cargo run --bin snake`.

use engine::{App, Canvas, Color, Frame, Key};

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    /// Head first, tail last.
    body: Vec<(i32, i32)>,
    direction: Direction,
    food: (i32, i32),
    alive: bool,
    score: u32,
}

fn random_cell(width: i32, height: i32) -> (i32, i32) {
    // Row 0 holds the score line.
    (rand::random_range(0..width), rand::random_range(1..height))
}

impl Snake {
    fn new(width: i32, height: i32) -> Snake {
        let center = (width / 2, height / 2);
        Snake {
            body: vec![center, (center.0 - 1, center.1), (center.0 - 2, center.1)],
            direction: Direction::Right,
            food: random_cell(width, height),
            alive: true,
            score: 0,
        }
    }

    /// Changes direction on arrow keys, never straight back into the body.
    fn turn(&mut self, frame: &Frame) {
        let wanted = if frame.pressed(Key::Up) {
            Direction::Up
        } else if frame.pressed(Key::Down) {
            Direction::Down
        } else if frame.pressed(Key::Left) {
            Direction::Left
        } else if frame.pressed(Key::Right) {
            Direction::Right
        } else {
            return;
        };
        let opposite = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        };
        if wanted != opposite {
            self.direction = wanted;
        }
    }

    /// Moves one cell. Eating food grows the snake by not removing the tail.
    fn step(&mut self, width: i32, height: i32) {
        let (head_x, head_y) = self.body[0];
        let next = match self.direction {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        };

        let off_screen = next.0 < 0 || next.0 >= width || next.1 < 1 || next.1 >= height;
        if off_screen || self.body.contains(&next) {
            self.alive = false;
            return;
        }

        self.body.insert(0, next);
        if next == self.food {
            self.score += 1;
            self.food = random_cell(width, height);
        } else {
            self.body.pop();
        }
    }
}

impl App for Snake {
    fn update(&mut self, frame: &Frame) {
        if !self.alive {
            if frame.pressed(Key::Enter) {
                *self = Snake::new(frame.width, frame.height);
            }
            return;
        }
        self.turn(frame);
        // Terminal cells are about twice as tall as they are wide, so moving
        // every frame vertically would look twice as fast as horizontally.
        let moving_sideways = matches!(self.direction, Direction::Left | Direction::Right);
        let frames_per_step = if moving_sideways { 2 } else { 4 };
        if frame.tick.is_multiple_of(frames_per_step) {
            self.step(frame.width, frame.height);
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        for (i, &(x, y)) in self.body.iter().enumerate() {
            let ch = if i == 0 { '█' } else { '▓' };
            canvas.put(x, y, ch, Color::Green);
        }
        canvas.put(self.food.0, self.food.1, '●', Color::Red);
        canvas.text(1, 0, &format!("score: {}   arrows: turn   esc: quit", self.score), Color::DarkGrey);

        if !self.alive {
            let message = "game over, press enter";
            let x = (canvas.width() - message.len() as i32) / 2;
            canvas.text(x, canvas.height() / 2, message, Color::Yellow);
        }
    }
}

fn main() {
    let (width, height) = engine::screen_size();
    engine::run(Snake::new(width, height));
}
