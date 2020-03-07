use std::ops::Rem;
fn main() {
    let res = fib(100);
    println!("{:.1}", res);
    println!("{}", res.rem(10.0));
}

/*
    Time Complexity: O(1)
    Space Complexity: O(1)
*/

fn fib(n: i32) -> f64 {
    let sqrt_five: f64 = 5.0_f64.sqrt();
    let phi: f64 = (1.0 + sqrt_five) / 2.0;
    (phi.powi(n) / sqrt_five).round()
}
