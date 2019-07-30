// #![feature(test)]
// extern crate test;
use std::ops::Rem;

fn main() {
    //println!("naiveBEST: {}", naive_gcd(10999, 4786)); //1
    //println!("euclidBEST: {}", euclid_gcd(10999, 4786)); //1
    // println!("euclidBEST: {}", euclid_gcd(3_918_848, 1_653_264)); //61232
    // println!("naiveBEST: {}", naive_gcd(3_918_848, 1_653_264)); //61232
    println!("{}", euclid_gcd(357, 234));
    println!("{}", e_gcd(234, 357));
    let a: i128 = 13;
    let b: i128 = 33;
    // println!("{}", e_gcd(a.pow(34), b.pow(13)));
    println!("{} {}", a.pow(34), b.pow(13));
}

#[allow(dead_code)]
fn naive_gcd(a: i128, b: i128) -> i128 {
    let mut best: i128 = 0;
    for d in 1..(a + b) {
        // format(a, b, d);
        if (a.rem(d) == 0) && (b.rem(d) == 0) {
            best = d;
            // println!("best: {}", best);
        }
    }
    best
}

fn euclid_gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    let a_bar = a.rem(b);
    euclid_gcd(b, a_bar)
}

fn e_gcd(a: i128, b: i128) -> i128 {
    if b == 0 {
        return a;
    }
    euclid_gcd(b, a.rem(b))
}

#[allow(dead_code)]
fn format(a: i128, b: i128, d: i128) {
    println!(
        "d:{}, a:{}, b:{}.  ({}.rem({})={})  ({}.rem({})={}), ({}/{}={}), ({}/{}={})",
        d,
        a,
        b,
        a,
        d,
        a.rem(d),
        b,
        d,
        b.rem(d),
        a,
        d,
        a / d,
        b,
        d,
        b / d
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn naive_gcd_small() {
        assert_eq!(2, naive_gcd(10, 4));
        assert_eq!(4, naive_gcd(12, 4));
        assert_eq!(1, naive_gcd(13, 4));
    }
    #[test]
    fn euclid_gcd_small() {
        assert_eq!(2, euclid_gcd(10, 4));
        assert_eq!(4, euclid_gcd(12, 4));
        assert_eq!(1, euclid_gcd(13, 4));
    }
    #[test]
    fn naive_1099_4786() {
        assert_eq!(1, naive_gcd(10999, 4786));
    }
    #[test]
    fn euclid_1099_4786() {
        assert_eq!(1, euclid_gcd(10999, 4786));
    }
    #[test]
    fn naive_3918848_1653264() {
        assert_eq!(61232, naive_gcd(3_918_848, 1_653_264));
    }
    #[test]
    fn euclid_3918848_1653264() {
        assert_eq!(61232, euclid_gcd(3_918_848, 1_653_264));
    }

    /*
    use test::Bencher;
    //cargo +nightly bench
    #[bench]
    fn bench_10_4_naive_gcd(b: &mut Bencher) {
        b.iter(|| naive_gcd(test::black_box(10), test::black_box(4)))
    }
    #[bench]
    fn bench_10_4_euclid_gcd(b: &mut Bencher) {
        b.iter(|| euclid_gcd(test::black_box(10), test::black_box(4)))
    }
    #[bench]
    fn bench_2022_4044_naive_gcd(b: &mut Bencher) {
        b.iter(|| naive_gcd(test::black_box(2022), test::black_box(4044)))
    }
    #[bench]
    fn bench_2022_4044_euclid_gcd(b: &mut Bencher) {
        b.iter(|| euclid_gcd(test::black_box(2022), test::black_box(4044)))
    }
    #[bench]
    fn bench_10999_4786_naive_gcd(b: &mut Bencher) {
        b.iter(|| naive_gcd(test::black_box(10999), test::black_box(4786)))
    }
    #[bench]
    fn bench_10999_4786_euclid_gcd(b: &mut Bencher) {
        b.iter(|| euclid_gcd(test::black_box(10999), test::black_box(4786)))
    }
    // #[bench]
    // // 130,097,971 ns/iter (+/- 12,325,463)
    // fn bench_3918848_1653264_naive_gcd(b: &mut Bencher) {
    //     b.iter(|| naive_gcd(test::black_box(3_918_848), test::black_box(1_653_264)))
    // }
    #[bench]
    fn bench_3918848_1653264_euclid_gcd(b: &mut Bencher) {
        b.iter(|| euclid_gcd(test::black_box(3_918_848), test::black_box(1_653_264)))
    }
    #[bench]
    fn bench_3918848_1653264_e_gcd(b: &mut Bencher) {
        b.iter(|| e_gcd(test::black_box(3_918_848), test::black_box(1_653_264)))
    }
    #[bench]
    fn bench_13e34_33e13_euclid_gcd(b: &mut Bencher) {
        let anum: i128 = 13;
        let bnum: i128 = 33;
        b.iter(|| euclid_gcd(test::black_box(anum.pow(34)), test::black_box(bnum.pow(13))))
    }
    #[bench]
    fn bench_13e34_33e13_e_gcd(b: &mut Bencher) {
        let anum: i128 = 13;
        let bnum: i128 = 33;
        b.iter(|| e_gcd(test::black_box(anum.pow(34)), test::black_box(bnum.pow(13))))
    }
    */
}
