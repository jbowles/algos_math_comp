// #![feature(test)]
// extern crate test;

fn main() {
    let should: Vec<u128> = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let sample: Vec<u128> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for (idx, n) in sample.iter().enumerate() {
        let res = fib_rec_one(*n);
        let res2 = fib_list(*n);
        println!("fib_rec_one({}) == {} for sample {}", n, res, idx);
        println!("fib_list({}) == {} for sample {}", n, res2, idx);
        assert_eq!(should[idx], res);
        assert_eq!(should[idx], res2);
    }
    let fib_tuple = (6765, 20);
    let res = fib_rec_one(fib_tuple.1);
    let res2 = fib_list(fib_tuple.1);
    println!("fib_rec_one: {}=={} Fib({})", res, fib_tuple.0, fib_tuple.1);
    println!("fib_list: {}=={}  Fib({})", res2, fib_tuple.0, fib_tuple.1);
    assert_eq!(fib_tuple.0, res);
    assert_eq!(fib_tuple.0, res2);

    println!("{}", fib_list(1768));
}

#[allow(dead_code)]
fn fib_rec_one(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    fib_rec_one(n - 1) + fib_rec_one(n - 2)
}

#[allow(dead_code)]
fn fib_list(n: u128) -> u128 {
    if n <= 1 {
        return n;
    }
    let mut v: Vec<u128> = (0..=n).collect();
    for i in 2..=n as usize {
        let a = (v[i - 1]) + (v[i - 2]);
        v[i] = a;
    }
    v[n as usize]
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    //cargo +nightly bench
    #[bench]
    fn bench_fibo_5_rec(b: &mut Bencher) {
        // use black_box to prevent compiler optimizations the end up with 0ns as becnch time
        b.iter(|| fib_rec_one(test::black_box(5)))
    }
    #[bench]
    fn bench_fibo_5_list(b: &mut Bencher) {
        // use black_box to prevent compiler optimizations the end up with 0ns as becnch time
        b.iter(|| fib_list(test::black_box(5)))
    }
    #[bench]
    fn bench_fibo_20_rec(b: &mut Bencher) {
        // use black_box to prevent compiler optimizations the end up with 0ns as becnch time
        b.iter(|| fib_rec_one(test::black_box(20)))
    }
    #[bench]
    fn bench_fibo_20_list(b: &mut Bencher) {
        // use black_box to prevent compiler optimizations the end up with 0ns as becnch time
        b.iter(|| fib_list(test::black_box(20)))
    }

}
*/
