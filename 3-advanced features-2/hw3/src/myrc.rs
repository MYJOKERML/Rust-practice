use std::mem;

#[derive(Debug)]
pub struct MyRc<T> {
    value: T,
    ref_count: usize,
}

impl<T> MyRc<T> {
    pub fn new(value: T) -> Self {
        MyRc {
            value: value,
            ref_count: 1,
        }
    }

    pub fn clone(&self) -> Self
    where
        T: Copy,
    {
        self.ref_count += 1;
        *self
    }

    pub fn drop(&mut self) {
        self.ref_count -= 1;
        if self.ref_count == 0 {
            mem::drop(self.value);
        }
    }

    pub fn DeRef(&self) -> &T {
        &self.value
    }

    pub fn Copy(&self) -> T
    where
        T: Copy,
    {
        self.value
    }
}

fn main() {
    let rc = MyRc::new(1);
    let _rc2 = rc.clone();
    // println!("rc: {:?}, rc2: {:?}", rc, rc2);
}