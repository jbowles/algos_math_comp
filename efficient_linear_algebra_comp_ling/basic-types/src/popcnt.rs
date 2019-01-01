pub fn popcnt1(val: u64) -> u32 {
    //val.count_ones() //built-in!
    let mut c = 0;
    let mut n = val;
    while n > 0 {
        c += n & 1;
        n >>= 1;
    }
    c as u32
}

#[inline(always)]
pub fn popcnt(mut x: u64) -> u32 {
    const M1_64: u64 = 0x5555555555555555; //binary: 0101...
    const M2_64: u64 = 0x3333333333333333; //binary: 00110011..
    const M4_64: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
    const H01_64: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...
    x -= (x >> 1) & M1_64; //count of each two bits into those 2 bits
    x = (x & M2_64) + ((x >> 2) & M2_64); //put count of each 4 bits
    (((x + (x >> 4)) & M4_64).wrapping_mul(H01_64) >> 24) as u32
}

const POPCNT_TABLE: [u8; 256] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];

#[allow(dead_code)]
fn compute_byte_table() -> [u8; 256] {
    let mut table = [0u8; 256];

    for i in 0..(table.len()) {
        table[i] = table[i / 2] + (i & 1) as u8;
    }

    table
}

pub fn popcnt_lookup(val: u64) -> u32 {
    (POPCNT_TABLE[(val & 0xff) as usize]
        + POPCNT_TABLE[(val >> 8) as usize & 0xff]
        + POPCNT_TABLE[(val >> 16) as usize & 0xff]
        + POPCNT_TABLE[(val >> 24) as usize & 0xff]
        + POPCNT_TABLE[(val >> 32) as usize & 0xff]
        + POPCNT_TABLE[(val >> 40) as usize & 0xff]
        + POPCNT_TABLE[(val >> 48) as usize & 0xff]
        + POPCNT_TABLE[(val >> 56) as usize & 0xff]) as u32
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use std::usize;

    use quickcheck::{QuickCheck, StdGen};

    use super::{popcnt, popcnt_lookup};

    #[test]
    fn popcnt_test() {
        fn prop(v: u64) -> bool {
            if popcnt(v) != v.count_ones() {
                println!("popcnt: {}, ones: {}", popcnt(v), v.count_ones());
                return false;
            }

            // Cheap way to check whether popcnt is correct on 32-bit
            // machines, since quickcheck only allows us to generate
            // values up to usize::MAX.
            let v_shift = v << 32;
            popcnt(v_shift) == v_shift.count_ones()
        }

        QuickCheck::new()
            .gen(StdGen::new(rand::thread_rng(), usize::MAX))
            .quickcheck(prop as fn(u64) -> bool)
    }

    #[test]
    fn popcnt_lookup_test() {
        fn prop(v: u64) -> bool {
            if popcnt_lookup(v) != v.count_ones() {
                return false;
            }

            let v_shift = v << 32;
            popcnt_lookup(v_shift) == v_shift.count_ones()
        }

        QuickCheck::new()
            .gen(StdGen::new(rand::thread_rng(), usize::MAX))
            .quickcheck(prop as fn(u64) -> bool)
    }
}
