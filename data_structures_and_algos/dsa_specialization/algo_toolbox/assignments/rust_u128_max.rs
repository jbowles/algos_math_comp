fn main() {
    assert_eq!(u128::max_value(), 340282366920938463463374607431768211455);
    println!("{}", u128::max_value());

    let f: u128 = 1281597540372340914251;
    let g: u128 = 573147844013817084101;
    println!("f: {} g: {}", f, g);
    println!("f+g: {}", f + g);
    //println!("f*g: {}", f * g); //'attempt to multiply with overflow'
    //println!("f*g + f: {}", f.saturating_mul(g) + f); //'attempt to multiply with overflow'
    println!("f*g: {}", f.saturating_mul(g)); //340282366920938463463374607431768211455
    println!("f*g + f: {}", f.saturating_mul(g).saturating_add(f)); //'attempt to multiply with overflow'
}
