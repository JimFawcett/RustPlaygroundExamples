use std::fmt::Debug;

fn arrays() {
    print!("\n  -- create and display arrays --");
    let arr1: [i32;3] = [1, 2, 3];
    print!("\n  arr1 = {:?}", arr1);
    
    let mut arr2 = arr1;
    arr2[1] = -2;
    print!("\n  arr2 = {:?}", arr2);
    println!();
}
fn tuples() {
    print!("\n  -- create and display tuples --");
    let t: (&'static str, i32, char) = ("a tuple", 42, 'Z');
    let mut u = t;
    u.1 = -42;     print!("\n  w = {:?}", u);
    println!();
}
fn structs() {
    print!("\n  -- create and display structs --");
    #[derive(Debug, Copy, Clone)]
    struct MyType { d: f64, s:&'static str, c: char }
    let s = MyType { d: 3.14, s: "a string", c: 'Z' }; 
    let t = s; 
    print!("\n  t = {:?}", t);
    println!();
}
fn vecs() {
    print!("\n  -- create and display vecs --");
    let mut v = Vec::<i32>::new();
    v.push(1);
    v.push(2);
    print!("\n  v = {:?}", v);
  
    let s = [7i32, 6, 5, 4];
    v.extend_from_slice(&s);
    print!("\n  v = {:?}", v);
  
    let w : Vec<f64> = vec![3.1415927, -0.3333];
    print!("\n  w = {:?}", w);
    println!();
}
use std::collections::{VecDeque};

fn vecdeques() {
    print!("\n  -- create and display vecdeques --");
    let mut v = VecDeque::<i32>::new();
    print!("\n  v = {:?}", v);
    print!("\n  push_back 1 and 2");
    v.push_back(1);
    v.push_back(2);
    print!("\n  v = {:?}", v);
    print!("\n  pop_front");
    v.pop_front();
    print!("\n  v = {:?}", v);
    println!();
}
use std::collections::{HashMap};

fn hashmaps() {
    print!("\n  -- create and display hashmaps --");
    let mut h = HashMap::<String, i32>::new();
    print!("\n  h = {:?}", h);
    print!("\n  insert(\"one\",1)");
    h.insert("one".to_string(),1);
    print!("\n  h = {:?}", h);
    print!("\n  insert(\"two\",2)");
    h.insert("two".to_string(),2);
    print!("\n  h = {:?}", h);
    println!();
}
fn strings() {
    print!("\n  -- create and display strings --");
    // create literal string
    let ls: &str = "a literal string";
    print!("\n  ls = {:?}", ls);
    
    // create String
    let s = String::from(ls);
    // create string
    let mut t = String::from("String from literal string - ");
    // append to string
    t.push_str(&s);
    print!("\n  t = {:?}", t);
    println!();
}
fn strs() {
    print!("\n  -- create, copy, and display strs --");
    let s = "a literal string";
    let t = "another literal string";
    print!("\n  s = {:?}", &s);
    print!("\n  t = {:?}", &t);
    
    let t = s;
    print!("\n  s = {:?}", &s);
    print!("\n  t = {:?}", &t);
}
fn main() {
    print!("\n  -- create and display data structures --\n");
    arrays();
    tuples();
    structs();
    vecs();
    vecdeques();
    hashmaps();
    strings();
    strs();

    print!("\n\n  That's all Folks!\n\n");
}
