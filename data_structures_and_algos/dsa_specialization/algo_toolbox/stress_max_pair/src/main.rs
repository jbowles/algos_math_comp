extern crate rand;
use rand::Rng;

fn main() {
    let rng = rand::thread_rng();
    // println!("i32 {}", rng.gen::<i32>());
    // println!("i64 {}", rng.gen::<i64>());
    // println!("f64 {}", rng.gen::<f64>());

    loop {
        /*
        let n: u64 = rng.gen::<u64>() % 1000 + 2;
        let mut a: Vec<u64> = Vec::new();
        for _ in 0..n {
            a.push(rng.gen::<u64>() % 100_000)
        }
        for (idx, v) in a.iter().enumerate() {
            print!("a[{}]: {}", idx, v);
        }
        */
        let a = get_list(rng);
        let res1 = max_pairwise_slow(&a);
        let res2 = max_pairwise(&a);
        if res1 != res2 {
            println!("WRONG: {} {}", res1, res2);
            break;
        } else {
            println!("OK: {}", res2);
        }
    }
}

fn get_list(mut rng: rand::prelude::ThreadRng) -> Vec<u64> {
    let n: u64 = rng.gen::<u64>() % 1000 + 2;
    println!("N: {}", n);
    let mut a: Vec<u64> = Vec::new();
    for _ in 0..n {
        a.push(rng.gen::<u64>() % 100_000)
    }
    a
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
    println!("max1={}, max2={}", max1, max2);
    v[max1 as usize] * v[max2 as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct() {
        assert_eq!(6, max_pairwise(&[1, 2, 3]))
    }

}