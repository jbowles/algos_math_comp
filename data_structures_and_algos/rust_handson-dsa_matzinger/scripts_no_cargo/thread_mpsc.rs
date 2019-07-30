use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

fn main(){
    channels();
    channels_two();
}

//spawn 10 threads, each send a number into the channel, collect into vector.
fn channels() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..N).map(|i| {
        println!("handles...");
        let _tx = tx.clone();
        thread::spawn(move || {
            //dont use result
            println!("spawn...");
            let _ = _tx.send(i).unwrap();
        })
    });
    //close thread channels
    for h in handles {
        h.join().unwrap();
    }

    // receive
    let nums: Vec<i32> = (0..N).map(|_| rx.recv().unwrap()).collect();
    println!("FINAL: {:?}", nums)
}


//spawn 10 threads, each send a number into the channel, create mut vector, loop through N, print
//n, push n into vec
fn channels_two() {
    const N: i32 = 10;
    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel();
    let handles = (0..N).map(|i| {
        let _tx = tx.clone();
        thread::spawn(move || {
            //dont use result
            let _ = _tx.send(i).unwrap();
        })
    });
    //close thread channels
    for h in handles {
        h.join().unwrap();
    }

    // receive
    let mut nums = vec![];
    for _ in 0..N {
        let n = rx.recv().unwrap();
        println!("Hello n: {:?}", n);
        nums.push(n)
    }
    println!("FINAL_TWO: {:?}", nums)
}
