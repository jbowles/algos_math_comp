use std::{thread, time};

fn main(){
    thread_simple();
    thread_move_var();
}


fn thread_move_var() {
    // move in pipes ||
    // to pass in variable
    for i in 0..25 {
        let millis = time::Duration::from_millis(10+i);
        let handle = thread::spawn(move || {
            thread::sleep(millis);
            println!("thread: {}", i);
        });
        handle.join().unwrap();
    };
}

fn thread_simple() {
    // pipes || is the space where params go
    // like function signature params.
    // variables can move from outer to inner scope
    let handle = thread::spawn(|| {
        println!("hello from thread");
    });
    handle.join().unwrap();
}
