use std::ops::Rem;
// CORRECT
fn main() {
    /*
    for i in 0..=10_000 {
        println!("{} == {}", i, sum(i));
    }
    */

    let vv: Vec<(usize, usize)> = vec![
        (1, 1), //1=1*1
        (2, 2), //2=1*2
        (3, 6), //6=2*3
        (4, 5), //15=3*5
        (5, 0), //40=5*8
        (6, 4), //104=8*13
        (7, 3), //273
        (47, 8),
        (57, 8),
        (73, 1),
        (239, 0),
        (1234567890, 0),
    ];
    for t in vv {
        println!("{} == {} ?= {}", t.0, sum(t.0), t.1);
        assert_eq!(t.1, sum(t.0));
    }

    /*
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: usize = args.next().unwrap().parse().unwrap();
    println!("{}", sum(a));
    */
}

fn sum(n: usize) -> usize {
    if n <= 2 {
        return n;
    }
    (fib(n) * fib(n + 1)).rem(10)
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
    for _ in 2..(n.rem(60) + 2) {
        let f2 = f0.rem(10) + f1.rem(10);
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0.rem(10)
}
