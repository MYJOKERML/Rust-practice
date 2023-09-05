use std::ops::Add;

pub struct  Buffer<T> {
    buffer: Vec<T>,
}

impl<T> Buffer<T> {
    pub fn new(v: Vec<T>) -> Buffer<T> {
        Buffer { buffer: v }
    }

    pub fn sum(&self) -> T
    where
        T: Add<Output = T> + Copy,
    {
        let mut sum = self.buffer[0];
        for i in 1..self.buffer.len() {
            sum = sum + self.buffer[i];
        }
        sum
    }
}