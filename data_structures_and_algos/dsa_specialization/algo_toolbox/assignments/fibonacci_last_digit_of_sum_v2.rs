use std::ops::Rem;
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: i32 = args.next().unwrap().parse().unwrap();
    let sum_fib_last: f64 = fib(a);
    println!("{}", sum_fib_last.rem(10.0));
}

fn fib(n: i32) -> f64 {
    let sqrt_five: f64 = 5.0_f64.sqrt();
    let phi: f64 = (1.0 + sqrt_five) / 2.0;
    (phi.powi(n) / sqrt_five).round()
}
