// O(Log n)
fn binary_search(needle: u32, haystack: &Vec<u32>) -> isize {
    //set low to zero
    let mut low = 0;
    //set high to vec.length()-1
    let mut high = haystack.len() - 1;

    // while low is less than or equal to high
    while low <= high {
        //define median index
        let median = (low + high) / 2;
        // compare haystack index with needle
        if haystack[median] < needle {
            //if less uptick low
            low = median + 1;
        } else {
            //else downtick high
            high = median - 1
        }
    }
    if low == haystack.len() || haystack[low] != needle {
        return -1 as isize;
    }
    return low as isize;
}

fn main() {
    let sterm: u32 = 63;
    let items: Vec<u32> = vec![1, 2, 9, 20, 31, 45, 63, 70, 100, 34];
    let idx = binary_search(sterm, &items);
    if idx > -1 {
        println!(
            "sterm: {}; vlaue at[{}] == {:?}",
            sterm, idx, items[idx as usize]
        );
    } else {
        println!("search term '{}' not found", sterm);
    }
}
