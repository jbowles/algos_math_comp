use std::ops::Rem;
//rustc -o fibo_last_digit_rs fibonacci_last_digit.rs
//// NOT CORRECT
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let arg1: usize = args.next().unwrap().parse().unwrap();
    println!("{}", fib_last_digit(arg1));
}

fn fib_last_digit(n: usize) -> u128 {
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;
    for _ in 0..n {
        let f2 = (f0 + f1).rem(10);
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0
}
