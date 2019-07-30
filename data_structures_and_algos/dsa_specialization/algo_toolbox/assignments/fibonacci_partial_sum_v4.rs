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

fn partial_sum(l: usize, r: usize) -> usize {
    //(fib(r + 2) - fib(l + 1)) //.rem(10)
    let lres = fib(l + 1);
    let rres = fib(r + 2);
    println!("r: {}, l: {}", rres, lres);
    println!("r-l: {}", rres - lres);
    (rres - lres).rem(10)
}

fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    let r = n; //n.rem(60);
    let mut v: Vec<usize> = vec![0; (r + 2) as usize];
    v[0] = 0;
    v[1] = 1;
    for i in 2..(r) {
        println!("i-1 + i-2: {}", (v[i - 1] + v[i - 2]));
        v[i] = (v[i - 1] + v[i - 2]).rem(10);
    }
    let mut sum: usize = v[r + 2];
    if sum == 0 {
        return sum;
    }
    sum -= 1;
    sum //.rem(10)
}
