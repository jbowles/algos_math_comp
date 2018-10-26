fn find_first_dup_item(v: &mut Vec<i32>) {
    print!("repeated items: ");
    for i in 0..(v.len() - 1) as usize {
        if v[i32::abs(v[i]) as usize] >= 0 {
            let idx = i32::abs(v[i]) as usize;
            v[idx] = -v[i32::abs(v[i]) as usize];
        } else {
            print!("{}  ", i32::abs(v[i]));
        }
    }
    println!();
}

fn main() {
    let mut items: Vec<i32> = vec![4, 2, 4, 5, 2, 3, 1, 5, 6];
    find_first_dup_item(&mut items);
}
