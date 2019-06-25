fn main() {
    let mut abar = vec![5, 2, 4, 6, 1, 3];
    // let mut abar = vec![31, 41, 59, 26, 41, 58];
    println!("TWOnosort: {:?}", abar);
    let a = insertion_sort_two(&mut abar);
    println!("TWOsorted: {:?}", a);

    println!("---");


    let mut a = vec![5, 2, 4, 6, 1, 3];
    println!("isort-sorted: {:?}", a);
    // let mut abar = vec![31, 41, 59, 26, 41, 58];
    isort(&mut a);
    println!("isort-nosort: {:?}", a);

    println!("---");

    /*
        // let mut abar = vec![5, 2, 4, 6, 1, 3];
        let mut abar = vec![31, 41, 59, 26, 41, 58];
        let a = insertion_sort_three(&mut abar);
        println!("THREEsorted: {:?}", a);
        println!("THREEnosort: {:?}", abar);
    */
}

fn isort(a: &mut [isize]) {
    for j in 1..a.len() {
        println!("j={}", j);
        let key = a[j];
        let mut i = (j - 1) as isize;
        println!("  i={}", i);
        while (i >= 0) && (a[i as usize] > key) {
            a[(i + 1) as usize] = a[i as usize];
            i -= 1;
        }
        a[(i + 1) as usize] = key;
    }
}

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