fn main() {
    /*
    let ord = 4;
    let s = "~~~~0123456789";
    let hist = &s.as_bytes()[0..=(ord)];
    println!("{}", std::str::from_utf8(hist).unwrap());
    let a = vec![('c',2),('b',2),('d',4)];
    println!("{}", sum(&a));
    */

    let s = "This is my string";
    println!("rewind: {}", rewind(s,4));
    println!("las_char: {}", last_char(s,4));
    println!("las_char_indices: {}", last_char_indices(s,4));

    let sa = "This is a 🏈 in my string";
    let i: usize = 9;
    let order: usize = 13; //13 will error, 17 is ok
    /*
     * thread 'main' panicked at 'byte index 13 is not a char boundary; it is inside '🏈' (bytes 10..14) of `This is a 🏈 in my string`
        let res = interval_char_indices(sa,i,order);
        println!("interval_char_indices: {}", res);
    */

    let saferes = safe_interval_char_indices(sa,i,order);
    println!("interval_char_indices: {}", saferes);

    let cc = "c";
    println!("the &str: {}, chars(): {:?} => {:?}", cc, cc.chars(), cc.chars().next().unwrap());
}

#[allow(dead_code)]
fn safe_interval_char_indices(token: &str, begin: usize, end: usize) -> &str {
    let mut result: &str = " ";
    let mut end_char_boundary = end;
    while !token.is_char_boundary(end_char_boundary) {
        end_char_boundary+=1
    }
    if let Some((i, _)) = token.char_indices().nth(begin) {
        result = &token[i..end_char_boundary];
    };
    result
}

#[allow(dead_code)]
fn last_char_indices(token: &str, order: usize) -> &str {
    let mut result: &str = " ";
    if let Some((i, _)) = token.char_indices().rev().nth(order-1) {
        result = &token[i..]
    }
    result
}


#[allow(dead_code)]
fn interval_char_indices(token: &str, begin: usize, end: usize) -> &str {
    let mut result: &str = " ";
    if let Some((i, _)) = token.char_indices().nth(begin) {
        result = &token[i..end];
    };
    result
}

#[allow(dead_code)]
fn rewind(token: &str, order: usize) -> String {
    let res = token.chars().rev().take(order).collect::<String>().chars().rev().collect();
    res
}

#[allow(dead_code)]
//fn rewind(token: &str, order: usize) -> String {
fn last_char(token: &str, order: usize) -> char {
    match token.chars().rev().nth(order) {
        None => '0',
        Some(s) => s,
    }
}

#[allow(dead_code)]
fn sum(a: &Vec<(char,i32)>) -> i32 {
    a.iter().map(|t| t.1).sum()
}
