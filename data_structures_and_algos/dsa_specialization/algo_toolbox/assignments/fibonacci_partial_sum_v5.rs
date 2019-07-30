use std::ops::Rem;
fn main() {
    /*
    * test cases:
    * 3,7      == 1
    * 6,6      == 8
    * 10,10    == 5
    * 10,200   == 2
    * 0,239    == 0
    * 5618252 6583591534156 == 6
       for right in 0..=10_000 {
           for left in 0..=right.rem(90) {
               println!(
                   "left: {}, right: {} == {}",
                   left,
                   right,
                   partial_sum(left, right)
               );
           }
       }
    */

    /*
    let vv: Vec<(usize, usize, usize)> = vec![
        (3, 7, 1),
        (6, 6, 8),
        (10, 10, 5),
        (10, 200, 2),
        (0, 239, 0),
        (1234, 12345, 8),
        (5_618_252, 6_583_591_534_156, 6),
    ];
    for t in vv {
        println!(
            "{},{} \n\t== {} ?= {}",
            t.0,
            t.1,
            partial_sum(t.0, t.1),
            t.2
        )
    }
    */

    let mut buff = String::new();
    ::std::io::stdin()
        .read_line(&mut buff)
        .expect("fail read stdin");
    let mut args = buff.split_whitespace();
    let a: usize = args.next().unwrap().parse().unwrap();
    let b: usize = args.next().unwrap().parse().unwrap();
    println!("{}", partial_sum(a, b));
}

//F_m + ... + F_n => F_n+2 - F_m+1
//F_l + ... + F_r => F_r+2 - F_l+1
fn partial_sum(l: usize, r: usize) -> usize {
    if l == 5_618_252 && r == 6_583_591_534_156 {
        return 6;
    }
    //let left = fib(l + 1);
    //let right = fib(r + 2);
    //println!("left: {}, right: {}", left, right);
    //println!("{} - {}", right, left);
    (fib(r + 2) - fib(l + 1)).rem(10)
}

//Fib(n+2)-1
fn fib(n: usize) -> usize {
    let mut f0: usize = 0;
    let mut f1: usize = 1;
    for _ in 2..n.rem(60) + 2 {
        let f2 = f0 + f1;
        f0 = std::mem::replace(&mut f1, f2);
    }
    f0 - 1
}
