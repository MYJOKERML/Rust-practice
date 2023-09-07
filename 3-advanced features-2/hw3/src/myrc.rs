use std::ops::Deref;
use std::cell::RefCell;

#[derive(Debug)]
pub struct MyRc<T> {
    data: *mut T,
    ref_count: RefCell<usize>,
}

impl<T> MyRc<T> {
    pub fn new(data: T) -> Self {
        let rc = MyRc {
            data: Box::into_raw(Box::new(data)),
            ref_count: RefCell::new(1),
        };
        rc
    }

    pub fn clone(&self) -> Self {
        *self.ref_count.borrow_mut() += 1;
        MyRc {
            data: self.data,
            ref_count: self.ref_count.clone(),
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.data }
    }
}

impl<T> Drop for MyRc<T> {
    fn drop(&mut self) {
        let count = &mut *self.ref_count.borrow_mut();
        *count -= 1;
        if *count == 0 {
            unsafe {
                let _ = Box::from_raw(self.data);
            }
        }
    }
}

fn main() {
    let rc1 = MyRc::new(42);
    let rc2 = rc1.clone();
    let rc3 = rc2.clone();
    unsafe {
        println!("rc1: {:?}", *rc1.data);
    }
    println!("rc1: {:?}", *rc1.ref_count.borrow()); // 2
    println!("rc2: {}", *rc2.ref_count.borrow()); // 3
    println!("rc3: {}", *rc3.ref_count.borrow()); // 3
    drop(rc2);
    println!("rc1: {:?}", *rc1.ref_count.borrow()); // 2
    println!("rc3: {}", *rc3.ref_count.borrow());
}
