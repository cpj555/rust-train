use std::f32::consts::PI;

pub trait CanArea {
    fn area(self: &Self) -> f32;
}

pub fn area<T: CanArea>(shape: &T) -> f32 {
    shape.area()
}

pub struct Rec {
    pub l: f32,
    pub w: f32,
}

impl CanArea for Rec {
    fn area(self: &Self) -> f32 {
        self.l * self.w
    }
}

struct Circle(f32);

impl CanArea for Circle {
    fn area(self: &Self) -> f32 {
        0.5 * PI * self.0
    }
}

struct Tri {
    a: f32,
    h: f32,
}

impl CanArea for Tri {
    fn area(self: &Self) -> f32 {
        0.5 * self.a * self.h
    }
}
