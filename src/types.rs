use std::ops::{Add, Div};

pub struct DisplayOptions {
    pub width: usize,
    pub height: usize,
}

impl DisplayOptions {
    pub fn default() -> DisplayOptions {
        DisplayOptions {
            width: 60,
            height: 30,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<u8> for Color {
    type Output = Self;

    fn div(self, scalar: u8) -> Self {
        Self {
            r: self.r / scalar,
            g: self.g / scalar,
            b: self.b / scalar,
        }
    }
}
