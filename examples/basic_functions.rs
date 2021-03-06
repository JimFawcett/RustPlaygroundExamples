// functions_and_methods::main.rs

fn f(mut s:String, t:String) -> String {
    s.push(' ');
    s.push_str(&t);
    s
}

fn g(s:&mut String, t:&str) -> String {
    s.push(' ');
    s.push_str(t);
    s.to_string()
}

fn main() {

    print!("\n  -- passing function arguments by value --");
    let s1 = String::from("a string");
    print!("\n  s1 = {:?}", s1);
    let s2 = String::from("and more");
    print!("\n  s2 = {:?}", s2);
    
    let s3 = f(s1, s2);
    
    print!("\n  s3 = {:?}", s3);
    // print!("\n  s2 = {:?}", s2);
    print!("\n  can't print s2, it's been moved");
    // print!("\n  s1 = {:?}", s1);
    print!("\n  can't print s1, it's been moved");
    println!();

    print!("\n  -- passing function arguments by reference --");
    let mut s1 = String::from("a refreshed string");
    let s2 = "and a new more";
    let s3 = g(&mut s1, s2);

    print!("\n  s3 = {:?}", s3);
    print!("\n  s2 = {:?}", s2);
    print!("\n  s1 = {:?}", s1);
    print!("\n  note: s1 has been changed as a side-effect");

    print!("\n\n  That's all Folks!\n\n");
}
