use std::ops::Rem;
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: usize = args.next().unwrap().parse().unwrap();
    let b: usize = args.next().unwrap().parse().unwrap();
    println!("{}", partial_sum(a, b));
}

fn partial_sum(l: usize, r: usize) -> f64 {
    let res2 = fib(l);
    let res1 = fib(r);
    println!("{:.1}, {:.1}", res1, res2);
    (res1 - res2).rem(10.0)
}

fn fib(n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if n == 1 {
        return 1.0;
    }
    for i in fibonacci().skip(n - 2).take(1) {
        return i.round();
    }
    0.0
}

/**
 * Iterative fibonacci
 * https://github.com/rust-lang/rust-by-example
 */
pub struct Fibonacci {
    curr: f64,
    next: f64,
}

impl Iterator for Fibonacci {
    type Item = f64;
    fn next(&mut self) -> Option<f64> {
        let new_next = (self.curr + self.next).round();

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}

/**
 * A "constructor" for Iterative fibonacci
 */
pub fn fibonacci() -> Fibonacci {
    Fibonacci {
        curr: 1.0,
        next: 1.0,
    }
}
