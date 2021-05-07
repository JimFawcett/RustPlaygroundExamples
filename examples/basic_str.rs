// Basic str demo

fn main() {
    let s = "a literal string";
    print!("\n  -- chars --\n  ");
    for ch in s.chars() {
        print!("{} ", ch);
    }
    print!("\n  -- char_indices --");
    for item in s.char_indices() {
        print!("\n  {:?} ", item);
    }
    print!("\n  -- find --");
    let ch = 't';
    if let Some(indx) = s.find(ch) {
        print!("\n  found \'{}\' at index {} in {:?}", ch, indx, s);
    }
    else {
        print!("\n  did not find \'{}\' in {:?}", ch, s);
    }
    print!("\n  -- demonstrate copy, t = s --");
    let t = s;
    let addr_t = &t;
    let addr_s = &s;
    print!("\n  address of s = {:p}", addr_s);
    print!("\n  address of t = {:p}", addr_t);
}