fn main() {
    let mut a = vec![5, 2, 4, 6, 1, 3];
    println!("isort-nosort: {:?}", a);
    // let mut abar = vec![31, 41, 59, 26, 41, 58];
    isort(&mut a);
    println!("isort-sorted: {:?}", a);

    let mut aa = vec![5, 2, 4, 6, 1, 3];
    // let order = "asc";
    println!("isort_order-nosort: {:?}", aa);
    // let mut abar = vec![31, 41, 59, 26, 41, 58];
    isort_order(&mut aa, "desc");
    println!("isort_oreder-sorted: {:?}", aa);
}


/*
fn isort_apply<F>(a: &mut [isize], apply: F)
where
    F: Fn() -> std::cmp::PartialOrd,
{
    for j in 1..a.len() {
        let key = a[j];
        let mut i = (j - 1) as isize;
        while (i >= 0) && (a[i as usize] apply() key) {
            a[(i + 1) as usize] = a[i as usize];
            i -= 1;
        }
        a[(i + 1) as usize] = key;
    }
}
*/


fn isort(a: &mut [isize]) {
    for j in 1..a.len() {
        // println!("j={}", j);
        let key = a[j];
        let mut i = (j - 1) as isize;
        // println!("  i={}", i);
        while (i >= 0) && (a[i as usize] > key) {
            a[(i + 1) as usize] = a[i as usize];
            i -= 1;
        }
        a[(i + 1) as usize] = key;
    }
}

//REVERSE: switch comparison operator in while loop: (a[i] < key) instead of (a[i] > key)
fn isort_order(a: &mut [isize], order: &str) {
    for j in 1..a.len() {
        let key = a[j];
        let mut i = (j - 1) as isize;

        match order {
            "desc" => {
                while (i >= 0) && (a[i as usize] < key) {
                    a[(i + 1) as usize] = a[i as usize];
                    i -= 1;
                }
            }
            _ => {
                while (i >= 0) && (a[i as usize] > key) {
                    a[(i + 1) as usize] = a[i as usize];
                    i -= 1;
                }
            }
        }

        a[(i + 1) as usize] = key;
    }
}


#[allow(dead_code)]
fn insertion_sort_two(abar: &mut Vec<usize>) -> Vec<usize> {
    let mut a = abar.to_owned();
    for j in 1..a.len() {
        println!("j={}", j);
        for i in (1..=j).rev() {
            println!("  i={}", i);
            if a[i - 1] <= a[i] {
                break;
            }
            a.swap(i - 1, i);
        }
    }
    a
}


#[allow(dead_code)]
fn insertion_sort_three(a: &mut Vec<usize>) -> Vec<usize> {
    //0 indexed, begin with the second element
    for i in 1..a.len() {
        let mut j = i;
        while j > 0 {
            if a[j - 1] > a[j] {
                a.swap(j - 1, j);
            }
            j -= 1;
        }
    }
    a.to_vec()
}