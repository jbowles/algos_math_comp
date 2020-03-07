fn main() {
    let mut v: Vec<i32> = vec![];
    for i in 1..10 {
        v.push(i)
    };
    let dists: Vec<f64> = distances_among_pairs(&mut v);
    println!("{:?}", dists);
}


fn distances_among_pairs(v: &mut Vec<i32>) -> Vec<f64> {
    //println!("{:?}", v);
    let nin: usize = v.len();
    /*
     * length out output vector; two wasy to get it:
     * let nout = (nin * nin - nin) /2; 
     * let nnout = (usize::pow(nin,2)-nin) /2;
     *
     * println!("nout: {}", nout);
     * println!("nnout: {}", nnout);
    */
    let mut dists: Vec<f64> = Vec::with_capacity((usize::pow(nin,2)-nin) /2);
    let mut k:usize = 0;
    for i in 0..nin-1 {
        let a:f64 = f64::from(v[i]);
        for j in i+1..nin {
            let b:f64 = f64::from(v[j]);
            dists.insert(k,f64::abs(a-b));
            k+=1;
        }
    }

    dists
}
