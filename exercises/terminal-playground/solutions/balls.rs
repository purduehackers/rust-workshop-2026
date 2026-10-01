// The starter with every `Try:` applied: colours, gravity, floor friction,
// collisions, trails, mouse attraction and a clear key.
//
// Run it with `cargo run --bin balls`.

use engine::{App, Canvas, Color, Frame};

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
}

impl Ball {
    fn new(x: f32, y: f32) -> Ball {
        Ball {
            x,
            y,
            dx: rand::random_range(-1.0..1.0),
            dy: rand::random_range(-0.5..0.5),
            color: Color::Rgb {
                r: rand::random_range(100..=255),
                g: rand::random_range(100..=255),
                b: rand::random_range(100..=255),
            },
        }
    }

    fn speed(&self) -> f32 {
        (self.dx * self.dx + self.dy * self.dy).sqrt()
    }
}

struct Playground {
    balls: Vec<Ball>,
}

impl Playground {
    fn move_balls(&mut self) {
        for ball in &mut self.balls {
            ball.dy += 0.03;
            ball.x += ball.dx;
            ball.y += ball.dy;
        }
    }

    fn bounce_off_walls(&mut self, width: i32, height: i32) {
        let max_x = (width - 1) as f32;
        let max_y = (height - 1) as f32;
        for ball in &mut self.balls {
            if ball.x < 0.0 {
                ball.x = 0.0;
                ball.dx = ball.dx.abs();
            }
            if ball.x > max_x {
                ball.x = max_x;
                ball.dx = -ball.dx.abs();
            }
            if ball.y < 0.0 {
                ball.y = 0.0;
                ball.dy = ball.dy.abs();
            }
            if ball.y > max_y {
                ball.y = max_y;
                ball.dy *= 0.8;
                ball.dy = -ball.dy.abs();
            }
        }
    }

    /// Swaps the velocities of balls that touch while moving toward each
    /// other. Without the "moving toward" check, overlapping balls swap every
    /// frame and stick together.
    fn collide_balls(&mut self) {
        for i in 0..self.balls.len() {
            for j in i + 1..self.balls.len() {
                let dx = self.balls[j].x - self.balls[i].x;
                let dy = self.balls[j].y - self.balls[i].y;
                let touching = dx * dx + dy * dy < 1.0;
                let relative_dx = self.balls[j].dx - self.balls[i].dx;
                let relative_dy = self.balls[j].dy - self.balls[i].dy;
                let approaching = dx * relative_dx + dy * relative_dy < 0.0;
                if touching && approaching {
                    let (dx_i, dy_i) = (self.balls[i].dx, self.balls[i].dy);
                    self.balls[i].dx = self.balls[j].dx;
                    self.balls[i].dy = self.balls[j].dy;
                    self.balls[j].dx = dx_i;
                    self.balls[j].dy = dy_i;
                }
            }
        }
    }

    fn handle_input(&mut self, frame: &Frame) {
        if frame.pressed(' ') {
            let x = frame.width as f32 / 2.0;
            let y = frame.height as f32 / 2.0;
            self.balls.push(Ball::new(x, y));
        }
        if frame.pressed('c') {
            self.balls.clear();
        }
        if frame.mouse.held {
            for ball in &mut self.balls {
                ball.dx += (frame.mouse.x as f32 - ball.x) * 0.002;
                ball.dy += (frame.mouse.y as f32 - ball.y) * 0.002;
            }
        }
    }
}

impl App for Playground {
    fn update(&mut self, frame: &Frame) {
        self.handle_input(frame);
        self.move_balls();
        self.bounce_off_walls(frame.width, frame.height);
        self.collide_balls();
    }

    fn draw(&self, canvas: &mut Canvas) {
        for ball in &self.balls {
            let trail_x = (ball.x - ball.dx) as i32;
            let trail_y = (ball.y - ball.dy) as i32;
            canvas.put(trail_x, trail_y, '·', Color::DarkGrey);
        }
        for ball in &self.balls {
            let ch = if ball.speed() > 0.5 { '●' } else { '∘' };
            canvas.put(ball.x as i32, ball.y as i32, ch, ball.color);
        }
        let status = format!(
            "balls: {}   space: add   mouse: attract   c: clear   esc: quit",
            self.balls.len()
        );
        canvas.text(1, 0, &status, Color::DarkGrey);
    }
}

fn main() {
    let mut balls = Vec::new();
    for _ in 0..20 {
        balls.push(Ball::new(rand::random_range(0.0..80.0), rand::random_range(0.0..20.0)));
    }
    engine::run(Playground { balls });
}
