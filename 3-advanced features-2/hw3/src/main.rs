#[macro_use]
extern crate std;

use std::collections::HashMap;

mod lifo;
mod myrc;

use lifo::SimpleStack;
use myrc::MyRc;

macro_rules! hash_map {
    ($($key:expr => $value:expr ), *) => {
        {
            let mut map = HashMap::new();
            $(
                    map.insert($key, $value);
            )*
            map
        }
    };
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
    let map = hash_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("Test macro hash_map: ");
    println!("{:?}", map);

    println!("\nTest lifo: ");
    let stack = my_stack![1, 2, 3, 4, 5];
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    stack.push(6);
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());
    println!("Poped value {:?}", stack.pop());

    println!("\nTest myrc: ");
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
