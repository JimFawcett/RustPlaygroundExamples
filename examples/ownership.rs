/////////////////////////////////////////////////////////////
// ownership::main.rs - demonstrates Rust ownership        //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.com, 07 Jun 2020 //
/////////////////////////////////////////////////////////////
#![allow(clippy::many_single_char_names)]

/*-- only one writer in function scope --*/
fn mutate_and_show(t:&mut String) {
    t.clear();
    t.push_str("new value");
    print!("\n  t in function = {:?}", t);
}

fn main() {
    // Blk #1
    /*-- multiple readers: O.K. --*/
    let s = String::from("immutable string");
    let t = &s;
    let u = &s;
    print!("\n  s = {:?}", s);
    print!("\n  t = {:?}", t);
    print!("\n  u = {:?}", u);

    // Blk #2
    /*-- one writer: O.K. --*/
    let mut u = String::from("mutable string");
    u.push_str(" u");
    print!("\n  u = {:?}", u);
    let r = &mut u; /* now u cannot mutate */
    r.push('!');
    print!("\n  r = {:?}", r);
    // next line is ok
    // if r is not used later
    u.push('!');
    print!("\n  u = {:?}", u);
    // next line fails to compile
    // print!("\n  r = {:?}", r);
    /*----------------------------------------------------- 
        attempting to add reader v with active writer u
        not O.K. 
    */
    let v = &u;
    print!("\n  v = {:?}", v);
    u.push_str("modfied by u");
    print!("\n  u = {:?}", u);
    // next line makes u push illegal
    // print!("\n  v = {:?}", v);
    
    // Blk #3
    /*-- mutable v borrows u's ability to write, O.K. --*/
    let v = &mut u;
    v.push_str(" referenced by v");
    print!("\n  v = {:?}", v);

    /*-- attempting to use second writer, not O.K. --*/
    // u.push_str(" and modifed again by u");
    // print!("\n  u = {:?}", u);
    // print!("\n  v = {:?}", v);

    /*-----------------------------------------------------
        can mutate u in function since ref v is not accessible 
    */
    mutate_and_show(&mut u);

    /*------------------------------------------------------
        attempting to use v after borrowing u in function call.
        Not O.K.
        */
    //  v.push_str(" modified by v");

    println!("\n\n  That's all Folks!\n\n");
}
