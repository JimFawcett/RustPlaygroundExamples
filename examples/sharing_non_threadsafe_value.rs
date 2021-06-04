// threads sharing string

use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn test() {
    let a = "        main thread pushes \'0\'";
    let b = " first child thread pushes \'1\'";
    let c = "second child thread pushes \'2\'";
    print!("\n  {}", a);
    print!("\n  {}", b);
    print!("\n  {}\n", c);
    
    let s = String::new();
    // create a thread-safe pointer (Arc) to Mutex
    // guarded shared string
    let shared = Arc::new(Mutex::new(s));
    // create another pointer to same content
    let shared0 = Arc::clone(&shared);
    // main thread gets first edit
    if let Ok(mut temp) = shared0.lock() {
        temp.push('0');
    }
    // create child threads
    // for each child create a thread-safe pointer
    // to shared content
    
    let shared1 = Arc::clone(&shared);
    let dur = Duration::from_millis(2);  // faster
    let handle1 = thread::spawn(
        move || {
            for _i in 0..15 {
                // child edits shared string
                if let Ok(mut temp) = shared1.lock() {
                    temp.push('1');
                }
                thread::sleep(dur);
            }
        }
    );
    // create second child thread
    let shared2 = Arc::clone(&shared);
    let dur = Duration::from_millis(3);  // slower
    let handle2 = thread::spawn(
        move || {
            for _i in 0..15 {
                // child edits string
                if let Ok(mut temp) = shared2.lock() {
                    temp.push('2');
                }
                thread::sleep(dur);
            }
        }
    );
    let _ = handle1.join();
    let _ = handle2.join();
    
    // client appends last '0' and prints
    let mut s = String::new();
    if let Ok(mut temp) = shared0.lock() {
        temp.push('0');
        s = temp.to_string();
    }
    print!("\n  {}", s);
}

fn main() {
    print!("\n  -- Threads sharing String --\n");
    test();
    print!("\n\n  That's all Folks!\n\n");
}