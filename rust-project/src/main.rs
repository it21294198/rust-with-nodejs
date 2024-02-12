// src/main.rs
use rust_project::{add_two_numbers, fib};

pub fn double_fun(n:i32)->i32{
    n*2
}
fn main() {
    let result1 = add_two_numbers(2, 3);
    println!("2+3={}",result1);
    let resuldt2 = fib(15);
    println!("{}",resuldt2);
    let num_to_double = 34;
    println!("Double {}",double_fun(num_to_double));
}
