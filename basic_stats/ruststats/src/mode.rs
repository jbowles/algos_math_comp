use core::hash::BuildHasher;
use std::collections::HashMap;

// appropriate use of generalization over a HashMap or HashSet...
// not really needed, can overide with clippy ignore implicit_hasher
// but want to show example of proper way..
// https://rust-lang.github.io/rust-clippy/master/index.html#implicit_hasher
pub fn find_mode<S: BuildHasher>(hist: HashMap<u32, u32, S>) -> u32 {
    let (mut res, mut tmp) = (0, 0);
    // println!("{:?}", hist);
    for (k, v) in hist.iter() {
        if v > &tmp {
            res = *k;
            tmp = *v;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::{find_mode, HashMap};
    #[test]
    fn find_mode_simple() {
        let p: HashMap<u32, u32> = [(1, 8), (2, 4), (6, 1), (8, 3), (9, 4), (19, 44)]
            .iter()
            .cloned()
            .collect();
        assert_eq!(find_mode(p), 19);
    }
}
