use std::cell::RefCell;

#[derive(Debug)]
pub struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    pub fn new() -> Self {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    pub fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

