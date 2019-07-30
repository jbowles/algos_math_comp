use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    let mut n: i32 = 0;
    let mut v: Vec<i32> = Vec::new();

    for i in 0..2 {
        input.clear();
        let _ = io::stdout().flush();
        if i == 0 {
            stdin.read_line(input).unwrap();
            n = input.trim().parse().unwrap();
        } else {
            stdin.read_line(input).unwrap();
            v = input
                .trim()
                .split_whitespace()
                .map(|w| w.parse().unwrap())
                .collect();
        }
    }

    println!("N: {:?}", n);
    println!("VEC: {:?}", v);
}
