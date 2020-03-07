fn main() {
    let input = read_val::<u64>().unwrap();
    println!("INITIAL VECTOR{:?}", &input);
    //let size = input[0].clone();
    //let set = &input[1..];
    //println!("{}, {:?}", size, set);
    println!("{}", max_pairwise(&input));

    //println!("RESULT: {}", max_pairwise(&vec![0; 100000]));
}

#[allow(dead_code)]
fn read_val<T: std::str::FromStr>() -> Result<Vec<T>, T::Err> {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("stdin error read_line");
    s.trim().split_whitespace().map(|w| w.parse()).collect()
}

/*
//O(n^2)
#[allow(dead_code)]
fn max_pairwise_slow(v: &[i64]) -> i64 {
    let mut result: i64 = 0;
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
*/
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
