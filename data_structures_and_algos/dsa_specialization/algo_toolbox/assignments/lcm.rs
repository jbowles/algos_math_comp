use std::ops::Rem;
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: u128 = args.next().unwrap().parse().unwrap();
    let b: u128 = args.next().unwrap().parse().unwrap();
    println!("{}", lcm(a, b));
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    let a_bar: u128 = a.rem(b);
    gcd(b, a_bar)
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

/*
fn lcm_naive(a: u128, b: u128) -> u128 {
    let product = a * b;
    for i in 1..=product {
        if (i.rem(a) == 0) && (i.rem(b) == 0) {
            return i;
        }
    }
    product
}
*/
