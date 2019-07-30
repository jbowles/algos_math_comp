extern crate num_bigint;
extern crate num_traits;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};
use std::ops::Rem;

fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: usize = args.next().unwrap().parse().unwrap();
    println!("{}", sum_fib_last_digit(a));
}

fn sum_fib_last_digit(n: usize) -> BigUint {
    let mut f: Vec<BigUint> = vec![Zero::zero(); (n + 3) as usize];
    let f1: BigUint = One::one();
    let f10: BigUint = 10_i32.to_biguint().unwrap();
    let res = fib_sl(&mut f, n + 2) - f1;
    //println!("BigUint: {}", res);
    res.rem(f10)
}

fn fib_sl(fvec: &mut Vec<BigUint>, n: usize) -> BigUint {
    let f0: BigUint = Zero::zero();
    let f1: BigUint = One::one();
    let f2: BigUint = &f1 + &f1;
    if n == 0 {
        return f0;
    } else if n == 1 || n == 2 {
        fvec[n] = f1;
        return fvec[n].clone();
    }
    if fvec[n] > f0 {
        return fvec[n].clone();
    }
    let k: usize = if (n.rem(2)) > 0 { (n + 1) / 2 } else { n / 2 };

    fvec[n as usize] = if (n.rem(2)) > 0 {
        (fib_sl(fvec, k) * fib_sl(fvec, k) + fib_sl(fvec, k - 1) * fib_sl(fvec, k - 1))
    } else {
        (f2 * fib_sl(fvec, k - 1) + fib_sl(fvec, k)) * fib_sl(fvec, k)
    };

    fvec[n].clone()
}
