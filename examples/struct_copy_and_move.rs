#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::*;

fn show_type<T>(t:&T) {
    let st = std::any::type_name::<T>();
    print!("\n  type:  {:?}", st);
}

fn show_value<T: Debug>(t:&T) {
    print!("\n  value: {:?}", t);
}
/*-----------------------------------------
No-copy arguments will be moved
*/
fn demo<T:Debug>(t:T) {
  show_type(&t);
  show_value(&t);
}

/*-----------------------------------------
refs so arguments will not be moved
*/
fn demo_ref<T>(t:&T) where T:Debug  {
  show_type(t);
  show_value(t);
}

#[derive(Debug)]
struct Point<T> { x:T, y:T, z:T, }

// Copy trait works because blittable, provided
// that T is blittable
#[derive(Debug, Copy, Clone)]
struct BetterPoint<T> { x:T, y:T, z:T, }

fn title(msg:&str) {
    print!("\n  -- {} --\n", msg);
}
fn putline() {
    println!();
}
fn putlinen(n:u32) {
    for i in 0..n {
       println!();
    }
}
fn main() {
    let mut s = String::from("this is a test");

    title("demo_ref");
    demo_ref(&s);
    #[allow(clippy::approx_constant)]
    let pi = 3.1415927;
    demo_ref(&pi);
    s.push('Z');
    putline();
    
    title("demo");
    demo(s);
    demo(pi);
    // statement below won't compile - s moved
    // s.push('Z');
    putline();
    title("demo_ref and demo with struct");
    let mut pt = Point { x:1.0, y:-1.5, z:2.3 };
    demo_ref(&pt);
    pt.x = 3.2;
    demo(pt);
    // statement below won't compile - pt moved for demo
    // pt.x = 3.2;
    putline();
    
    title("demo_ref and demo with copy-able struct");
    let mut bpt = BetterPoint { x:1.0, y:-1.5, z:2.3 };
    demo_ref(&bpt);
    bpt.x = 3.2;
    demo(bpt);
    // statement ok - pt copied for demo
    bpt.x = 3.2;
    putline();
    
    title("demo_ref and demo with copy-able struct");
    let mut bpt = BetterPoint { x:"one", y:"two", z:"three" };
    demo_ref(&bpt);
    bpt.x = "1";
    demo(bpt);
    // statement ok - pt copied for demo
    bpt.x = "one";
    putline();
    
    title("demo_ref and demo with non copy-able struct");
    let mut bpt = BetterPoint {
      x:"one".to_string(),
      y:"two".to_string(),
      z:"three".to_string()
    };
    demo_ref(&bpt);
    bpt.x = "four".to_string();
    demo(bpt);
    // won't compile - bpt not blittable so it was moved
    // bpt.x = "one".to_string();
    
    putlinen(2);
}