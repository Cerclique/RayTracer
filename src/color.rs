use std::ops::{Add, Mul};

pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }
}

impl Color {
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}