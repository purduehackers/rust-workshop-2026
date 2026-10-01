use engine::{App, Canvas, Color, Frame};

/// One moving dot.
///
/// Try: give each ball its own colour. Add `color: Color` here, set it in
/// `new` with `Color::Rgb { r: rand::random_range(0..=255), g: ..., b: ... }`
/// and use it in `draw`.
struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    fn new(x: f32, y: f32) -> Ball {
        Ball {
            x,
            y,
            dx: rand::random_range(-1.0..1.0),
            dy: rand::random_range(-0.5..0.5),
        }
    }
}

/// Everything the program remembers between frames.
struct Playground {
    balls: Vec<Ball>,
}

/// Each step of the simulation is its own method. Add more and call them
/// from `update` below.
impl Playground {
    /// Moves every ball one step.
    fn move_balls(&mut self) {
        for ball in &mut self.balls {
            // Try: gravity. Add `ball.dy += 0.03;` here.
            ball.x += ball.dx;
            ball.y += ball.dy;
        }
    }

    /// Keeps balls on screen by turning them around at the edges.
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
                // Try: lose some energy on the floor with `ball.dy *= 0.8;`.
                ball.dy = -ball.dy.abs();
            }
        }
    }

    /// Bounces balls off each other. Empty on purpose: this one is yours.
    fn collide_balls(&mut self) {
        // Two balls touch when they are less than one cell apart.
        // Rust will not let you hold two `&mut` balls from the same Vec at
        // once, so use indices and read before you write:
        //
        // for i in 0..self.balls.len() {
        //     for j in i + 1..self.balls.len() {
        //         let dx = self.balls[j].x - self.balls[i].x;
        //         let dy = self.balls[j].y - self.balls[i].y;
        //         if dx * dx + dy * dy < 1.0 {
        //             let (dx_i, dy_i) = (self.balls[i].dx, self.balls[i].dy);
        //             self.balls[i].dx = self.balls[j].dx;
        //             self.balls[i].dy = self.balls[j].dy;
        //             self.balls[j].dx = dx_i;
        //             self.balls[j].dy = dy_i;
        //         }
        //     }
        // }
    }

    /// Reacts to the keyboard and mouse.
    fn handle_input(&mut self, frame: &Frame) {
        if frame.pressed(' ') {
            let x = frame.width as f32 / 2.0;
            let y = frame.height as f32 / 2.0;
            self.balls.push(Ball::new(x, y));
        }
        if frame.mouse.held {
            self.balls.push(Ball::new(frame.mouse.x as f32, frame.mouse.y as f32));
        }
        // Try: `frame.pressed('c')` empties the screen with `self.balls.clear()`.
        // Try: instead of painting, pull every ball toward the held mouse:
        //      ball.dx += (frame.mouse.x as f32 - ball.x) * 0.002;
        //      ball.dy += (frame.mouse.y as f32 - ball.y) * 0.002;
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
            // Try: a trail. Before the dot, draw '·' in Color::DarkGrey at
            //      (ball.x - ball.dx, ball.y - ball.dy).
            // Try: a character that depends on speed: '●' when fast, '∘' when slow.
            canvas.put(ball.x as i32, ball.y as i32, '●', Color::Cyan);
        }
        let status = format!("balls: {}   space: add   mouse: paint   esc: quit", self.balls.len());
        canvas.text(1, 0, &status, Color::DarkGrey);
    }
}

fn main() {
    engine::run(Playground { balls: vec![Ball::new(10.0, 5.0)] });
}
