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

pub enum MessageXPTO {
    Abc,
    Dfg,
}

#[derive(Clone)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8
}

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}