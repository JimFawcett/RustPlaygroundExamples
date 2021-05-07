#![allow(dead_code)]

fn demo_point() {

    print!("\n  -- demo_point --");

    use std::fmt::*;
    // Traits like Debug and Default are often called bounds
    // because they bound the set of types that are allowed
    // for the generic parameter T
    #[derive(Debug, Default)]
    pub struct Point<T> 
    where T:Default + Debug {
        x:T, y:T, z:T
    }
    impl<T> Point<T> where T:Default + Debug + Clone {
        pub fn new() -> Point<T> {
            Point {
                x:std::default::Default::default(), 
                y:std::default::Default::default(), 
                z:std::default::Default::default() 
            }
        }
        pub fn default() -> Point<T> {
            Point::new()
        }
        pub fn get_coordinates(&self) -> [T; 3] {
            [self.x.clone(), self.y.clone(), self.z.clone()]
        }
        pub fn set_coordinates(&mut self, coor: &[T; 3]) {
            self.x = coor[0].clone();
            self.y = coor[1].clone();
            self.z = coor[2].clone();
        } 
    }
    let mut pt = Point::new();
    pt.set_coordinates(&[1, 2, 3]);
    print!("\n  pt = {:?}", &pt);
    print!("\n  coordinates are: {:?}", &pt.get_coordinates());
    pt.set_coordinates(&[3, 2, 1]);
    print!("\n  pt = {:?}", pt);
    print!("\n  coordinates are: {:?}", pt.get_coordinates());
}

fn main() {
    demo_point();
}