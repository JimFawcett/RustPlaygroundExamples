// Basic Threads
// sharing std::output and AtomicUsize - both thread safe

use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

static COUNT: AtomicUsize = AtomicUsize::new(0usize);

fn test() {
    // child thread processing
    let dur = Duration::from_millis(2);
    let handle = thread::spawn(
        move || {
            for _i in 0..10 {
                print!("\n  child thread printing");
                let _ = COUNT.fetch_add(1, Ordering::SeqCst);
                thread::sleep(dur);
            }
        }
    );
    // main thread processing
    let dur = Duration::from_millis(3);
    for _i in 0..10 {
        print!("\n   main thread printing");
        let _ = COUNT.fetch_add(1, Ordering::SeqCst);
        thread::sleep(dur);
    }
    // wait for thread to complete
    let _ = handle.join();
    print!("\n\n  number of prints = {:?}", COUNT);
}

fn main() {
    print!("\n  -- Basic Threads --\n");
    test();
    print!("\n\n  That's all Folks!\n\n");
}