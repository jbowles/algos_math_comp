extern crate rand;
use rand::Rng;
#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;

/*
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}


fn max_pairwise_slow(v: &[u64]) -> u64 {
    let mut result: u64 = 0;
    for i in 0..v.len() {
        //println!("i: {}", i);
        for j in (i + 1)..v.len() {
            //println!("    j: {}", j);
            if v[i] * v[j] > result {
                result = v[i] * v[j];
            }
        }
    }
    result
}

fn get_list_randn() -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let n: u64 = rng.gen::<u64>() % 1000 + 2;
    println!("N: {}", n);
    let mut a: Vec<u64> = Vec::new();
    for _ in 0..n {
        a.push(rng.gen::<u64>() % 100_000)
    }
    a
}
*/

fn get_list() -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let n: u64 = 1001;
    let mut a: Vec<u64> = Vec::new();
    for _ in 0..n {
        a.push(rng.gen::<u64>() % 100_000)
    }
    a
}

fn max_pairwise(v: &[u64]) -> u64 {
    let mut max1: isize = -1;
    let mut max2: isize = -1;
    for i in 0..v.len() {
        if (max1 == -1) || v[i] > v[max1 as usize] {
            max1 = i as isize;
        }
    }
    for j in 0..v.len() {
        if (j != max1 as usize) && ((max2 == -1) || (v[j] > v[max2 as usize])) {
            max2 = j as isize;
        }
    }
    // println!("max1={}, max2={}", max1, max2);
    v[max1 as usize] * v[max2 as usize]
}

/*
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}
*/

fn bench_max_pairwise(c: &mut Criterion) {
    let l = get_list();
    c.bench_function("max pairwise", move |b| {
        // b.iter(|| max_pairwise(black_box(&l)))
        b.iter(|| max_pairwise(&l))
    });
}

/*
fn bench_max_pair_slow(c: &mut Criterion) {
    let l = get_list();
    c.bench_function("max pair slow", move |b| {
        b.iter(|| max_pairwise_slow(black_box(&l)))
    });
}
*/

criterion_group!(
    benches,
    // criterion_benchmark,
    bench_max_pairwise,
    // bench_max_pair_slow
);
criterion_main!(benches);
