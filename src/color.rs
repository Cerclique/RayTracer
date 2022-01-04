use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Clone)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {r, g, b}
    }

    pub fn zeroes() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Color {
        Color::new(1.0, 1.0, 1.0)
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

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        *self = Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b);
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Color::new(self.r * rhs, self.g * rhs, self.b * rhs);
    }
}