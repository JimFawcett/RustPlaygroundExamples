// code modified from example by Matt Brubeck
// https://users.rust-lang.org/t/rust-is-very-aggressive-towards-beginners/57125

struct MyStruct {
    items: Vec<String>
}

impl MyStruct {
    fn strings(&self) -> MyIterator<'_> {
        MyIterator { 
            index: 0, items: &self.items 
        }
    }
}

struct MyIterator<'a> {
    index: usize,
    items: &'a [String],
}

impl<'a> Iterator for MyIterator<'a> {
    type Item = &'a String;
    
    fn next(&mut self) 
        -> Option<&'a String> {
        let item = self.items.get(self.index);
        self.index += 1;
        item
    }
}

fn main() {
    let s1 = "one".to_string();
    let s2 = "two".to_string();
    let s3 = "three".to_string();
    let ms = MyStruct { items: vec![s1, s2, s3] };
    
    for item in ms.strings() {
        print!(" {:?}", item);
    }
}