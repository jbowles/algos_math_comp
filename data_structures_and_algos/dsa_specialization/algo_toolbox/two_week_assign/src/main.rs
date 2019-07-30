extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::ops::Rem;

fn main() {
    // let a: i128 = 7_389_263_219_321;
    // let b: i128 = 748_963_438_048_302_949_037_248_932;
    // println!("{}", gcd(a, b));
    // println!("{}", fib(200));
    // println!("max u64 {}", std::u64::MAX);
    // println!("max f64 {}", std::f64::MAX);
    // println!("max i128 {}", std::i128::MAX);
    // println!("max u128 {}", std::u128::MAX);
    // println!("176: {}", fib_u128(176));
    // println!("176: {}", fib_f64(176));
    // println!("176: {}", fib(176));
    // println!("200: {}", fib(200));
    // println!("1000: {}", fib(1000));
    // let fib_128_176 = fib_u128(176);
    // println!("fib_128_176: {}", fib_128_176);
    // println!("fib_128_176 mod 10: {}", fib_128_176.rem(10));
    // let fib_nth_dynamic_176 = fib_nth_dynamic(176);
    // println!("fib_nth_176: {}", fib_nth_dynamic_176);
    // println!("fib_nth_176: {}", fib_nth(176));
    // println!("fib_176:     {}", fib(176));

    //  //println!("{}", fib_huge_slow(2816213588, 239)); //151
    //println!("{}", fib_huge_slow(10, 3)); //1
    //println!("{}", fib_huge_slow(239, 1000)); //161
    //println!("{}", pisano_period_brute(3)); //8
    //println!("pi(3)=={}", pisano_period_brute(3)); //8
    //println!("pi(2)=={}", pisano_period_brute(2)); //3
    /*
    println!(
        "{}",
        fib_huge_slow(2816213588.rem(pisano_period_brute(239)), 239)
    ); //151
    */

    // println!("pi(3)=={}", pisano_period_brute(3)); //8
    // println!("pi(8)=={}", pisano_period_brute(8)); //12
    // println!("pi(10)=={}", pisano_period_brute(10)); //60
    // println!("pi(12)=={}", pisano_period_brute(12)); //24

    // let max: usize = 3;
    // let mut f: Vec<u128> = vec![0; max + 1];
    // println!("3==4? {}", fib_sld(&mut f, max));
    // println!("{}", sum_fib(3)); //4
    // println!("{}", sum_fib(4)); //7
    // println!("{}", sum_fib(5)); //12
    // println!("{}", sum_fib(6)); //20
    // println!("sum 100: {}", sum_fib(100)); //927372692193078999175
    // println!("sum 100: {}", sum_fib(200)); //927372692193078999175
    // println!("last digit of sum 10000: {}", sum_fib_last_digit(1000)); //5
    // println!("{}", std::u128::MAX);
    // println!("last digit of sum 100: {}", sum_fib_last_digit(200)); //5
    // println!("{}", fib_range_sum_rem(2, 5)); // last digit -> 1, (total == 11)
    // println!("{}", fib_range_sum_rem(4, 8)); // last digit -> 0, (total == 50)
    // println!("{}", fib_range_sum_rem(3, 7)); // last digit -> 1, (total == 31)
    // println!("{}", fib_range_sum_rem(10, 10)); // last digit -> 5, (total == 55)

    //734544867157817600000000000000000000000000
    //734544867157818093234908902110449296423262
    // println!("{:.1}", fib_range_sum(10, 200)); // last digit -> 2, (total ...)
    // println!("{}", fib_range_sum_rem(10, 200)); // last digit -> 2, (total ...)
    let one: u128 = 9_283_502_900_904_314_433_175_552;
    let two: u128 = 2_400_317_512_179_828_157_054_976;
    println!("{}", one + two);
    println!("{}", one * two);
}

#[allow(dead_code)]
fn fib_range(n: i32) -> f64 {
    let sqrt_five: f64 = 5.0_f64.sqrt();
    let phi: f64 = (1.0 + sqrt_five) / 2.0;
    (phi.powi(n) / sqrt_five).round()
}

#[allow(dead_code)]
fn fib_range_sum(l: i32, r: i32) -> f64 {
    (fib_range(r + 2) - fib_range(l + 1)) //.rem(10.0)
}

#[allow(dead_code)]
fn fib_range_sum_rem(l: i32, r: i32) -> f64 {
    (fib_range(r + 2) - fib_range(l + 1)).rem(10.0)
}

// sum fib last digit
#[allow(dead_code)]
fn sum_fib(n: u128) -> u128 {
    let mut f: Vec<u128> = vec![0; (n + 3) as usize];
    fib_sl(&mut f, n + 2) - 1
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
    let k: u128 = if (n.rem(2)) > 0 {
        ((n + 1) as u128) / 2
    } else {
        (n as u128) / 2
    };

    fvec[n as usize] = if (n.rem(2)) > 0 {
        let first = fib_sl(fvec, k).pow(2);
        let second = fib_sl(fvec, k - 1).pow(2);
        println!("first: {}, second: {}", first, second);
        first + second
    // (fib_sl(fvec, k) * fib_sl(fvec, k) + fib_sl(fvec, k - 1) * fib_sl(fvec, k - 1))
    } else {
        let third = fib_sl(fvec, k - 1);
        let fourth = fib_sl(fvec, k);
        let fifth = 2 * (third + fourth);
        let sixth = fib_sl(fvec, k);
        println!(
            "third: {}, fourth: {}, fifth: {}, sixth: {}",
            third, fourth, fifth, sixth
        );
        fifth * sixth
    };

    fvec[n as usize]
}

#[allow(dead_code)]
/*
If m and n are coprime, then π(mn) is the least common multiple of π(m) and π(n), by the Chinese remainder theorem. For example, π(3) = 8 and π(4) = 6 imply π(12) = 24. Thus the study of Pisano periods may be reduced to that of Pisano periods of prime powers q = p^k, for k ≥ 1.

use prime factorization for pisano period....


#[allow(dead_code)]
fn pisano_period(n: u128, m: u128) -> u128 {
    lcm(n, m)
}
*/
fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

#[allow(dead_code)]
fn pisano_period_brute(m: usize) -> usize {
    let mut a: usize = 0;
    let mut b: usize = 1;
    for i in 0..(m * m) {
        let c = (a + b).rem(m);
        a = b;
        b = c;
        if a == 0 && b == 1 {
            return i + 1;
        }
    }
    0
}

#[allow(dead_code)]
fn pisano_period_brute_while(m: usize) -> usize {
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

#[allow(dead_code)]
fn fib_huge_slow(n: usize, m: u128) -> u128 {
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;
    for _ in 0..n {
        let f2 = (f0 + f1).rem(m);
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0
}

//1,1,2,3,5,8,13,24,....
#[allow(dead_code)]
fn fib_nth(n: usize) -> u128 {
    let mut f0: u128 = 0;
    let mut f1: u128 = 1;
    for _ in 0..n {
        let f2 = (f0 + f1).rem(10);
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0
}

//1,1,2,3,5,8,13,24,....
#[allow(dead_code)]
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        // 1 = 0 + 1
        // 2 = 1 + 1
        // 3  = 1 + 2
        // 5 = 2 + 3
        let f2 = f0 + &f1;
        // 1[previous destination] = 1[destination] <- 1[source]
        // 1[previous destination] = 1[destination] <- 2[source]
        // 2[previous destination] = 2[destination] <- 3[source]
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0
}

//1,1,2,3,5,8,13,24,....
#[allow(dead_code)]
fn fib_nth_dynamic(n: u32) -> u128 {
    //we know 0,1
    if n <= 1 {
        return u128::from(n);
    }
    //much faster than recursive procedure
    //fib((n-1) + (n-2)) n^2 ??
    //allocate vector of numbers to n
    let mut v: Vec<u128> = (0..=u128::from(n)).collect();
    for i in 2..=n as usize {
        v[i] = ((v[i - 1]) + (v[i - 2])).rem(10);
    }
    v[n as usize]
}

//1,1,2,3,5,8,13,24,....
#[allow(dead_code)]
fn fib_u128(n: u32) -> u128 {
    //we know 0,1
    if n <= 1 {
        return u128::from(n);
    }
    //much faster than recursive procedure
    //fib((n-1) + (n-2)) n^2 ??
    //allocate vector of numbers to n
    let mut v: Vec<u128> = (0..=u128::from(n)).collect();
    for i in 2..=n as usize {
        v[i] = (v[i - 1]) + (v[i - 2]);
    }
    v[n as usize]
}

//greatest common divisor 12,4 == 4
#[allow(dead_code)]
fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    gcd(b, a.rem(b))
}

/*
//1,1,2,3,5,8,13,24,....
#[allow(dead_code)]
fn fib_f64(n: u32) -> f64 {
    //we know 0,1
    if n <= 1 {
        return f64::from(n);
    }

    let mut v: Vec<f64> = Vec::with_capacity(n as usize);
    for i in 0..=n {
        v.push(f64::from(i));
    }

    for i in 2..=n as usize {
        v[i] = (v[i - 1]) + (v[i - 2]);
    }
    v[n as usize]
}
*/
