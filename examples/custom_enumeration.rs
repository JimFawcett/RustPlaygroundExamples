#![allow(dead_code)]
#![allow(unused_variables)]
use std::fmt::*;


/*-----------------------------------------------------
   The generic parameter T represents some value 
   associated with events, perhaps a name String 
   or id number.
*/
#[derive(Debug, Clone)]
enum Event<T> { 
    Normal(T), Warning(T), Critical(T), None 
}
impl<T> Event<T> {
    fn unwrap(&self) -> &T {
        if let Event::Normal(ev) = self{ ev } 
        else if let Event::Warning(ev) = self{ev}
        else if let Event::Critical(ev) = self{ev}
        else { panic!() }
    }
}

use Event::*;

fn main() {
    /*-- numbers are event ids --*/
    let e1: Event<u8> = 
        Event::<u8>::Normal(1);
    let e2 = Warning(2);
    let e3 = Critical(3);
    let e4: Event<u8> = None;
    
    /*-- match works like switch stmt on steriods --*/

    match e3 {
        Normal(ev) => 
            print!("\n  event {} is Normal", ev),
        Warning(ev) => 
            print!("\n  event {} is Warning", ev),
        Critical(ev) => 
            print!("\n  event {} is Critical!", ev),
        None => print!("\n  no events occurred"),
    }
    
    /*-------------------------------------------------
      without the clone() operation below e2 would 
      move into e and become invalid for future 
      operations.
    */
    #[allow(clippy::redundant_clone)]
    let e = e2.clone();
    
    /*-------------------------------------------------  
      if let statements use "=" as match operator
    */
    
      if let Warning(ev) = e {
        print!("\n  event {} is Warning", ev);
    }
    
    /*-- next stmt panics if event is NoEvent type --*/

    let v = e3.unwrap();
    print!("\n  inner value of {:?} is {}", e3, v);

    /*-- will panic if you unwrap() e4 --*/
} 
