const M1_64: u64 = 0x5555555555555555; //binary: 0101...
const M2_64: u64 = 0x3333333333333333; //binary: 00110011..
const M4_64: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
const M8_64: u64 = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
const H01_64: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...

/*
References:
    * [HW efficient](https://en.wikipedia.org/wiki/Hamming_weight#Efficient_implementation)
    * [primesieve](https://github.com/kimwalisch/primesieve)
    * [HW rust](https://github.com/huonw/hamming)
    * [tree-merging](http://web.archive.org/web/20120411185540/http://perso.citi.insa-lyon.fr/claurado/hamming.html)

*/
pub trait HammingWeight {
    fn popcount(self) -> u64;
    fn native(self) -> u64;
}
impl HammingWeight for u64 {
    fn native(self) -> u64 {
        self.count_ones() as u64
    }
    fn popcount(self) -> u64 {
        let mut x = self;
        x -= (x >> 1) & M1_64;
        x = (x & M2_64) + ((x >> 2) & M2_64);
        ((x + (x >> 4)) & M4_64).wrapping_mul(H01_64) >> 24
    }
}
impl HammingWeight for u32 {
    fn native(self) -> u64 {
        self.count_ones() as u64
    }
    fn popcount(self) -> u64 {
        let mut x = self as u64;
        x -= (x >> 1) & M1_64;
        x = (x & M2_64) + ((x >> 2) & M2_64);
        ((x + (x >> 4)) & M4_64).wrapping_mul(H01_64) >> 24
    }
}
impl HammingWeight for &[u8] {
    fn native(self) -> u64 {
        self.iter().fold(0, |a, b| a + b.count_ones() as u64)
    }
    /*
    borrowed heavily from here:
    https://github.com/huonw/hamming/blob/master/src/weight_.rs#L39
    */
    fn popcount(self) -> u64 {
        //tuple for head,mid,tail to vectorize
        let (head, buf, tail) = (&self[..1], [[0 as u64; 30]], &self[1..]);
        let mut count = internal_weight(head) + internal_weight(tail);

        for b in buf.iter() {
            let mut accum = 0;
            for _j in 0..10 {
                let j = _j * 3;
                let mut c1 = b[j];
                let mut c2 = b[j + 1];
                let mut half1 = b[j + 2];
                let mut half2 = half1;
                half1 &= M1_64;
                half2 = (half2 >> 1) & M1_64;
                c1 -= (c1 >> 1) & M1_64;
                c2 -= (c2 >> 1) & M1_64;
                c1 += half1;
                c2 += half2;
                c1 = (c1 & M2_64) + ((c1 >> 2) & M2_64);
                c1 += (c2 & M2_64) + ((c2 >> 2) & M2_64);
                accum += (c1 & M4_64) + ((c1 >> 4) & M4_64);
            }
            accum = (accum & M8_64) + ((accum >> 8) & M8_64);
            accum = accum + (accum >> 16);
            accum = accum + (accum >> 32);
            count += accum & 0xFFFF;
        }
        count
    }
}
fn internal_weight(x: &[u8]) -> u64 {
    x.iter().fold(0, |a, b| a + b.count_ones() as u64)
}
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
pub enum DistanceError {
    Size,
}
impl std::fmt::Display for DistanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DistanceError::Size => write!(f, "ERROR: byte arrays must be the same size"),
        }
    }
}

/* Computes the bitwise [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance) between
 `x` and `y`. Number of bits where `x` and `y` differ, or, the number of set bits in the xor of `x` and `y`.
*/
fn distance_naive(x: &[u8], y: &[u8]) -> Result<u64, DistanceError> {
    if x.len() != y.len() {
        return Result::Err(DistanceError::Size);
    }
    let d = x
        .iter()
        .zip(y)
        .fold(0, |a, (b, c)| a + (*b ^ *c).count_ones() as u64);
    Ok(d)
}
fn distance_vectorize(x: &[u8], y: &[u8]) -> Result<u64, DistanceError> {
    if x.len() != y.len() {
        return Result::Err(DistanceError::Size);
    }
    let (head1, buffer1, tail1) = (&x[..1], [[0 as u64; 30]], &x[1..]);
    let (head2, buffer2, tail2) = (&y[..1], [[0 as u64; 30]], &y[1..]);

    let c_head = match distance_naive(head1, head2) {
        Ok(v) => v,
        Err(err) => return Result::Err(err),
    };
    let c_tail = match distance_naive(tail1, tail2) {
        Ok(v) => v,
        Err(err) => return Result::Err(err),
    };
    let mut count = c_head + c_tail;
    for (buf1, buf2) in buffer1.iter().zip(&buffer2) {
        let mut accum = 0;
        for _j in 0..10 {
            let j = _j * 3;
            let mut c1 = buf1[j] ^ buf2[j];
            let mut c2 = buf1[j + 1] ^ buf2[j + 1];
            let mut half1 = buf1[j + 2] ^ buf1[j + 2];
            let mut half2 = half1;
            half1 &= M1_64;
            half2 = (half2 >> 1) & M1_64;
            c1 -= (c1 >> 1) & M1_64;
            c2 -= (c2 >> 1) & M1_64;
            c1 += half1;
            c2 += half2;
            c1 = (c1 & M2_64) + ((c1 >> 2) & M2_64);
            c1 += (c2 & M2_64) + ((c2 >> 2) & M2_64);
            accum += (c1 & M4_64) + ((c1 >> 4) & M4_64);
        }
        accum = (accum & M8_64) + ((accum >> 8) & M8_64);
        accum = accum + (accum >> 16);
        accum = accum + (accum >> 32);
        count += accum & 0xFFFF;
    }
    Ok(count)
}

pub fn distance(x: &[u8], y: &[u8]) -> Result<u64, DistanceError> {
    let d = match distance_vectorize(x, y) {
        Ok(v) => v,
        Err(ref error) if error == &DistanceError::Size => match distance_naive(x, y) {
            Ok(v) => v,
            Err(e) => return Result::Err(e),
        },
        Err(e) => return Result::Err(e),
    };
    Ok(d)
}

/*
u32 binary representations
const M1_32: u32 = 0x55555555; //binary: 0101...
const M2_32: u32 = 0x33333333; //binary: 00110011..
const M4_32: u32 = 0x0f0f0f0f; //binary:  4 zeros,  4 ones ...
const M8_32: u32 = 0x00ff00ff00; //binary:  8 zeros,  8 ones ...
const M16_32: u32 = 0x0000ffff0000; //binary: 16 zeros, 16 ones ...
const M32_32: u32 = 0x00000000ffffffff; //binary: 32 zeros, 32 ones
const H01_32: u32 = 0x01010101; //0x1010101; sum of 256 to the power of 0,1,2,3...

u64 binary representations
const M1_64: u64 = 0x5555555555555555; //binary: 0101...
const M2_64: u64 = 0x3333333333333333; //binary: 00110011..
const M4_64: u64 = 0x0f0f0f0f0f0f0f0f; //binary:  4 zeros,  4 ones ...
const M8_64: u64 = 0x00ff00ff00ff00ff; //binary:  8 zeros,  8 ones ...
const M16_64: u64 = 0x0000ffff0000ffff; //binary: 16 zeros, 16 ones ...
const M32_64: u64 = 0x00000000ffffffff; //binary: 32 zeros, 32 ones
const H01_64: u64 = 0x0101010101010101; //the sum of 256 to the power of 0,1,2,3...
*/
