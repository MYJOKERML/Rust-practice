mod generic;
mod cmpstr;

use generic::Buffer;
use cmpstr::compare_string;

fn main() {
    let buffer = Buffer::new(vec![1, 2, 3, 4, 5]);
    println!("sum: {}", buffer.sum());

    let s1: &str = "abcd";
    let s2: &str = "bcdef";
    println!("{}", compare_string(s2, s1));

    let t: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];
    let t_prime: Vec<char> = t.iter().map(|x| ((*x as u8 + 1) as char)).collect();
    println!("{:?}", t_prime);
    
}
