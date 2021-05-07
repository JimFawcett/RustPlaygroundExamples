// Copy and Move

#![allow(dead_code)]
use std::fmt::Debug;

fn show_type_and_value<T: Debug>(t:&T) {
    let typ = std::any::type_name::<T>();
    print!("\n  type is {:?}, value is {:?}", typ, t);
}
fn test_copy() {
    print!("\n  -- test copy of i32 value --");
    let x:i32 = 42;
    let rx = &x;
    print!("\n  address of x = {:p}", rx);
    show_type_and_value(rx);
    let y = x;
    let ry = &y;
    print!("\n  address of y = {:p}", ry);
    show_type_and_value(ry);
    println!();
}
fn test_move() {
    print!("\n  -- test move of Vec<i32> --");
    let x: Vec<i32> = vec![42, 21];
    let rx = &x;
    let rx0 = &x[0];
    print!("\n  address of x = {:p}", rx);
    print!("\n  address of x[0] = {:p}", rx0);
    show_type_and_value(rx);
    let y = x;
    let ry = &y;
    let ry0 = &y[0];
    print!("\n  address of y = {:p}", ry);
    print!("\n  address of y[0] = {:p}", ry0);
    print!("\n  -- note addresses of x and y are different --");
    print!("\n  -- note addresses of x[0] and y[0] are same");
    show_type_and_value(ry);
    println!();
}

fn main() {
    test_copy();
    test_move();
}