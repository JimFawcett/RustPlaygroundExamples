// ThreadResult
#![allow(clippy::mutex_atomic)]

use std::sync::*;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct ThreadResult<T> {
    pub result: Mutex<T>,
    cv: Condvar,
    ready: Mutex<bool>
}

impl<T: Debug + Default + Clone> ThreadResult<T> {
    pub fn new() -> Self {
        Self {
            result: Mutex::new(T::default()),
            cv: Condvar::new(),
            ready: Mutex::new(false),
        }
    }
    /*--------------------------------------------
      Unwrapping is appropriate here.  The 
      operation fails if the Mutex becomes
      poisoned, due to panic on a thread
      holding the lock.  But then you can't
      do much except quit, which the unwrap
      does for you.
    --------------------------------------------*/
    pub fn set(&self, t:T) {
        let mut lr = self.ready.lock().unwrap();
        *lr = true;
        let mut lrslt = self.result.lock().unwrap();
        *lrslt = t;
        self.cv.notify_one();
    }
    pub fn get(&self) -> T {
        let mut rdy = self.ready.lock().unwrap();
        while !*rdy {
            rdy = self.cv.wait(rdy).unwrap();
        }
        let rslt = self.result.lock().unwrap();
        rslt.clone()
    }
    pub fn ready(&self) -> bool {
        *self.ready.lock().unwrap()
    }
}
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn test() {
    let thrd_rslt = Arc::new(ThreadResult::<i32>::new());
    let thrd_rslt1 = Arc::clone(&thrd_rslt);
    let thrd_rslt2 = Arc::clone(&thrd_rslt);

    let cls = move || {
        let dur = Duration::from_millis(1000u64);
        thread::sleep(dur);
        thrd_rslt2.set(42);
    };
    let handle = thread::spawn(cls);

    let dur = Duration::from_millis(200);
    loop {
        if !thrd_rslt1.ready() {
            print!("\n  main waiting");
            thread::sleep(dur);
        }
        else {
            print!("\n  thread result is {}", thrd_rslt.get());
            break;
        }
    }
    let _ = handle.join();
}
fn main() {
    test();
}
