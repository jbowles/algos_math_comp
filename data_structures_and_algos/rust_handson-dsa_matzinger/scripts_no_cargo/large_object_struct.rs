fn main(){
    my_func();
}

struct LargeObject {
    number: u32
}

impl LargeObject {
    fn new(val: u32) -> Self {
        LargeObject{number: val}
    }
}

impl LargeObject {
    fn add(&mut self) {
        self.number += 1;
    }
}

fn my_func() {
    //compiler allocates memory for x
    let mut x = LargeObject::new(5);
    println!("{}", x.number);
    x.add();
    println!("{}", x.number)
}
