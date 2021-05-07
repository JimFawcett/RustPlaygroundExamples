#![allow(dead_code)]
#![allow(unused_variables)]
// use std::fmt::*;

fn do_match(c: char) {
    match c {
        ('0'..='9') => 
            print!("\n  {} is a digit", c),
        ('a'..='z') => 
            print!("\n  {} is lower case ascii char", c),
        ('A'..='Z') => 
            print!("\n  {} is upper case ascii char", c),
        _ => print!("\n  {} is some other char type", c)
    }
}

fn main() {
    let x = 'a';
    do_match(x);
    do_match('Q');
    do_match('7');
    do_match('@');
}
