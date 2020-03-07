use std::thread;
use std::sync::{Mutex, Arc};

fn main() {
    shared_state();
}

//ARC atomic reference counter
// thread safe equivalent of Rc::new(Refcell::new());
fn shared_state() {
    let v = Arc::new(Mutex::new(vec![]));
    let handles = (0..10).map(|i| {
        let numbers = Arc::clone(&v);
        thread::spawn(move || {
            let mut vctr = numbers.lock().unwrap();
            (*vctr).push(i);
        })
    });

    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", *v.lock().unwrap());
}
