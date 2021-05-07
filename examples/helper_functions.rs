use std::fmt::Debug;
use core::mem::size_of;
/*-----------------------------------------------
   Pass by value moves the argument into the
   function's stack frame, so invalid after call
*/
fn pass_by_value_str(s:String) {  // move
  show_type(&s);   // def in display::lib.rs
  show_value(&s);  // def in display::lib.rs
}
/*-----------------------------------------------
   Pass by ref borrows the argument. Borrow
   moved into the function's stack frame.  
   Borrow ends at end of function call, so 
   param is valid after call
*/
fn pass_by_ref_str(rs:&String) {  // borrow
  show_type(&rs);   // def in display::lib.rs
  show_value(&rs);  // def in display::lib.rs
}
/*-----------------------------------------------
   Generic pass by value moves argument into the
   function's stack frame, so invalid after call
*/
fn pass_by_value<T>(t:T) where T:Debug {
  show_type(&t);
  show_value(&t);
}
/*-----------------------------------------------
   Generic pass by ref borrows argument into the
   function's stack frame.  Borrow ends at end
   of function stack frame, so param is valid 
   after call
*/
fn pass_by_ref<T>(rt:&T) where T:Debug {
  show_type(&rt);
  show_value(&rt);
}
/*-------------------------------------------------------------
  Helper functions
--------------------------------------------------------------- 
   show value
   - expects T to implement Debug
*/
pub fn show_value<T: Debug>(value: &T) {
    print!("\n  value: {:?}", value);
}
/*------------------------------------------------------------- 
   show type name
*/
pub fn show_type<T>(_value: &T) {
    let name = std::any::type_name::<T>();
    print!("\n  TypeId: {}, size: {}", name, size_of::<T>());
}
/*-------------------------------------------------------------
   Display underlined sub title on console
*/
pub fn sub_title(msg: &str) {
    print!("\n  {}", msg);
    let s = std::iter::repeat('-').take(msg.len() + 2).collect::<String>();
    print!("\n {}", s);
}
/*-------------------------------------------------------------
   Generic display message and value
   - no automatic newline
*/
pub fn show<T: Debug>(msg:&str, t:&T) {
    print!("{}{:?}", msg, t);
}
/*-----------------------------------------------
   Accepts either String or str
*/
fn shows<S: Into<String>>(s:S) {
  print!("{}",s.into());
}
/*------------------------------------------------------------- 
   show line with len hyphens
*/
pub fn separator(len:u8) {
    let mut s = String::new();
    for _i in 1..len+2 { s.push('-');}
    print!("\n {}",s);
}

fn main() {
  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass string by value");
  let mut s = "xyz".to_string();
  let s1 = s.clone();
  pass_by_value_str(s1);  // moves s1
  ////////////////////////////////////////////////
  // next statement fails to compile - s1 moved
  // pass_by_value(s1);
  pass_by_value(s.clone());
  /*-------------------------------------------*/
  separator(48);
  sub_title("Pass string by reference");
  pass_by_ref_str(&s);
  s.push('a');
  pass_by_ref(&s);  // borrows s
  s.push('b');
  show("\n  after pushing a and b, s = ",&s);
  /*-------------------------------------------*/
  separator(48);
  sub_title("function accepting either String or str");
  shows("\n  a literal string");
  shows("\n  a String".to_string());
}