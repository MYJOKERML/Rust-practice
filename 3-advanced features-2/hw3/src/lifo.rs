use std::cell::RefCell;

#[derive(Debug)]
struct SimpleStack<T> {
    stack: RefCell<Vec<T>>,
}

impl<T> SimpleStack<T> {
    fn new() -> Self {
        SimpleStack {
            stack: RefCell::new(Vec::new()),
        }
    }

    fn push(&self, value: T) {
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self) -> Option<T> {
        self.stack.borrow_mut().pop()
    }
}

macro_rules! my_stack {
    ($($value:expr), *) => {
        {
            let stack = SimpleStack::new();
            $(
                stack.push($value);
            )*
            stack
        }
    }
}

fn main() {
    let stack = my_stack![1, 2, 3, 4, 5];
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    stack.push(6);
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
}