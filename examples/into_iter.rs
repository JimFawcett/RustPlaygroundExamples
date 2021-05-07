/*-- iter does not move collection, into_iter does --*/

fn type_is<T>(_rt: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    let v = vec![1, 2, 3, 4, 5];
    print!("\n  ");
    for item in v.iter() {
        print!("{} ", item);
    }
    
    // Code below builds because v was not
    // moved into iterator
    
    print!("\n  ");
    for item in v.into_iter() {
        print!("{} ", item);
    }
    
    // Code below fails to compile because v was
    // moved into iterator.
    
    // print!("\n  ");
    // for item in v.iter() {
    //     print!("{} ", item);
    // }
    
    let v = vec![
        "1".to_string(), 
        "2".to_string(), 
        "3".to_string(), 
        "4".to_string(), 
        "5".to_string()
    ];
    // iter() returns reference, does not consume collection
    let mut iter = v.iter();
    if let Some(e) = iter.next() {
        print!("\n  first elem: {} has type: {}", e, type_is(&e));
    }
    // into_iter() returns value, does concome colletion
    let mut iter = v.into_iter();
    if let Some(e) = iter.next() {
        print!("\n  first elem: {} has type: {}", e, type_is(&e));
    }
    // into_iter for array is depricated    
    // let mut iter = [1, 2, 3].into_iter();
    // if let Some(e) = iter.nth(0) {
    //     print!("\n  first elem: {} has type: {}", e, type_is(&e));
    // }
}