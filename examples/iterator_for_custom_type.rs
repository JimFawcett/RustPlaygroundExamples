// code modified from example by Matt Brubeck
// https://users.rust-lang.org/t/rust-is-very-aggressive-towards-beginners/57125

struct MyStruct {
    items: Vec<String>
}

impl MyStruct {
    fn strings(&self) 
        -> impl Iterator<Item = &String> {
        self.items.iter()
    }
}

fn main() {
    let s1 = "1".to_string();
    let s2 = "2".to_string();
    let s3 = "3".to_string();
    let ms = MyStruct { items:vec![s1, s2, s3] };
    println!();
    for item in ms.strings() {
        print!(" {:?}", item);
    }
    println!();
}