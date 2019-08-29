#![feature(test)]
use rand::Rng;

extern crate test;

// cargo +nightly run
//cargo +nightly run --release
fn main() {
    let mut rng = rand::thread_rng();
    let mut v: Vec<f64> = vec![];
    for _ in 1..10_000 {
        v.push(rng.gen::<f64>())
    }
    let dists: Vec<f64> = not_idiomatic(&v);
    println!("length: {}", dists.len());
    // println!("first100: {:?}", dists.len());
}

fn not_idiomatic(v: &Vec<f64>) -> Vec<f64> {
    //println!("{:?}", v);
    let nin: usize = v.len();
    /*
     * length out output vector; two wasy to get it:
     * let nout = (nin * nin - nin) /2;-
     * let nnout = (usize::pow(nin,2)-nin) /2;
     *
     * println!("nout: {}", nout);
     * println!("nnout: {}", nnout);
     */
    //preallocating output vector
    let mut dists: Vec<f64> = Vec::with_capacity((usize::pow(nin, 2) - nin) / 2);
    let mut k: usize = 0; //position of output vector
    for i in 0..nin - 1 {
        let a: f64 = v[i];
        for j in i + 1..nin {
            let b: f64 = v[j];
            dists.insert(k, f64::abs(a - b));
            k += 1;
        }
    }

    dists
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    //bench: 606,177,099 ns/iter (+/- 123,017,984)
    // 0.6 second
    #[bench]
    fn bench_distance_pairs(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut v: Vec<f64> = vec![];
        for _ in 1..10_000 {
            v.push(rng.gen::<f64>())
        };
        b.iter(|| {
            not_idiomatic(&v);
        });
    }
}
