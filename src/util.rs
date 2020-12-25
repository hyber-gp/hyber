use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub struct Queue<T> {
    pub queue: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { queue: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn dequeue(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn lenght(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    ///remove the first
    pub fn peek(&self) -> Option<&T> {
        self.queue.first()
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Color {
        Color {
            a: a,
            r: r,
            g: g,
            b: b,
        }
    }

    pub fn from_hex(hex: u32) -> Color {
        Color {
            a: ((hex >> 24) & 0xff) as u8,
            r: ((hex >> 16) & 0xff) as u8,
            g: ((hex >> 8) & 0xff) as u8,
            b: (hex & 0xff) as u8,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vector2D {
    pub x: usize,
    pub y: usize,
}

impl Vector2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn from_tuple(tuple: (usize, usize)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<usize> for Vector2D {
    type Output = Self;

    fn add(self, other: usize) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign<usize> for Vector2D {
    fn add_assign(&mut self, other: usize) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

impl Sub for Vector2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<usize> for Vector2D {
    type Output = Self;

    fn sub(self, other: usize) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl SubAssign for Vector2D {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl SubAssign<usize> for Vector2D {
    fn sub_assign(&mut self, other: usize) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

impl Mul for Vector2D {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<usize> for Vector2D {
    type Output = Self;

    fn mul(self, other: usize) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl MulAssign for Vector2D {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl MulAssign<usize> for Vector2D {
    fn mul_assign(&mut self, other: usize) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl Div for Vector2D {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<usize> for Vector2D {
    type Output = Self;

    fn div(self, other: usize) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl DivAssign for Vector2D {
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl DivAssign<usize> for Vector2D {
    fn div_assign(&mut self, other: usize) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        };
    }
}

pub struct IDMachine {
    id: usize,
}

impl IDMachine {
    pub fn new() -> IDMachine {
        IDMachine { id: 0 }
    }

    pub fn fetch_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }
}
