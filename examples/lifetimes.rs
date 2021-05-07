#![allow(dead_code)]
#![allow(unused_variables)]

use std::result::*;

fn demo_result<'a>(p: bool) -> Result<&'a str, &'a str> {
    print!("\n  value of input predicate is {}", p);
    if p {
        Ok("it's ok")
    } 
    else {
        Err("not ok")
    }
}

fn demo_option<'a>(p:bool) -> Option<&'a str> {
    print!("\n  value of input predicate is {}", p);
    if p {
        Some("something just for you!")
    }
    else {
        None
    }
}

fn title(msg:&str) {
    print!("\n  -- {} --\n", msg);
}

fn main() {
    // use display::{*};

  title("demo Result");
  print!("\n-- using match");

  let r = demo_result(true);
  match r {
      Ok(rslt) => print!("\n  result is {}", rslt),
      Err(rslt) => print!("\n  result is {}", rslt)
  }
  let r = demo_result(false);
  match r {
      Ok(rslt) => print!("\n  result is {}", rslt),
      Err(rslt) => print!("\n  result is {}", rslt)
  }
  print!("\n\n-- using expect");

  let r = demo_result(true)
    .expect("predicate was false");
  print!("\n    result is {}", r);
  /////////////////////////////////////////////
  // uncomment to see panic
  // let _r = demo_result(false)
  //   .expect("predicate was false");
  println!();

  title("demo Option");
  print!("\n--using match");

  let r = demo_option(true);
  match r {
      Some(rslt) => print!("\n    {}", rslt),
      None => print!("\n    sorry, nothing here")
  }
  let r = demo_option(false);
  match r {
      Some(rslt) => print!("\n    {}", rslt),
      None => print!("\n    sorry, nothing here")
  }
  print!("\n\n--using unwrap");

  let r = demo_option(true).unwrap();
  print!("\n    {}", r);
  /////////////////////////////////////////////
  // uncomment to see panic
  // let _r = demo_option(false).unwrap();

  print!("\n\n  That's all folks!\n\n");

}