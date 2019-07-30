use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    println!("Hello, world!");
}

// type CellNode = Rc<RefCell<Node>>;
// type SingleLink = Option<CellNode>;
type SingleLink = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: SingleLink,
}
#[derive(Clone)]
struct TransactionLog {
    head: SingleLink,
    tail: SingleLink,
    pub length: u64,
}

impl Node {
    // fn new(value: String) -> CellNode {
    fn new(value: String) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}
/*
    NEED:
        append entreid at end
        remove entreis from front
*/
impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
}
