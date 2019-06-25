fn main(){
    let v1 = vec![1,2,3];
    let v2 = vec![4,5,7];
    let result = dot(&v1, &v2);
    println!("RESUlT {}: {}", result.1, result.0);



    let v1 = vec![1,2,3];
    let v2 = vec![4,5];

    assert_eq!((0,"BAD!!".to_string()), dot(&v1, &v2));
}

fn dot(v1: &[u32],v2: &[u32]) -> (u32, String) {
    // Result<u32, Err>
    if v1.len() != v2.len() {
        return (0,"BAD!!".to_string())
    }
    (v1.iter().zip(v2).map(|(a,b)| a*b).sum(), "GOOD".to_string())
}
