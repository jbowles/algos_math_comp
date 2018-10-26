// O(n)
fn linear_search(list: [i32; 10], key: i32) -> bool {
    for (_, v) in list.iter().enumerate() {
        if v == &key {
            return true;
        }
    }
    false
}

fn linear_search_vec(list: &Vec<i32>, key: i32) -> isize {
    for (idx, v) in list.into_iter().enumerate() {
        if v == &key {
            return idx as isize;
        }
    }
    return -1;
}

fn main() {
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
}
