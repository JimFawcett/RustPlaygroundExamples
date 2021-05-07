// derived and defined clones

#![allow(dead_code)]
use std::fmt::Debug;

/*-- derived default and clone --*/
#[derive(Debug, Default, Clone)]
pub struct MyClonable {
    name: String
}
impl MyClonable {
    pub fn new() -> MyClonable {
        MyClonable { name:String::new() }
    }
    pub fn get_name(&self) -> &str {
        &self.name.as_str()
    }
    pub fn set_name(&mut self, name:&str) {
        self.name = name.to_string();
    }
}

/*-- defined default and clone --*/
#[derive(Debug)]
pub struct YourClonable<T> 
    where T: Clone + Debug + Default
{
    arg : T
}
impl<T> YourClonable<T>
    where T: Clone + Debug + Default {
    pub fn new() -> YourClonable<T> {
        YourClonable {
            arg: T::default()
        }
    }
    pub fn set_arg(&mut self, arg:&T) {
        self.arg = arg.clone();
    }
    pub fn get_arg(&self) -> T {
        self.arg.clone()
    }
}
impl<T> Default for YourClonable<T> 
    where T: Clone + Debug + Default {
        fn default() -> Self {
            Self::new()
        }
    }
impl<T> Clone for YourClonable<T> 
    where T: Clone + Debug + Default {
    fn clone(&self) -> YourClonable<T> {
        YourClonable {
            arg: self.arg.clone()
        }
    }
}

fn main() {
    let mut mc = MyClonable::default();
    mc.set_name("Fred");
    let fred = mc.get_name();
    print!("\n  orig:  {:?}", fred);
    let mcc = mc.clone();
    print!("\n  clone: {:?}", mcc);
    
    let mut yc = YourClonable::<i32>::new();
    print!("\n  orig:  {:?}", yc);
    yc.set_arg(&42);
    print!("\n  orig:  {:?}", yc);
    let ycc = yc.clone();
    print!("\n  clone: {:?}", ycc);
    print!("\n  orig:  {:?}", yc);
}