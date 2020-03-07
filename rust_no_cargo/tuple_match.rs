fn main() {
    let v1: Vec<(char,i32)> =vec![('b', 1),('c', 1),('d', 1)];
    cmat(v1, 'z');
}

#[allow(dead_code)]
fn cmat(t: Vec<(char,i32)>, c: char) {
    let res = t.into_iter().map(|(one,_)| one).collect::<Vec<char>>();
    println!("{:?}", res);
    if !res.contains(&c) {
        println!("{:?} Does not contain '{}'", res, c);
    } else {
        println!("key '{}' found", c);
    }
}

#[allow(dead_code)]
fn vmat(t: Vec<(char,i32)>, c: char) {
    println!("{:?}, {}",t,c);
    for i in t.iter() {
        match i {
            (x,_) if x == &c => {println!("YES: {:?} {:?}", i, i.1)},
            _ => (),
        }
    }
}

#[allow(dead_code)]
fn smatch(t: (char,i32), input: char) {
    match t {
        (x,_) => { if x==input {println!("YES {:?}, {}, {}", t, x,input)} }
    }
}


#[allow(dead_code)]
fn tma(t: (char,i32), val: char) {
    if val == t.0 {
        println!("tma: YES {:?} == {}", t, val)
    } else {
        print!("tma: NO")
    }
}
