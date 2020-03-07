fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("failed to read stdin");
    let mut args = buff.split_whitespace();

    let a: i64 = args.next().unwrap().parse().unwrap();
    let b: i64 = args.next().unwrap().parse().unwrap();
    println!("{}", a + b);
}
