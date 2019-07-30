use std::ops::Rem;
//rustc -o fibo_huge_mod fibonacci_hug_mode.rs
//// CORRECT
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let n: usize = args.next().unwrap().parse().unwrap();
    let m: usize = args.next().unwrap().parse().unwrap();

    println!("{}", fib_huge_slow(n.rem(pisano_period_brute(m)), m));
}

fn pisano_period_brute(m: usize) -> usize {
    let mut prev: usize = 1;
    let mut current: usize = 1;
    let mut result: usize = 1;
    while !(prev == 0 && current == 1) {
        let b = (prev + current).rem(m);
        prev = current;
        current = b;
        result += 1;
    }
    result
}

fn fib_huge_slow(n: usize, m: usize) -> usize {
    let mut f0: usize = 0;
    let mut f1: usize = 1;
    for _ in 0..n {
        let f2 = (f0 + f1).rem(m);
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0
}
