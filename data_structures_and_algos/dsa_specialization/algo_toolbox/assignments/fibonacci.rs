//rustc -o fibrs fibonacci.rs
//// CORRECT
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("failed to read stdin");
    let mut args = buff.split_whitespace();

    let a: u32 = args.next().unwrap().parse().unwrap();
    println!("{}", fib(a));
}

fn fib(n: u32) -> u128 {
    //we know 0,1
    if n <= 1 {
        return u128::from(n);
    }
    let mut v: Vec<u128> = (0..=u128::from(n)).collect();
    for i in 2..=n as usize {
        v[i] = (v[i - 1]) + (v[i - 2]);
    }
    v[n as usize]
}
