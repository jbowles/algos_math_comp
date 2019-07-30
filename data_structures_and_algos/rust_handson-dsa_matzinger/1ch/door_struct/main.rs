fn main() {
    let mut d = Door::new(false);
    println!("{:?}", d);
    Openable::open(&mut d);
    println!("{:?}", d);
}

#[derive(Debug)]
struct Door {
    is_open: bool
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door {is_open: is_open}
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}
