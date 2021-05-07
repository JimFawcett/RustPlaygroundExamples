#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::vec_init_then_push)]

use std::fmt::Debug;
// use std::char::*;
use std::collections::{VecDeque, HashMap};
use std::collections::hash_map::{Entry};

fn title(msg: &str) {
    print!("\n  -- {} --\n", msg);
}
/*-------------------------------------------*/
fn array_ops() {
    title("array_ops");
    let mut arr: [u8;6] = [1, 2, 3, 4, 5, 6];
    print!("\n  arr: {:?}", arr);
    
    print!("\n  arr items: ");
    for item in &arr {
        print!("{} ", item);
    }
    let slice = &mut arr[1..4];
    print!("\n  slice items: ");
    for item in slice {
        print!("{} ", item);
    }
    let slice = &mut arr[1..4];
    slice[0] = 255;
    print!("\n  slice: {:?}", slice);
    print!("\n  arr: {:?}", arr);
    println!();
}
/*-------------------------------------------*/
fn string_ops() {
    title("string_ops()");
    let ls = "abc123";
    print!(
        "\n  size of char: {} bytes", 
        std::mem::size_of::<char>()
    );
    for chi in ls.char_indices() {
        let (_, ch) = chi;
        print!(
            "\n  indexed: {:?}, char: {}", 
            chi, ch
        );
    }
    println!();
    let mut s = String::from("Hello Dolly");
    print!("\n  chars: ");
    for ch in s.chars() {
        print!("{:?} ", ch);
    }
    println!();
    s.push(' ');
    // s.push(2764 as char);
    s.push('\u{2764}');
    s.push(' ');
    print!("\n  {:?}", s);
    s.push_str(ls);
    print!("\n  {:?}", s);
    println!();
    
    let f = |t:&char| t.is_numeric();
    let numerics:String = s.chars().filter(f).collect();
    print!("\n  numerics: {:?}", &numerics);
    println!();
}
/*-------------------------------------------*/
fn vec_ops() {
    title("vec_ops");
    let mut v1 = Vec::<u8>::new();
    v1.push(b'a');  // same as 'a' as u8
    v1.push(b'b');
    v1.push(b'1');
    print!("\n  {:?}", v1);
    
    print!("\n  ");
    for item in v1 {
        print!("{} ", item as char);
    }
    println!();
    
    /*-- push trait object --*/
    let answer = 42;
    let message = "hello";
    let float = 2.7212;
    
    let display: Vec<&dyn Debug> = vec![&message, &answer, &float];
    
    for d in display {
        print!("\n  got {:?}", d);
    }    
    println!();
}
/*-------------------------------------------*/
fn vecdeque_ops() {
    title("vecdeque_ops");
    let mut vd = VecDeque::<char>::new();
    vd.push_front('z');
    vd.push_front('y');
    vd.push_front('x');
    print!("\n  {:?}", vd);
    print!("\n  vd.back() = {:?}", vd.back());
    vd.pop_back();
    print!("\n  {:?}", vd);
    println!();
}
/*-------------------------------------------*/
fn hashmap_ops() {
    title("hashmap_ops");
    let mut h = HashMap::<String, u32>::new();
    h.insert("zero".to_string(), 0);
    h.insert("one".to_string(), 1);
    h.insert("two".to_string(), 2);
    print!("\n  {:?}", h);
    println!();
    h.insert("four".to_string(), 5);
    print!("\n  {:?}", h);
    h.insert("four".to_string(), 4);  // replace value
    print!("\n  {:?}", h);
    h.insert("three".to_string(), 3);  // replace value
    print!("\n  {:?}", h);
    println!();
    
    let iter= h.iter();
    for item in iter {
        print!("\n  {:?}", item);
    }
    println!();
    
    let iter = h.iter();  // prev iter was moved
    for (key, value) in iter {
        print!("\n  key: {:?}, value: {:?}", key, value);
    }
    println!();
    
    let key = "three";
    let mut value = 0;
    if h.contains_key(key) {
        value = h[key];
    }
    print!("\n  key: {:?}, value: {:?}", key, value);
    println!();
    // let key = "fortytwo".to_string();
    // let value = h[key];  // panics
    let key = "fortytwo".to_string();
    let entry = h.entry(key);
    print!("\n  entry: {:?}", entry);
    let key = "three".to_string();
    let entry = h.entry(key);
    print!("\n  entry: {:?}", entry);
    if let Entry::Occupied(rentry) = entry {
        print!("\n  entry: {:?}", rentry);
        print!(
            "\n  key: {:?}, value: {:?}", 
            rentry.key(), 
            rentry.get()
        );
    }
    println!();
}
/*-------------------------------------------*/
fn main() {
    array_ops();
    string_ops();
    vec_ops();
    vecdeque_ops();
    hashmap_ops();
}
