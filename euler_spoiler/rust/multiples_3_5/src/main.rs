/*
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below 1000.
*/
fn mult_three_five(count: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..count {
        if i % 3 == 0 {
            sum += i;
            continue;
        }
        if i % 5 == 0 {
            sum += i;
            continue;
        }
    }
    return sum;
}
fn mult_three_five_v2(count: u64) -> u64 {
    let mut sum: u64 = 0;
    for i in 1..count {
        match (i % 3, i % 5) {
            (0, _) => sum += i,
            (_, 0) => sum += i,
            (_, _) => (),
        }
    }
    return sum;
}

fn main() {
    //let answer = mult_three_five(10);
    //println!("expect 23: {}", answer);
    let answer = mult_three_five(1000);
    println!("expect 233168: {}", answer);
    let answer = mult_three_five_v2(1000);
    println!("expect 233168: {}", answer);
}
