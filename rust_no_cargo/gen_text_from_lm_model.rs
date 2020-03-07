use std::collections::HashMap;
fn main(){
    //let mm = mock_model();
    //println!("{:?}", mm);
    println!("{}", rewind("~~~s", 4));
    println!("{}", rewind("~~~~", 4));
}


#[allow(dead_code)]
//fn rewind(token: &str, order: usize) -> String {
fn rewind(token: &str, order: usize) -> String {
    match token.chars().nth(order-1) {
        None => {return "".to_string()},
        Some(s) => {return s.to_string()},
    }
}

#[allow(dead_code)]
fn mock_model() -> HashMap<String, Vec<(char,f64)>> {
    //fake char ratio with commented counts that produced these scores
    let chr_ratio_one = vec![('c',0.33),('a',0.33),('n',0.33)]; //c:1, a:1, n:1
    let chr_ratio_two = vec![('a',0.5),('t',0.5)]; //a;1, t:1
    let chr_ratio_three = vec![('z', 1.0)]; //z:1
    let chr_ratio_four = vec![('b',0.25),('a',0.25),('l',0.50)]; //b:1, a:1, l:2
    let rats = vec![chr_ratio_one, chr_ratio_two, chr_ratio_three, chr_ratio_four];
    //let grams = vec![" I ".to_string(),"pot".to_string(),"fiz".to_string(),"lay".to_string()];
    let grams = vec![" I ","pot","fiz","lay"];
    /*
    println!("{:?}", rats);
    println!("{:?}", grams);
    for i in rats.iter() {
        println!("sum: {}", normalize(i));
    }
    */

    let mut mm: HashMap<String, Vec<(char,f64)>> = HashMap::new();

    for (idx,g) in grams.iter().enumerate() {
        mm.insert(g.clone().to_string(), rats[idx].clone());
        //println!("{:?}", mm);
    }
    mm
}

/*
#[allow(dead_code)]
fn normalize(accum: &[(char,f64)]) -> f64 {
    accum.iter().fold(0.0, |sum,a| sum + a.1)
}
*/
