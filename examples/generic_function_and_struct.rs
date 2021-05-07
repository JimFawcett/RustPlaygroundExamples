#![allow(dead_code)]
#![allow(unused_variables)]

/////////////////////////////////////////////////////
// generics_and_traits::main.rs - funct & struct   //
// - Copied from RustBites_Traits.html             //
/////////////////////////////////////////////////////

use std::any::type_name;
use std::fmt::*;

/*-----------------------------------------
  Generic function gt
  - underscore in name, _t, indicates it 
    will not be used in the function
  - the :Debug, below, is a trait constraint
    required by gf
  - if not satisfied, compilation fails
*/
fn gf<T: Debug>(_t:T) {
    let tn = type_name::<T>();
    print!("\n  t is type {:?}", tn);
}
/*-----------------------------------------
  Generic struct
  - #[derive(Debug)] requests compiler to
    implement the Debug trait for Point
*/
#[derive(Debug, Clone)]
struct Point<T> { x:T, y:T, z:T, }

/*-----------------------------------------
  First demnstration of generics and traits
*/
fn main() {

    #[allow(clippy::approx_constant)]
    gf(3.14159);
    let pt = Point { x:0, y:1, z:2 };
    gf(pt.clone());  // gf accepts by value
    print!("\n  pt: {:?}", pt);
    println!();
    
    let pt = Point { x:0.0, y:0.5, z:1.0 };
    gf(pt.clone());
    print!("\n  pt: {:?}", pt);
    
    println!("\n\n  That's all Folks!\n\n");
}
