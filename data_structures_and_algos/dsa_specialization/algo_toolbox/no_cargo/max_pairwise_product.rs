#[allow(unused_assignments)]
#[allow(unused_variables)]
fn main() {
    use std::io::Write;
    let stdin = std::io::stdin();
    let input = &mut String::new();
    let mut n: i32 = 0;
    let mut v: Vec<u64> = Vec::new();

    for i in 0..2 {
        input.clear();
        let _ = std::io::stdout().flush();
        if i == 0 {
            stdin.read_line(input).unwrap();
            n = input.trim().parse().unwrap();
        //continue;
        } else {
            stdin.read_line(input).unwrap();
            v = input
                .trim()
                .split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
        }
    }
    println!("{:?}", max_pairwise(&v))
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
    //println!("max1={}, max2={}", max1, max2);
    v[max1 as usize] * v[max2 as usize]
}

/*
// O(n^2) ...
fn max_pairwise(v: &[i64]) -> i64 {
    let mut result: i64 = 0;
    for i in 0..v.len() {
        for j in (i + 1)..v.len() {
            if v[i] * v[j] > result {
                result = v[i] * v[j];
            }
        }
    }
    result
}
*/
