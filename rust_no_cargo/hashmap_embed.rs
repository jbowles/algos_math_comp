use std::collections::HashMap;
use std::collections::hash_map::Entry;
fn main() {
    let mut lm: Tlm = HashMap::new();
    /*
    two_d_insert(&mut lm, "this".to_string(), ('c',3));
    println!("{:?}", lm);
    two_d_insert(&mut lm, "this".to_string(), ('d',1));
    println!("{:?}", lm);
    two_d_insert(&mut lm, "this".to_string(), ('c',3));
    println!("{:?}", lm);
    */


    two_d_ins(&mut lm, "this".to_string(), 'c');
    println!("{:?}", lm);
    two_d_ins(&mut lm, "this".to_string(), 'c');
    println!("{:?}", lm);
    two_d_ins(&mut lm, "this".to_string(), 'c');
    println!("{:?}", lm);

    lookup(lm);
}

type CharI = (char,i32);
type Counter = Vec<CharI>;
type Tlm = HashMap<String, Counter>;

#[allow(dead_code)]
fn lookup(t: Tlm) {
    println!("lookup: {:?}", t[&"this".to_string()])
}

#[allow(dead_code)]
fn two_d_ins(t: &mut Tlm, key: String, current_char: char) {
    match t.entry(key) {
        Entry::Vacant(e) => {e.insert(vec![(current_char,1)]);},
        Entry::Occupied(mut e) => {
            for i in e.into_mut().iter_mut() {
                if i.0 == current_char {
                    i.1+=1
                }
            }
        },
    }
}

/*
match e.get_mut() {
    (c,i) => { if c == value {i+=1}},
}
*/

#[allow(dead_code)]
fn two_d_insert(t: &mut Tlm, key: String, value: CharI) {
    match t.entry(key) {
        Entry::Vacant(e) => {e.insert(vec![value]);},
        Entry::Occupied(mut e) => {e.get_mut().push(value);},
    }
}







