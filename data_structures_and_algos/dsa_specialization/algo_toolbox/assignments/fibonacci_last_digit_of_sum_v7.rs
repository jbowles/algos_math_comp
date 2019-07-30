use std::ops::Rem;
// CORRECT
fn main() {
    /*
        for i in 0..=10_000 {
            println!("fib sum({}): {}", i, fib(i));
        }
    */

    /*
     * conditions for passing:
     * 3 => 4
     * 100 => 5
     * 239 => 0
     * 614162383528 => 9
    //let vv: Vec<usize> = vec![3, 100, 239];
    //let vv: Vec<usize> = vec![3, 100, 239, 614162383528];
    let vv: Vec<usize> = vec![3, 100, 239, 614162383528, 99999999999999999];
    for i in vv {
        println!("fib sum({}): {}", i, fib(i));
    }
    */

    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: usize = args.next().unwrap().parse().unwrap();
    println!("{}", fib(a));
}

// the nth Fibonacci number can be calculated as F(n+2)-1
// `(n.rem(60) + 2)` is the `F(n+2)` and `f0-1` is the final subtraction `-1`
// Doing modulo 60 for our index helps us leapfrog indexes
// by exploiting periodicity patterns in the fibonacci sequence.
// Without the mod(60) the additions would take a very long time,
// noticeable mostly with largest nth numbers [614162383528, 99999999999999999]
fn fib(n: usize) -> usize {
    let mut f0: usize = 0;
    let mut f1: usize = 1;
    for _ in 0..(n.rem(60) + 2) {
        let f2 = f0.rem(10) + f1.rem(10);
        f0 = std::mem::replace(&mut f1, f2);
    }
    (f0 - 1).rem(10)
}

/*
#[allow(dead_code)]
fn fib_slow(n: usize) -> usize {
    let mut f0: usize = 0;
    let mut f1: usize = 1;
    //let r = n.rem(60);
    for _ in 0..(n + 2) {
        let f2 = f0.rem(10) + f1.rem(10);
        f0 = std::mem::replace(&mut f1, f2);
    }
    (f0 - 1).rem(10)
}
*/
