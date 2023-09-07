use std::ops::Deref;
use std::cell::Cell;

pub struct MyRc<T> {
    data: *mut T,
    ref_count: *mut Cell<usize>,
}

impl<T> MyRc<T> {
    pub fn new(data: T) -> Self {
        let rc = MyRc {
            data: Box::into_raw(Box::new(data)),
            ref_count: Box::into_raw(Box::new(Cell::new(1))),
        };
        rc
    }

    pub fn clone(&self) -> Self {
        unsafe {
            let count = &mut *self.ref_count;
            count.set(count.get() + 1);
        }
        MyRc {
            data: self.data,
            ref_count: self.ref_count,
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
        unsafe {
            let count = &mut *self.ref_count;
            count.set(count.get() - 1);
            if count.get() == 0 {
                let _ = Box::from_raw(self.data);
                let _ = Box::from_raw(self.ref_count);
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
    println!("rc1 ref count: {}", unsafe { (*rc1.ref_count).get() }); // 3
    println!("rc2 ref count: {}", unsafe { (*rc2.ref_count).get() }); // 3
    println!("rc3 ref count: {}", unsafe { (*rc3.ref_count).get() }); // 3
    drop(rc2);
    println!("rc1 ref count: {}", unsafe { (*rc1.ref_count).get() }); // 2
    println!("rc3 ref count: {}", unsafe { (*rc3.ref_count).get() }); // 2
}
