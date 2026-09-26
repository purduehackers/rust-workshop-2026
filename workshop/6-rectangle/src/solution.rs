pub struct Rectangle {
    pub width: f64,
    pub height: f64,
    pub color: Color,
}

pub enum Color {
    Blue,
    Red,
    Grayscale(f64),
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    pub fn describe(&self) {
        print!(
            "Rectangle: width={}; height={} color=",
            self.width, self.height
        );
        match self.color {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
            Color::Grayscale(level) => println!("grayscale({level})"),
        }
    }
}
