#![allow(clippy::print_literal)]

use core::fmt::{ Debug };

#[derive(Debug)]
pub struct TestLifeCycle {
    count: u32,
}
/*-- any type implementing drop cannot be Copy --*/
impl Drop for TestLifeCycle {
    fn drop(&mut self) {
        print!("\n  TestLifeCycle instance {} dropped", self.count);
    }
}
/*-- any type can be Clone --*/
impl Clone for TestLifeCycle {
    fn clone(&self) -> TestLifeCycle {
        print!("\n  TestLifeCycle instance {} cloned", self.count);
        TestLifeCycle { 
            count: self.count, 
        }
    }
}
/*-- default constructor --*/
impl TestLifeCycle {
    pub fn new() -> TestLifeCycle {
        print!("\n  creating instance {} of TestLifeCycle", 0);
        TestLifeCycle { 
            count: 0, 
        }
    }
    /*-- parameterized constructor --*/
    pub fn param_new(cnt: u32) -> TestLifeCycle {
        print!("\n  creating instance {} of TestLifeCycle", cnt);
        TestLifeCycle { 
            count: cnt,
        }
    }
    /*-- value getter --*/
    pub fn get_value(&self) -> u32 {
            self.count
    }
    
    pub fn set_value(&mut self, v:u32) {
        self.count = v;
    } 
}

/*-- same as print!("\n  sp = {:?}", sp) --*/
fn show<T:Debug>(name:&str, t:&T) {
    print!("\n  {} = {:?}", name, t);
}

fn main() {

    let putline = || print!("\n");

    type TLC = TestLifeCycle;

    print!("\n  -- create Box pointer to TLC on heap --");
    let sp = Box::new(TLC::new());
    show("sp", &sp);
    putline();

    print!("\n  -- access member of TLC using Box --");
    let v = sp.get_value();
    show("v ", &v);
    putline();

    print!("\n  -- clone Box pointer to TLC on heap --");
    let mut spc = sp.clone();
    show("spc", &spc);
    putline();

    print!("\n  -- mutate TLC through Box --");
    spc.set_value(42);
    show("spc", &spc);
    show("v ", &spc.get_value());
    show("sp", &sp);
    putline();

    print!("\n  -- create reference to Box pointer --");
    let rspc = &mut spc;
    rspc.set_value(21);
    show("rspc", &rspc);    // holds new value
    show("spc", &spc);      // holds new value too
    putline();

    print!("\n  -- move contents of Box back to stack with deref --");
    let tlc = *sp;  // move
    show("tlc", &tlc);

    // statement below won't compile since sp has been moved
    // show("sp", &sp);

    println!("\n\n  That's all Folks!");

}  /*-- drop Boxed item --*/