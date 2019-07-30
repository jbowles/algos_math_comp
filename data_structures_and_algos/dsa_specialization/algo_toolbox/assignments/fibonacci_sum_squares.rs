use std::ops::Rem;
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: u128 = args.next().unwrap().parse().unwrap();
    println!("{}", sum_sqr_last_digit(a));
}

#[allow(dead_code)]
fn sum_sqr_last_digit(n: u128) -> u128 {
    let mut f: Vec<u128> = vec![0; (n + 3) as usize];
    (fib_sl(&mut f, n) * fib_sl(&mut f, n + 1)).rem(10)
}

#[allow(dead_code)]
fn fib_sl(fvec: &mut Vec<u128>, n: u128) -> u128 {
    if n == 0 {
        return 0;
    } else if n == 1 || n == 2 {
        fvec[n as usize] = 1;
        return fvec[n as usize];
    }
    if fvec[n as usize] > 0 {
        return fvec[n as usize];
    }
    let k: u128 = if (n.rem(2)) > 0 { (n + 1) / 2 } else { n / 2 };

    fvec[n as usize] = if (n.rem(2)) > 0 {
        (fib_sl(fvec, k) * fib_sl(fvec, k) + fib_sl(fvec, k - 1) * fib_sl(fvec, k - 1)).rem(10)
    } else {
        ((2 * fib_sl(fvec, k - 1) + fib_sl(fvec, k)) * fib_sl(fvec, k)).rem(10)
    };

    fvec[n as usize]
}
