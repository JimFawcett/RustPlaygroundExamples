fn demo_basic() {

    print!("\n  -- demo_basic --");
    
    #[derive(Debug)]
    pub struct Demo { value: i32 }
    impl Demo {
        pub fn new() -> Demo {
            Demo {
                value:0
            }
        }
        pub fn set_value(&mut self, v: i32) {
            self.value = v;
        }
        pub fn get_value(&self) -> i32 {
            self.value
        }
    }
  
    let mut d = Demo::new();
    d.set_value(42);
    print!("\n  d = {:?}", d);
    print!("\n  d.value = {:?}", d.get_value());
  }
  
  fn main() {
      demo_basic();
  }