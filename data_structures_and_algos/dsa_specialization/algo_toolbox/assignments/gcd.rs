use std::ops::Rem;
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: u128 = args.next().unwrap().parse().unwrap();
    let b: u128 = args.next().unwrap().parse().unwrap();

    println!("{}", gcd(a, b));
}

fn gcd(a: u128, b: u128) -> u128 {
    if b == 0 {
        return a;
    }
    let a_bar: u128 = a.rem(b);
    gcd(b, a_bar)
}
