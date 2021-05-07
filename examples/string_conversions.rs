#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::fmt::*;
use std::fmt::Display;

fn show_type<T: Debug + ?Sized>(t:&T) {
    print!("\n  {:?}", t);
    print!("\n  type is: {:?}\n", std::any::type_name::<T>());
}
fn main() {
  if let Ok(curr_dir) = std::env::current_dir() {
    print!("\n  curr dir: {:?}\n", curr_dir);
    show_type(&curr_dir);
  }
  /*-- create Path --*/
  let path = Path::new("./some_dir");
  show_type(path);
  /*-- convert to PathBuf --*/
  let pathbuf = path.to_path_buf();
  show_type(&pathbuf);
  /*-- convert Path to &str --*/
  if let Some(pathstr) = path.to_str() {
    show_type(&pathstr);
  }
  /*-- convert PathBuf to String --*/
  let mut pathbufstring = "".to_string();
  if let Some(pstr) = pathbuf.to_str() {
      pathbufstring = pstr.to_string();
      show_type(&pathbufstring);
  }
  /*-- convert String to PathBuf --*/
  let pathbuf = PathBuf::from(pathbufstring);
  show_type(&pathbuf);
  /*-- convert PathBuf to Path --*/
  let path = pathbuf.as_path();
  show_type(path);
}