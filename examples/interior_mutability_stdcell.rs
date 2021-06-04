// Interior mutability

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::cell::Cell;      // mutable memory location
use std::cell::Ref;       // immutable value wrapper returned from RefCell
use std::cell::RefMut;    // mutable value wrapper returned from RefCell
use std::cell::RefCell;   // mutable, dynamically checked

/*--------------------------------------------------
    Cell<T> holds t:T on stack
    - immutable interface, focus on value in, value out
      - Cell<T>::get(&self) -> T
      - Cell<T>::set(&self, t:T)
      - Cell<T>::Replace(&self, t:T) -> T
    - no run-time checking because ownership is
      transfered, so no memory safety issues
--------------------------------------------------*
    fn increment_usize mutates cell value passed as immutable arg
    - because get() -> T and set(t:T) pass values
      there are no memory safety issues
--------------------------------------------------*/
fn increment_usize(rc:&Cell<usize>) {
    let num = rc.get() + 1;
    rc.set(num);
}
/*--------------------------------------------------*
    replace_string mutates cell value passed as immutable arg
    - interior mutability
--------------------------------------------------*/
fn replace_string(rs:&Cell<String>) {
    // can't use get() because String is not copy
    let mut s = rs.replace("".to_string());
    s.push_str(" plus more");
    rs.set(s);
}
fn demo_cell() {
    print!("\n  -- Cell demo --");
    
    /*-- copy type --*/
    let cu = Cell::new(0usize);  // not declared mut
    cu.set(3);                   // copy in
    let u = cu.get();            // copy out
    print!("\n  cu.get() -> {}", u);
    increment_usize(&cu);
    print!("\n  increment_usize(cu) = {:?}", cu);

    /*-- move type --*/
    let s = "a string".to_string();
    let cs = Cell::new(s.clone());
    cs.set("another string".to_string());
    print!("\n  cs holds {:?}", cs.into_inner());
    // cs now invalid - been moved

    let cs = Cell::new(s);
    replace_string(&cs);
    print!("\n  replace_string(&cs) = {:?}", cs.into_inner());
    
    // Strings are not Copy so can't call cs.get() as below
    // pub fn Cell::get(&self) -> T
    // print!("\n  cs holds {:?}", cs.get());

    let c4 = Cell::new("the final String".to_string());
    let s4 = c4.replace("the really final string".to_string());
    print!("\n  s4 = {:?}", s4);
    print!("\n  c4 = {:?}", c4.into_inner());

    // only place mutability is declared in this demo
    let mut v = Cell::new(vec![1, 2, 3i32]);
    let r = v.get_mut();  // v must be mut to call get_mut()
    r.push(4);
    print!("\n  v holds {:?}", r);
}

/*--------------------------------------------------
    RefCell<T> holds t:T on stack
    - focus on providing references to held value
      - fn borrow(&self) -> Ref<'_, T>
      - fn borrow_mut(&self) -> RefMut<'_, T>
      - fn into_inner(self) -> T
      - fn replace(&self, t: T) -> T
      - fn take(&self) -> T
    - has dynamically checked borrow rules
      - borrow() panics if already mutably borrowed
      - borrow_mut(), replace(...) panic if already borrowed
--------------------------------------------------*
--------------------------------------------------*
    rc_increment_usize mutates cell value passed as immutable arg
    - interior mutability
--------------------------------------------------*/
fn rc_increment_usize(rc:&RefCell<usize>) {
    let mut rrc = rc.borrow_mut();
    *rrc += 1;
}
/*--------------------------------------------------*
    rc_replace_string mutates cell value passed as immutable arg
    - interior mutability
--------------------------------------------------*/
fn rc_replace_string(rs:&RefCell<String>) {
    // replace panics if rs is currently borrowed
    let mut rrs = rs.borrow_mut();
    rrs.push_str(" with addendum");
}
fn demo_refcell() {
    print!("\n\n  -- RefCell demo --");
    
    /*-- copy type --*/
    let rcu = RefCell::new(0usize);     // not declared mut
    let u = rcu.replace(3);             // copy in, copy out
    print!("\n  rcu.replace() -> {:?}", rcu);
    print!("\n  u = {}", u);
    // -- use fn rc_incu, above --
    rc_increment_usize(&rcu);
    print!("\n  rc_increment_usize(&rcu) = {:?}", rcu);

    /*-- move type --*/
    let rcs = RefCell::new("a string".to_string());
    print!("\n  rcs.borrow() -> {:?}", rcs.borrow());
    let s = rcs.replace("another string".to_string());
    let rrcs = rcs.borrow();
    print!("\n  rcs.borrow() -> {:?}", rrcs);
    // -- use fn rc_rpls, above --
    let rcs2 = RefCell::new("a string".to_string());
    rc_replace_string(&rcs2);     // uses interior mutability
    print!("\n  rc_replace_string(&rs) -> {:?}", &rcs2);

    let rcs2 = RefCell::new("the final String".to_string());
    let mut r = rcs2.borrow_mut();  // rcs2 must be mut to call borrow_mut()
    r.push('!');
    print!("\n  rcs2 holds {:?}", r);
    
    print!("\n\n  -- RefCell borrowing --");
    let c = RefCell::new("a string".to_string());
    print!("\n  c = {:?}", c);
    let cref: Ref<String> = c.borrow();
    // remove the *, below, to see auto deref, e.g., no change
    print!("\n  *cref = {:?}", *cref);
    // comment out the drop to see run-time panic
    drop(cref);  // removes outstanding borrow
    let mut crefmut: RefMut<String> = c.borrow_mut();
    crefmut.push_str(" with more");
    print!("\n  *crefmut = {:?}", *crefmut);
}

fn main() {
    demo_cell();
    demo_refcell();
}