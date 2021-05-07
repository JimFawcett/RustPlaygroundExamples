// String comparisons
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::vec_init_then_push)]

fn show_type<T: ?Sized>(t:&T) {
    let typ = std::any::type_name::<T>();
    print!("{} ", typ);
}

fn main() {
    let s = "a literal string";
    print!("\n  ");
    show_type(s);
    show_type(&s);
    show_type(&*s);
    show_type(&s[..]);
    // show_type(&*(s[..]));  // fails - str can't be deref'd
    
    let strg = String::from(s);
    
    let demo = strg.as_str() == s;
    print!("\n  demo is {}", demo);
    
    let demo = strg == *s;
    print!("\n  demo is {}", demo);

    let demo = strg == s;           // clippy: redundant slicing
    print!("\n  demo is {}", demo);

    let demo = strg == &s[..];      // clippy: op_ref, redundant slicing
    print!("\n  demo is {}", demo);

    let demo = &strg == s;          // clippy: needless ref of left op
    print!("\n  demo is {}", demo);
}