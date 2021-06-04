// Interior mutability shifts invariant checking to run-time

#![allow(unused_imports)]
#![allow(dead_code)]

use std::cell::RefCell;   // mutable, dynamically checked

/*-----------------------------------------------------
  Illustrates that perfectly safe code can fail to 
  compile.  The safety invariant is very strong.
-----------------------------------------------------*/
fn may_fail_to_build() {
    print!("\n  -- no interior mutability --");
    let mut x = 42i32;
    print!("\n    x = {}", x);
    print!("\n    - immutable reference -");
    let rx = &x;
    print!("\n    *rx = {}", rx);
    print!("\n    - mutable reference -");
    let mrx = &mut x;
    *mrx += 1;
    // drop(mrx); won't save compilation - not used by borrow chkr
    print!("\n    *mrx = {}", mrx);
    // error: attempt to use immutable ref with mutable ref in scope
    //        causes compile failure
    // print!("\n    *rx = {}", rx);
    println!();
    // final observation: error is not due to declaration
    // it's due to use.
}

/*-----------------------------------------------------
  Illustrates that code violating safety invariant
  can compile with the help of interior mutability.
  - you still can't break the iinvariant rule due to
    run-time checks.  However, using locks avoids 
    run-time failure (not illustrated here, but later).
-----------------------------------------------------*/
fn may_fail_at_runtime() {
    print!("\n  -- interior mutability --");
    let x = RefCell::new(42i32);
    print!("\n    x = {}", x.into_inner());
    // RefCell is not copy so now invalid
    print!("\n    - immutable reference -");
    let x = RefCell::new(42i32);
    let rx = x.borrow();
    print!("\n    *rx = {}", rx);
    // commenting out drop causes run-time panic
    drop(rx);
    print!("\n    - mutable reference -");
    *x.borrow_mut() += 1;  // mutable ref ends here
    print!("\n    *mrx = {}", x.borrow());
    
}
fn main() {
    may_fail_to_build();
    may_fail_at_runtime();
}