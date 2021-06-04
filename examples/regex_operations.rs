// Regular Expression Parsing
#![allow(dead_code)]
#![allow(unused_imports)]

use regex::{Regex, Match, Captures};

fn check(pattern: &str, text: &str, pred:bool) {
    if pred {
        print!("\n  pattern: {:?}  matches text: {:?}", pattern, text);
    }
    else {
        print!("\n  pattern: {:?} !matches text: {:?}", pattern, text);
    }
}

fn range(pattern: &str, text: &str, mat: &Option<Match>) {
    if let Some(mt) = mat {
        print!("\n  find pattern {:?} in text {:?}:", pattern, text);
        print!("  match in [{}, {})", mt.start(), mt.end());
    }
    else {
        print!("\n  no match");
    }
}

fn range_iter(pattern: &str, text: &str, mat: Match) {
    print!("\n  find pattern {:?} in text {:?}:", pattern, text);
    print!("  match in [{}, {})", mat.start(), mat.end());
}

fn test_match() {
    print!("\n  -- test_match --");
    let pattern = r"[a-q]{3,4}$";
    let re = Regex::new(pattern).unwrap();
    let text = "12cde";
    let pred = re.is_match(text);
    check(pattern, text, pred);

    let text = "12cdefg";
    let pred = re.is_match(text);
    check(pattern, text, pred);

    let text = "12cd";
    let pred = re.is_match(text);
    check(pattern, text, pred);

    let text = "12cd3e";
    let pred = re.is_match(text);
    check(pattern, text, pred);

    let text = "12cds";
    let pred = re.is_match(text);
    check(pattern, text, pred);
}

fn test_find() {
    print!("\n  -- test_find --");
    let pattern = r"abc";
    let re = Regex::new(pattern).unwrap();
    let text = "123abc456";
    let op: Option<Match> = re.find(text);
    range(pattern, text, &op);
}

fn test_find_iter() {
    print!("\n  -- test_find_iter --");
    let pattern = r"abc";
    let re = Regex::new(pattern).unwrap();
    let text = "123abc456abc789";
    let matches = re.find_iter(text);
    for mat in matches {
        range_iter(pattern, text, mat);
    }
}

fn test_captures() {
    print!("\n  -- test_captures --");
    let text = "123abc456def789";
    let pattern = "\
      ([a-z]{3}|[0-9]{3})\
      ([a-z]{3}|[0-9]{3})\
      ([a-z]{3}|[0-9]{3})\
      ([a-z]{3}|[0-9]{3})\
      ([a-z]{3}|[0-9]{3})\
    ";
    // These don't work as you might expect.
    // Capture doesn't work well with repetitions
    // let pattern = "([a-z]{3}|[0-9]{3}){5}";
    // let pattern = r"([a-z]{3}|[0-9]{3}){5}";
    // let pattern = r"(([a-z]{3})([0-9]{3}))+";
    // let pattern = r"((?:\d+)+)+";
    let re = Regex::new(pattern).unwrap();
    let captures: Option<Captures> = re.captures(text);
    print!("\n  captures: {:?}", captures);
    let caps = captures.unwrap();
    for i in 0..caps.len() {
        print!("\n  captures[{}] = {:?}", i, &caps.get(i));
        let cap = &caps.get(i).unwrap();
        print!(
            "\n  cap = {:?}, {}, {}", 
            cap.as_str(), cap.start(), cap.end()
        );
    }
}

fn main() {
    test_match();
    test_find();
    test_find_iter();
    test_captures();
}