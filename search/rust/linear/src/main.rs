fn main() {
    /*
    let a = [1, 95, 78, 46, 58, 45, 86, 99, 251, 320];
    println!("expect true: {}", linear_search(a, 58));
    println!("expect false: {}", linear_search(a, 2005));

    let v: Vec<i32> = vec![1, 95, 78, 46, 58, 45, 86, 99, 251, 320];
    println!("vec expect true: {}", linear_search_vec(&v, 58));
    println!("vec expect false: {}", linear_search_vec(&v, 2005));

    let sterm: i32 = 86;
    let idx = linear_search_vec(&v, sterm);
    println!(
        "search term: {}; value at[{}] == {:?}",
        sterm, idx, v[idx as usize]
    );
    */

    let key_terms: &[usize] = &[13, 24, 34, 678, 890, 7487, 432];
    let vv: &[usize] = &[12, 13, 14, 78, 678, 432];
    for value in key_terms {
        match linear(vv, *value) {
            Ok(n) => println!("linres found value:{} at index:{}", value, n),
            Err(e) => println!("{}", e),
        }
    }
}

fn linear(list: &[usize], v: usize) -> Result<usize, String> {
    for (i, ival) in list.iter().enumerate() {
        if ival == &v {
            return Ok(i);
        }
    }
    Err("NIL".to_string())
}

#[allow(dead_code)]
fn linear_search(list: [i32; 10], key: i32) -> bool {
    for (_, v) in list.iter().enumerate() {
        if v == &key {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn linear_search_vec(list: &[i32], key: i32) -> isize {
    for (idx, v) in list.iter().enumerate() {
        if v == &key {
            return idx as isize;
        }
    }
    println!("NIL");
    -1
}